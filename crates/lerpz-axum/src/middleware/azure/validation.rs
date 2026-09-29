use jsonwebtoken::{Algorithm, Validation, decode, decode_header};

use super::{AzureAccessToken, AzureConfig};
use crate::problem::Problem;

/// Validates an encoded Entra access token and returns its claims.
///
/// `token` must not include the `Bearer ` prefix. This checks the signature,
/// issuer, audience, lifetime and Azure claims, but does not enforce application
/// permissions such as scopes, roles or resource ownership.
pub async fn validate_azure_token(
    token: &str,
    config: &AzureConfig,
) -> Result<AzureAccessToken, Problem> {
    let header = match decode_header(token) {
        Ok(header) => header,
        Err(err) => {
            tracing::debug!("failed to decode JWT token header: {err}");
            return Err(Problem::unauthorized());
        }
    };
    let kid = match header.kid {
        Some(kid) => kid,
        None => {
            tracing::debug!("JWT token does not provide a 'kid' in header");
            return Err(Problem::unauthorized());
        }
    };

    let decoding_key = match config.find_jwk(&kid).await {
        Ok(Some(key)) => key,
        Ok(None) => {
            tracing::warn!("unknown key ID: {}", kid);
            return Err(Problem::unauthorized());
        }
        Err(err) => {
            tracing::error!("failed to find JWK: {err}");
            return Err(Problem::from(err));
        }
    };

    let validation = get_token_validation(config);
    let token_data = match decode::<AzureAccessToken>(token, &decoding_key, &validation) {
        Ok(token) => token,
        Err(err) => {
            tracing::trace!("failed to validate JWT claims: {err}");
            return Err(Problem::unauthorized());
        }
    };

    if !config.validate_azure_claims(&token_data.claims) {
        tracing::trace!("failed to validate azure claims");
        return Err(Problem::unauthorized());
    }

    Ok(token_data.claims)
}

/// Default JWT validation claims.
///
/// The algorithm will always be [`Algorithm::RS256`].
pub fn get_token_validation(config: &super::AzureConfig) -> Validation {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_required_spec_claims(&["aud", "iss", "exp"]);
    validation.set_audience(&[&config.aud()]);
    validation.set_issuer(&[&config.iss()]);
    validation.validate_exp = true;
    validation.validate_nbf = true;
    validation.leeway = 60;
    validation
}
