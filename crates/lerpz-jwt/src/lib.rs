//! Functions related to JWT token generation and verification.
//!
//! The purpose of this module is to provide a generic way of handling JWT
//! tokens in a platform-independent manner. This should streamline the process
//! of generating and verifying JWT tokens. This will also allow for
//! generalising security and simplifying the process of working with JWT
//! tokens.

/// Claims related to JWT tokens.
pub mod claims;
/// Errors that can occur when working with JWT tokens.
pub mod error;

pub use jsonwebtoken::{DecodingKey, EncodingKey};
use jsonwebtoken::{Header, TokenData, Validation, decode, encode};

pub use claims::Claims;
pub use error::{Error, Result};


/// Create a new JWT token with the given claims and key.
///
/// This uses [`encode_jwt_with_header`] and uses [`Header::default()`] as
/// header value.
pub fn encode_jwt(claims: impl Into<Claims>, key: &EncodingKey) -> Result<String> {
    let header = Header::default();
    let token = encode_jwt_with_header(claims, header, key)?;
    Ok(token)
}

pub fn encode_jwt_with_header(
    claims: impl Into<Claims>,
    header: Header,
    key: &EncodingKey,
) -> Result<String> {
    let claims = claims.into();
    let token = encode(&header, &claims, key).map_err(Error::TokenError)?;
    Ok(token)
}

/// Decode given token into token data with claims.
///
/// This uses [`decode_jwt_with_validation`] and uses [`Validation::default()`] as
/// validation value.
pub fn decode_jwt(token: &str, key: &DecodingKey) -> Result<TokenData<Claims>> {
    let validation = Validation::default();
    let token_data = decode_jwt_with_validation(token, key, &validation)?;
    Ok(token_data)
}

pub fn decode_jwt_with_validation(
    token: &str,
    key: &DecodingKey,
    validation: &Validation,
) -> Result<TokenData<Claims>> {
    let token_data = decode::<Claims>(token, key, &validation).map_err(Error::TokenError)?;
    Ok(token_data)
}
