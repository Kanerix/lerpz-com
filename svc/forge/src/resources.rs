//! Shared conventions for the Kubernetes objects Forge manages.
//!
//! Resource access requires Forge's management label, user ownership and owner
//! IDs matching the caller's Entra object and tenant IDs. Named access also
//! checks the agent label. Creator labels record attribution, not permissions.

use std::collections::BTreeMap;

use axum::http::StatusCode;
use k8s_openapi::{
    api::{apps::v1::Deployment, core::v1::PersistentVolumeClaim},
    apimachinery::pkg::apis::meta::v1::{ObjectMeta, Time},
};
use kube::{
    Api,
    api::{DeleteParams, Preconditions},
};
use lerpz_axum::{middleware::azure::AzureAccessToken, problem::Problem};

use crate::{config::CONFIG, state::KubeClient};

pub(crate) const MANAGED_BY_LABEL: &str = "app.kubernetes.io/managed-by";
pub(crate) const NAME_LABEL: &str = "app.kubernetes.io/name";
pub(crate) const PART_OF_LABEL: &str = "app.kubernetes.io/part-of";
pub(crate) const COMPONENT_LABEL: &str = "app.kubernetes.io/component";
pub(crate) const AGENT_LABEL: &str = "lerpz.com/agent";
pub(crate) const CREATED_BY_OID_LABEL: &str = "lerpz.com/created-by-oid";
pub(crate) const CREATED_BY_TENANT_ID_LABEL: &str = "lerpz.com/created-by-tenant-id";
pub(crate) const OWNER_TYPE_LABEL: &str = "lerpz.com/owner-type";
pub(crate) const OWNER_ID_LABEL: &str = "lerpz.com/owner-id";
pub(crate) const OWNER_TENANT_ID_LABEL: &str = "lerpz.com/owner-tenant-id";

const USER_OWNER_TYPE: &str = "user";

/// Value of [`MANAGED_BY_LABEL`] on every object Forge creates.
pub(crate) const MANAGED_BY: &str = "forge";

/// Longest accepted agent identifier.
///
/// Object names are derived as `agent-{id}-{component}`, and a Kubernetes
/// object name may not exceed 63 characters. 40 leaves comfortable room for the
/// prefix and suffix.
const MAX_AGENT_LEN: usize = 40;

/// Label selector matching only the objects Forge owns.
pub(crate) fn managed_selector() -> String {
    format!("{MANAGED_BY_LABEL}={MANAGED_BY}")
}

/// Label selector matching Forge resources belonging to the authenticated caller.
pub(crate) fn owned_selector(
    agent: Option<&str>,
    token: &AzureAccessToken,
) -> Result<String, Problem> {
    let (object_id, tenant_id) = caller_identity(token)?;
    let mut selector = format!(
        "{},{OWNER_TYPE_LABEL}={USER_OWNER_TYPE},{OWNER_ID_LABEL}={object_id},{OWNER_TENANT_ID_LABEL}={tenant_id}",
        managed_selector(),
    );
    if let Some(agent) = agent {
        validate_agent(agent)?;
        selector.push_str(&format!(",{AGENT_LABEL}={agent}"));
    }
    Ok(selector)
}

/// Checks management, agent and user ownership before named resource access.
pub(crate) fn is_owned_by(
    labels: Option<&BTreeMap<String, String>>,
    agent: &str,
    object_id: &str,
    tenant_id: &str,
) -> bool {
    !object_id.is_empty()
        && !tenant_id.is_empty()
        && labels.is_some_and(|labels| {
            [
                (MANAGED_BY_LABEL, MANAGED_BY),
                (AGENT_LABEL, agent),
                (OWNER_TYPE_LABEL, USER_OWNER_TYPE),
                (OWNER_ID_LABEL, object_id),
                (OWNER_TENANT_ID_LABEL, tenant_id),
            ]
            .into_iter()
            .all(|(key, value)| labels.get(key).map(String::as_str) == Some(value))
        })
}

/// Name of the memory volume backing `agent`.
pub(crate) fn memory_volume_name(agent: &str) -> String {
    format!("agent-{agent}-memory")
}

/// Name of the runtime workload for `agent`.
pub(crate) fn runtime_name(agent: &str) -> String {
    format!("agent-{agent}-runtime")
}

/// The standard label set applied to every object Forge creates.
pub(crate) fn labels(agent: &str, component: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        (NAME_LABEL.to_owned(), format!("agent-{agent}")),
        (COMPONENT_LABEL.to_owned(), component.to_owned()),
        (PART_OF_LABEL.to_owned(), "lerpz".to_owned()),
        (MANAGED_BY_LABEL.to_owned(), MANAGED_BY.to_owned()),
        (AGENT_LABEL.to_owned(), agent.to_owned()),
    ])
}

/// The object and tenant IDs required for private resource access.
pub(crate) fn caller_identity(token: &AzureAccessToken) -> Result<(&str, &str), Problem> {
    let object_id = token.oid.as_deref().unwrap_or_default();
    if object_id.is_empty() || token.tid.is_empty() {
        return Err(Problem::new(
            StatusCode::UNAUTHORIZED,
            "Missing caller identity",
            "The authentication token must contain non-empty oid and tid claims.",
        ));
    }

    Ok((object_id, token.tid.as_str()))
}

