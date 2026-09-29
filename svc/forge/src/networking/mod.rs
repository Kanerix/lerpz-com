use std::collections::BTreeMap;

use k8s_openapi::{
    api::{
        apps::v1::Deployment,
        core::v1::{Service, ServicePort, ServiceSpec},
    },
    apimachinery::pkg::{
        apis::meta::v1::{ObjectMeta, OwnerReference},
        util::intstr::IntOrString,
    },
};
use kube::{
    Api,
    api::PostParams,
    core::{ApiResource, DynamicObject, GroupVersionKind},
};
use lerpz_axum::problem::{HandlerResult, Problem};
use serde_json::json;

use crate::{config::CONFIG, resources, state::KubeClient};

pub(crate) const RUNTIME_ID_LABEL: &str = "lerpz.com/runtime-id";
pub(crate) const BASE_URL_ANNOTATION: &str = "lerpz.com/runtime-base-url";

pub(crate) fn base_url(runtime_id: &str) -> String {
    format!(
        "https://{}/{runtime_id}",
        CONFIG
            .AGENT_RUNTIME_ORIGIN
            .authority()
            .expect("runtime origin has an authority"),
    )
}

/// Creates the private HTTP route and its dependencies, owned by the deployment.
pub(crate) async fn provision(
    kube: KubeClient,
    deployment: &Deployment,
    runtime_id: &str,
) -> HandlerResult<()> {
    let uid = deployment.metadata.uid.clone().ok_or_else(|| {
        Problem::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Missing runtime metadata",
            "The cluster did not provide the deployment UID required for networking cleanup.",
        )
    })?;
    let deployment_name = deployment.metadata.name.clone().ok_or_else(|| {
        Problem::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Missing runtime metadata",
            "The cluster did not provide the deployment name required for networking cleanup.",
        )
    })?;
    let name = format!("runtime-{runtime_id}");
    let metadata = ObjectMeta {
        name: Some(name.clone()),
        namespace: Some(CONFIG.KUBE_NAMESPACE.to_string()),
        labels: deployment.metadata.labels.clone(),
        owner_references: Some(vec![OwnerReference {
            api_version: "apps/v1".to_owned(),
            kind: "Deployment".to_owned(),
            name: deployment_name,
            uid,
            controller: Some(true),
            block_owner_deletion: Some(false),
        }]),
        ..Default::default()
    };

    let service = Service {
        metadata: metadata.clone(),
        spec: Some(ServiceSpec {
            type_: Some("ClusterIP".to_owned()),
            selector: Some(BTreeMap::from([
                (
                    resources::MANAGED_BY_LABEL.to_owned(),
                    resources::MANAGED_BY.to_owned(),
                ),
                (RUNTIME_ID_LABEL.to_owned(), runtime_id.to_owned()),
            ])),
            ports: Some(vec![ServicePort {
                name: Some("http".to_owned()),
                port: 80,
                target_port: Some(IntOrString::String("http".to_owned())),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    };
    Api::<Service>::namespaced(kube.clone(), &CONFIG.KUBE_NAMESPACE)
        .create(&PostParams::default(), &service)
        .await
        .map_err(|err| resources::kube_problem(err, "runtime service"))?;

    let middleware_resource = traefik_resource("Middleware", "middlewares");
    let middleware_api = Api::<DynamicObject>::namespaced_with(
        kube.clone(),
        &CONFIG.KUBE_NAMESPACE,
        &middleware_resource,
    );
    let prefix = format!("/{runtime_id}");
    let auth_url = format!(
        "http://forge.{}.svc:5000/api/v1/runtimes/{runtime_id}/authorize",
        CONFIG.KUBE_NAMESPACE,
    );
    let allowed_origin = CONFIG.ALLOWED_ORIGINS.to_str().map_err(|err| {
        Problem::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid origin configuration",
            "The configured frontend origin cannot be used for runtime CORS.",
        )
        .with_error(err)
    })?;
    let middlewares = [
        (
            "cors",
            json!({
                "headers": {
                    "accessControlAllowOriginList": [allowed_origin],
                    "accessControlAllowMethods": ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"],
                    "accessControlAllowHeaders": ["Authorization", "Content-Type", "Accept"],
                    "accessControlMaxAge": 600,
                    "addVaryHeader": true,
                },
            }),
        ),
        (
            "auth",
            json!({
                "forwardAuth": {
                    "address": auth_url,
                    "trustForwardHeader": false,
                    "authRequestHeaders": ["Authorization"],
                },
            }),
        ),
        (
            "prefix",
            json!({
                "stripPrefix": {"prefixes": [prefix]},
            }),
        ),
        (
            "credentials",
            json!({
                "headers": {
                    "customRequestHeaders": {
                        "Authorization": "",
                        "Cookie": "",
                        "Proxy-Authorization": "",
                    },
                    "customResponseHeaders": {
                        "Set-Cookie": "",
                        "Cache-Control": "no-store",
                    },
                },
            }),
        ),
    ];
    let mut middleware_refs = Vec::new();
    for (suffix, spec) in middlewares {
        let middleware_name = format!("{name}-{suffix}");
        let mut middleware =
            DynamicObject::new(&middleware_name, &middleware_resource).data(json!({"spec": spec}));
        middleware.metadata = ObjectMeta {
            name: Some(middleware_name.clone()),
            ..metadata.clone()
        };
        middleware_api
            .create(&PostParams::default(), &middleware)
            .await
            .map_err(|err| resources::kube_problem(err, "runtime middleware"))?;
        middleware_refs.push(json!({"name": middleware_name}));
    }

    let host = CONFIG
        .AGENT_RUNTIME_ORIGIN
        .host()
        .expect("runtime origin has a host");
    let route_resource = traefik_resource("IngressRoute", "ingressroutes");
    let mut route = DynamicObject::new(&name, &route_resource).data(json!({
        "spec": {
            "entryPoints": ["websecure"],
            "routes": [{
                "kind": "Rule",
                "match": format!("Host(`{host}`) && (Path(`{prefix}`) || PathPrefix(`{prefix}/`))"),
                "middlewares": middleware_refs,
                "services": [{"name": name, "port": 80}],
            }],
            "tls": {"secretName": CONFIG.AGENT_RUNTIME_TLS_SECRET.as_ref()},
        },
    }));
    route.metadata = metadata;
    Api::<DynamicObject>::namespaced_with(kube, &CONFIG.KUBE_NAMESPACE, &route_resource)
        .create(&PostParams::default(), &route)
        .await
        .map_err(|err| resources::kube_problem(err, "runtime ingress route"))?;

    Ok(())
}

fn traefik_resource(kind: &str, plural: &str) -> ApiResource {
    ApiResource {
        plural: plural.to_owned(),
        ..ApiResource::from_gvk(&GroupVersionKind::gvk("traefik.io", "v1alpha1", kind))
    }
}
