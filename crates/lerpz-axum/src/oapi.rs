//! OpenAPI helpers for documenting Lerpz services with [`utoipa`].

use std::borrow::Cow;

use utoipa::Modify;
use utoipa::openapi::SecurityRequirement;
use utoipa::openapi::security::{AuthorizationCode, Flow, OAuth2, Scopes, SecurityScheme};

/// A [`utoipa`] [`Modify`] that registers a Microsoft Entra ID OAuth2
/// (authorization code) security scheme on an OpenAPI document.
///
/// The scheme is registered under the `oauth2` name and added as a global
/// security requirement.
///
/// Because [`utoipa`]'s `modifiers(...)` attribute only accepts a reference to
/// a path (not a constructor call), apply this modifier at runtime after
/// generating the document:
///
/// ```rust
/// use lerpz_axum::oapi::EntraAuth;
/// use utoipa::{Modify, OpenApi};
///
/// #[derive(OpenApi)]
/// #[openapi()]
/// struct ApiDoc;
///
/// let mut openapi = ApiDoc::openapi();
/// EntraAuth::new("my-tenant-id", "api://my-app/.default").modify(&mut openapi);
/// ```
pub struct EntraAuth {
    tenant_id: Cow<'static, str>,
    client_scope: Cow<'static, str>,
}

impl EntraAuth {
    /// Create a new [`EntraAuth`] modifier.
    ///
    /// * `tenant_id` is the Entra ID tenant the tokens are issued for.
    /// * `client_scope` is the scope that clients must request for API access.
    pub fn new(
        tenant_id: impl Into<Cow<'static, str>>,
        client_scope: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            client_scope: client_scope.into(),
        }
    }
}

impl Modify for EntraAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let ms_url = "https://login.microsoftonline.com";
        let tenant_id = self.tenant_id.as_ref();
        let client_scope = self.client_scope.as_ref();
        let auth_url = format!("{ms_url}/{tenant_id}/oauth2/v2.0/authorize");
        let token_url = format!("{ms_url}/{tenant_id}/oauth2/v2.0/token");

        let oauth2_scheme = SecurityScheme::OAuth2(OAuth2::new([Flow::AuthorizationCode(
            AuthorizationCode::new(
                auth_url,
                token_url,
                Scopes::from_iter([(client_scope, "Default API access")]),
            ),
        )]));

        openapi
            .components
            .get_or_insert_with(utoipa::openapi::Components::new)
            .add_security_scheme("oauth2", oauth2_scheme);

        openapi
            .security
            .get_or_insert_with(Vec::new)
            .push(SecurityRequirement::new("oauth2", [client_scope]));
    }
}