/// Records the creator and initial user ownership from a validated Entra token.
///
/// Keep these labels out of deployment selectors, which are immutable.
pub(crate) fn creation_labels(
    token: &AzureAccessToken,
) -> Result<BTreeMap<String, String>, Problem> {
    let (object_id, tenant_id) = caller_identity(token)?;
    Ok(BTreeMap::from([
        (CREATED_BY_OID_LABEL.to_owned(), object_id.to_owned()),
        (CREATED_BY_TENANT_ID_LABEL.to_owned(), tenant_id.to_owned()),
        (OWNER_TYPE_LABEL.to_owned(), USER_OWNER_TYPE.to_owned()),
        (OWNER_ID_LABEL.to_owned(), object_id.to_owned()),
        (OWNER_TENANT_ID_LABEL.to_owned(), tenant_id.to_owned()),
    ]))
}

/// Reads the `lerpz.com/agent` label off an object's metadata.
pub(crate) fn agent_of(labels: Option<&BTreeMap<String, String>>) -> Option<String> {
    labels.and_then(|labels| labels.get(AGENT_LABEL).cloned())
}

/// Rejects agent identifiers that would produce an invalid object name.
///
/// Kubernetes object names must be DNS-1123 labels: lowercase alphanumerics and
/// `-`, starting and ending with an alphanumeric. Validating here means an
/// invalid identifier is a `400` from Forge rather than a `422` relayed back
/// from the API server.
pub(crate) fn validate_agent(agent: &str) -> Result<(), Problem> {
    let invalid =
        |detail: String| Problem::new(StatusCode::BAD_REQUEST, "Invalid agent identifier", detail);

    if agent.is_empty() {
        return Err(invalid(
            "The agent identifier must not be empty.".to_owned(),
        ));
    }

    if agent.len() > MAX_AGENT_LEN {
        return Err(invalid(format!(
            "The agent identifier must be at most {MAX_AGENT_LEN} characters, got {}.",
            agent.len()
        )));
    }

    let valid_chars = agent
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    let valid_edges = !agent.starts_with('-') && !agent.ends_with('-');

    if !valid_chars || !valid_edges {
        return Err(invalid(
            "The agent identifier must be a DNS-1123 label: lowercase \
             alphanumerics and '-', starting and ending with an alphanumeric."
                .to_owned(),
        ));
    }

    Ok(())
}

/// Converts a Kubernetes timestamp into a `chrono` timestamp.
///
/// `k8s-openapi` represents times with `jiff`, while every other Lerpz service
/// serialises timestamps with `chrono`. Converting at this boundary keeps
/// Forge's OpenAPI schema consistent with the rest of the platform rather than
/// leaking a second time library into the API surface.
pub(crate) fn timestamp(time: Option<&Time>) -> Option<chrono::DateTime<chrono::Utc>> {
    let instant = time?.0;
    let nanos = u32::try_from(instant.subsec_nanosecond()).ok()?;
    chrono::DateTime::from_timestamp(instant.as_second(), nanos)
}

/// Typed handle for the memory volumes in the configured namespace.
pub(crate) fn volume_api(client: KubeClient) -> Api<PersistentVolumeClaim> {
    Api::namespaced(client, &CONFIG.KUBE_NAMESPACE)
}

/// Typed handle for the agent runtimes in the configured namespace.
pub(crate) fn runtime_api(client: KubeClient) -> Api<Deployment> {
    Api::namespaced(client, &CONFIG.KUBE_NAMESPACE)
}

/// Prevents deleting a resource replaced or modified after authorisation.
pub(crate) fn delete_params(metadata: &ObjectMeta) -> Result<DeleteParams, Problem> {
    let (Some(uid), Some(resource_version)) = (&metadata.uid, &metadata.resource_version) else {
        return Err(Problem::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Missing resource metadata",
            "The cluster did not provide the metadata required to safely delete the resource.",
        ));
    };

    Ok(DeleteParams {
        preconditions: Some(Preconditions {
            uid: Some(uid.clone()),
            resource_version: Some(resource_version.clone()),
        }),
        ..Default::default()
    })
}

/// The problem returned when a named resource is absent or inaccessible.
pub(crate) fn not_found(resource: &str, name: &str) -> Problem {
    Problem::new(
        StatusCode::NOT_FOUND,
        "Resource not found",
        format!("No accessible {resource} named '{name}' was found."),
    )
}

/// Translates a [`kube::Error`] into an RFC 9457 problem.
///
/// Status codes the API server reports are meaningful to our callers (a `409`
/// means "already provisioned", a `403` means Forge's RBAC is too narrow), so
/// they are surfaced rather than flattened into a generic `500`. Transport and
/// auth failures are reported as `502`, because from the caller's perspective
/// the fault is between Forge and the cluster, not in their request.
pub(crate) fn kube_problem(err: kube::Error, resource: &str) -> Problem {
    let status = match &err {
        kube::Error::Api(response) => {
            StatusCode::from_u16(response.code).unwrap_or(StatusCode::BAD_GATEWAY)
        }
        _ => StatusCode::BAD_GATEWAY,
    };

    // Anything the API server itself did not classify is a cluster-side fault.
    let status = if status.is_client_error() || status.is_server_error() {
        status
    } else {
        StatusCode::BAD_GATEWAY
    };

    let (title, detail) = match status {
        StatusCode::NOT_FOUND => (
            "Resource not found",
            format!("The {resource} does not exist."),
        ),
        StatusCode::CONFLICT => (
            "Resource already exists",
            format!("The {resource} has already been provisioned."),
        ),
        StatusCode::FORBIDDEN => (
            "Cluster access denied",
            format!("Forge is not permitted to manage the {resource} in this namespace."),
        ),
        _ => (
            "Cluster request failed",
            format!("The cluster rejected the request for the {resource}."),
        ),
    };

    tracing::error!(%resource, %status, "kubernetes request failed: {err}");

    Problem::new(status, title, detail).with_error(err)
}
