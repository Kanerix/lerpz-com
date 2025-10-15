//! Scheme 0 implemented using Argon2.

use std::sync::LazyLock;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};

use super::{
    Scheme,
    error::{Error, Result},
};

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(|| Argon2::default());

pub struct Scheme01;

impl Scheme for Scheme01 {
    fn hash(&self, pwd: &str, salt: &str) -> Result<String> {
        let salt = SaltString::encode_b64(salt.as_bytes()).map_err(Error::PwdHash)?;

        let pwd = ARGON2
            .hash_password(pwd.as_bytes(), &salt)
            .map_err(Error::PwdHash)?
            .to_string();

        Ok(pwd)
    }

    fn validate(&self, pwd_hash: &str, pwd_ref: &str, _pwd_ref_salt: &str) -> Result<bool> {
        let pwd_hash_parsed = PasswordHash::new(pwd_hash).map_err(Error::PwdHash)?;

        let pwd_ref_bytes = pwd_ref.as_bytes();

        Ok(ARGON2
            .verify_password(pwd_ref_bytes, &pwd_hash_parsed)
            .is_ok())
    }
}
