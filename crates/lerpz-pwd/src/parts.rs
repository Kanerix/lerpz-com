use regex::Regex;
use std::{str::FromStr, sync::LazyLock};

use super::{DEFAULT_SCHEME, error::Error};

/// A regex that turns a password hash into its parts.
static PWD_PARTS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#(?<scheme>\w+)#(?<hash>.+)$").unwrap());

/// All parts a password needs to be hashed.
pub struct PwdParts {
    pub scheme: String,
    pub salt: String,
    pub pwd: String,
}

/// What passwords gets turned into when hashed.
#[derive(Debug, Clone)]
pub struct HashParts {
    pub scheme: String,
    pub hash: String,
}

impl PwdParts {
    /// Creates a new [`PwdParts`] structure.
    ///
    /// This will have the latest scheme for hashing.
    pub fn new(pwd: String, salt: String) -> Self {
        Self {
            scheme: DEFAULT_SCHEME.into(),
            salt,
            pwd,
        }
    }
}

impl HashParts {
    /// Creates a new [`HashParts`] structure.
    pub fn new(scheme: String, hash: String) -> Self {
        Self { scheme, hash }
    }
}

impl FromStr for HashParts {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PWD_PARTS_REGEX.captures(s).ok_or(Error::PwdParsingFailed(
            "password hash is not in the correct format".into(),
        ))?;

        let get_group = |name: &str| -> Result<String, Self::Err> {
            Ok(captures
                .name(name)
                .ok_or(Error::PwdParsingFailed(
                    "missing \"scheme\" part in password hash".into(),
                ))?
                .as_str()
                .to_string())
        };

        let scheme = get_group("scheme")?;
        let hash = get_group("hash")?;

        Ok(HashParts::new(scheme, hash))
    }
}
