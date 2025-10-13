//! Schemas used for password hashing.

/// Hashing and validation schemes errors.
pub mod error;
/// Password scheme that uses argon2.
pub mod scheme_01;

use error::{Error, Result};
use scheme_01::Scheme01;

/// Implemented by schemes that can hash and validate passwords.
pub trait Scheme {
    /// Hashes a password from some [`PwdParts`](super::parts::PwdParts).
    fn hash(&self, pwd: &str, salt: &str) -> Result<String>;
    /// Validate a password hash against a real password and a salt.
    fn validate(&self, pwd_hash: &str, pwd_ref: &str, pwd_ref_salt: &str) -> Result<bool>;
}

/// Returns a scheme given a scheme name as a string.
#[inline]
pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme> {
    match scheme_name {
        "01" => Ok(Scheme01),
        _ => Err(Error::SchemeNotFound(scheme_name.into())),
    }
}
