use jsonwebtoken::{Algorithm, Validation};

/// Default JWT validation claims.
///
/// The algorithm will always be [`Algorithm::RS256`].
pub fn get_token_validation(config: &super::AzureConfig) -> Validation {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_required_spec_claims(&["aud", "iss", "exp"]);
    validation.set_audience(&[&config.client_id]);
    validation.set_issuer(&[&config.get_issuer_url()]);
    validation.validate_exp = true;
    validation.validate_nbf = true;
    validation.leeway = 60;
    validation
}

/// Azure specific validation of token claims.
pub fn azure_claims_validation(
    config: &super::AzureConfig,
    claims: &super::AzureAccessToken,
) -> bool {
    if claims.ver.as_deref() != Some("2.0") {
        return false;
    }

    if let Some(tid) = &claims.tid {
        if tid != &config.tenant_id {
            return false;
        }
    } else {
        return false;
    }

    if let Some(sub) = &claims.sub {
        if sub.is_empty() {
            return false;
        }
    }

    true
}
