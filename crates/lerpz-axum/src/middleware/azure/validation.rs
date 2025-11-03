use jsonwebtoken::{Algorithm, Validation};

/// Default JWT validation claims.
///
/// The algorithm will always be [`Algorithm::RS256`].
pub fn get_token_validation(
    config: &super::AzureConfig,
) -> Validation {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_required_spec_claims(&["aud", "iss", "exp"]);
    validation.set_audience(&[&config.client_id]);
    validation.set_issuer(&[&config.issuer]);
    validation.validate_exp = true;
    validation.validate_nbf = true;
    validation.leeway = 60;
    validation
}
