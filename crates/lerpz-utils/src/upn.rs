//! Utilities for generating User Principal Names (UPNs).
//!
//! A UPN is an internet-style login name for user based identities. It is
//! commonly used in Microsoft environments, but can be used in other contexts
//! as well. This will often look like a mail address, but does not have to be
//! one.
//!
//! ### Globally unique identifier (GUID)
//!
//! Allowed characters are the English alphabet (a-z) and digits (0-9). The GUID
//! MUST always be lowercase. Special characters in names (such as `ø`, `é`, `ä`, `ß`)
//! are normalised to their closest ASCII equivalents (e.g., `ø` → `o`, `é` → `e`)
//! before constructing the GUID. A special character without a mapping is skipped,
//! and a name left with fewer than two usable characters is rejected.
//!
//! Format: `<x><y>.<w>@<domain>`
//!
//! Where:
//!
//! - **x** = first 2 to 3 letters of the user's first name
//! - **y** = first 2 to 3 letters of the user's last name (or middle name)
//! - **w** = the last 2 digits of the year the user was employed
//! - **domain** = the company's email domain (letters, digits, and `-` as allowed by normal DNS rules)
//!
//! The domain is checked with [`is_valid_domain`] before it goes into a UPN.
//!
//! Examples:
//! - Kasper Jønsson, Engineer - 15/11/2020 @ lerpz.com -> kasjon.20@lerpz.com
//!
//! ### Borrowed and owned user information
//!
//! [`UserInfo`] holds [`Cow`] fields, so it reads straight out of whatever the
//! caller already has, be that string literals, a database row or command line
//! arguments. Borrowing ties the value to the data it came from. Call
//! [`UserInfo::into_owned`] for an [`OwnedUserInfo`], which borrows nothing and
//! is therefore `Send + Sync + 'static`, ready to move into a thread, a task or
//! a struct that outlives the source.
//!
//! ### Generating in bulk
//!
//! [`generate_upn`] allocates the string it returns. When walking a table of
//! users, [`generate_upn_into`] appends to a buffer the caller owns, so one
//! allocation covers the whole run.

use std::borrow::Cow;

/// A type alias for handling results from this module.
///
/// This is a convenience alias for `Result<T, Error>` where [`Error`]
/// represents UPN-specific errors.
pub type Result<T> = std::result::Result<T, Error>;

/// A [`UserInfo`] that borrows nothing, and so is `Send + Sync + 'static`.
///
/// Produced by [`UserInfo::into_owned`].
pub type OwnedUserInfo = UserInfo<'static>;

/// Longest `<x><y>.<w>@` the format can produce, being three letters from each
/// name, the dot, two year digits and the separator.
const MAX_LEN_BEFORE_DOMAIN: usize = 10;

/// The DNS limit on a whole domain name.
const MAX_DOMAIN_LEN: usize = 253;

/// The DNS limit on a single dot separated label.
const MAX_LABEL_LEN: usize = 63;

/// Errors that can occur when generating a UPN.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// The user has no surname to build the second part of the GUID from.
    #[error("user has no surname")]
    NoSurname,
    /// Normalisation left the name with fewer than the two characters the
    /// format requires.
    #[error("name \"{0}\" has fewer than two usable characters")]
    UnusableName(String),
    /// The domain is not one [`is_valid_domain`] accepts.
    #[error("invalid domain \"{0}\"")]
    InvalidDomain(String),
}

/// User information to generate a UPN for.
///
/// Each field borrows or owns independently, so a mix of literals and owned
/// strings is fine. Note that a single borrowed field ties the whole value to
/// that borrow. Use [`UserInfo::into_owned`] to detach from it.
#[derive(Debug, Clone)]
pub struct UserInfo<'a> {
    pub forename: Cow<'a, str>,
    pub surnames: Vec<Cow<'a, str>>,
    pub hire_year: u32,
    pub domain: Cow<'a, str>,
}

impl<'a> UserInfo<'a> {
    pub fn new(
        forename: impl Into<Cow<'a, str>>,
        surnames: impl IntoIterator<Item = impl Into<Cow<'a, str>>>,
        hire_year: u32,
        domain: impl Into<Cow<'a, str>>,
    ) -> UserInfo<'a> {
        Self {
            forename: forename.into(),
            surnames: surnames.into_iter().map(Into::into).collect(),
            hire_year,
            domain: domain.into(),
        }
    }

    /// Takes ownership of every borrowed field.
    ///
    /// Already owned fields are moved, borrowed ones are copied. The result
    /// outlives whatever the value was built from, which is what lets it cross
    /// a thread or task boundary.
    pub fn into_owned(self) -> OwnedUserInfo {
        UserInfo {
            forename: Cow::Owned(self.forename.into_owned()),
            surnames: self
                .surnames
                .into_iter()
                .map(|surname| Cow::Owned(surname.into_owned()))
                .collect(),
            hire_year: self.hire_year,
            domain: Cow::Owned(self.domain.into_owned()),
        }
    }
}

/// Generate a UPN based on the provided user information.
///
/// This is a shorthand for [`generate_upn_with_iteration`] with iteration set to 0.
pub fn generate_upn(user: &UserInfo<'_>) -> Result<String> {
    generate_upn_with_iteration(user, 0)
}

/// Generate a UPN based on the provided user information and iteration number.
///
/// The iteration number is used to select which surname to use when the user
/// has multiple surnames. The iterator will start with the rightmost surname
/// and move leftwards as the iterations increase. If the iteration number
/// exceeds the number of surnames, it will wrap around to the rightmost surname,
/// so a caller looking for an unused UPN has no options left after
/// `surnames.len()` iterations.
///
/// ### Example:
///
/// ```no_test
/// use lerpz_utils::upn::{UserInfo, generate_upn_with_iteration};
///
/// let user_info = UserInfo::new("Kasper", ["Jønsson"], 2020, "lerpz.com");
///
/// let upn = generate_upn_with_iteration(&user_info, 0)
///     .expect("user info has a usable name and a surname");
///
/// assert_eq!(upn, "kasjon.20@lerpz.com");
/// ```
pub fn generate_upn_with_iteration(user: &UserInfo<'_>, iteration: usize) -> Result<String> {
    let mut buf = String::with_capacity(MAX_LEN_BEFORE_DOMAIN + user.domain.len());
    generate_upn_into(user, iteration, &mut buf)?;
    Ok(buf)
}

/// Append a UPN to an existing buffer instead of allocating a new string.
///
/// ### Example:
///
/// ```
/// use lerpz_utils::upn::{UserInfo, generate_upn_into};
///
/// let users = [
///     UserInfo::new("Kasper", ["Jønsson"], 2020, "lerpz.com"),
///     UserInfo::new("Astrid", ["Sørensen"], 2019, "lerpz.com"),
/// ];
///
/// let mut buf = String::new();
/// let mut upns = Vec::new();
/// for user in &users {
///     buf.clear();
///     generate_upn_into(user, 0, &mut buf).expect("both users have a surname");
///     upns.push(buf.clone());
/// }
///
/// assert_eq!(upns, ["kasjon.20@lerpz.com", "astsor.19@lerpz.com"]);
/// ```
pub fn generate_upn_into(user: &UserInfo<'_>, iteration: usize, buf: &mut String) -> Result<()> {
    let original_len = buf.len();
    buf.reserve(MAX_LEN_BEFORE_DOMAIN + user.domain.len());

    match write_upn(user, iteration, buf) {
        Ok(()) => Ok(()),
        Err(err) => {
            buf.truncate(original_len);
            Err(err)
        }
    }
}

fn write_upn(user: &UserInfo<'_>, iteration: usize, buf: &mut String) -> Result<()> {
    if !is_valid_domain(&user.domain) {
        return Err(Error::InvalidDomain(user.domain.to_string()));
    }

    shortname(&user.forename, buf)?;

    let surname = get_surname(&user.surnames, iteration).ok_or(Error::NoSurname)?;
    shortname(surname, buf)?;

    buf.push('.');
    push_hire_year(user.hire_year, buf);

    buf.push('@');
    for c in user.domain.chars() {
        buf.push(c.to_ascii_lowercase());
    }

    Ok(())
}

/// Checks that a domain can be used in a UPN.
///
/// Accepts dot separated labels of ASCII letters, digits and hyphens, with no
/// hyphen at either end of a label, within the DNS length limits. Case is
/// ignored, since the domain is lowercased when the UPN is built.
///
/// An internationalised domain has to be punycode encoded before it gets here,
/// as anything outside ASCII is rejected.
///
/// ### Example:
///
/// ```
/// use lerpz_utils::upn::is_valid_domain;
///
/// assert!(is_valid_domain("lerpz.com"));
/// assert!(is_valid_domain("my-company.co.uk"));
///
/// assert!(!is_valid_domain("lerpz.com."));
/// assert!(!is_valid_domain("not a @ domain"));
/// ```
pub fn is_valid_domain(domain: &str) -> bool {
    if domain.is_empty() || domain.len() > MAX_DOMAIN_LEN {
        return false;
    }

    domain.split('.').all(is_valid_label)
}

fn is_valid_label(label: &str) -> bool {
    let bytes = label.as_bytes();

    if bytes.is_empty() || bytes.len() > MAX_LABEL_LEN {
        return false;
    }

    if bytes.starts_with(b"-") || bytes.ends_with(b"-") {
        return false;
    }

    bytes
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
}

/// Gets the surname to use for the given iteration.
///
/// This will start with the most right surname and move leftwards as the
/// iteration number increases. Then it resets to the rightmost surname and
/// continues. Returns [`None`] when there are no surnames to pick from.
#[inline]
fn get_surname<T>(surnames: &[T], iteration: usize) -> Option<&T> {
    if surnames.is_empty() {
        return None;
    }

    let idx = surnames.len() - 1 - (iteration % surnames.len());
    Some(&surnames[idx])
}

/// Pushes the last two digits of the hire year, zero padded.
#[inline]
fn push_hire_year(hire_year: u32, buf: &mut String) {
    let year = hire_year % 100;
    buf.push(char::from(b'0' + (year / 10) as u8));
    buf.push(char::from(b'0' + (year % 10) as u8));
}

/// Replace non-allowed characters with an allowed equivalent.
///
/// Returns [`None`] for a character that has no ASCII equivalent, which the
/// format says to skip.
#[inline]
pub fn replace_char(c: char) -> Option<char> {
    if c.is_ascii() {
        let lowercase = c.to_ascii_lowercase();
        return matches!(lowercase, 'a'..='z' | '0'..='9').then_some(lowercase);
    }

    replace_non_ascii_char(c)
}

/// Lowercasing outside ASCII needs the Unicode tables, which is both slower and
/// far rarer than the path in [`replace_char`], so it stays out of line.
fn replace_non_ascii_char(c: char) -> Option<char> {
    match c.to_lowercase().next()? {
        lowercase @ ('a'..='z' | '0'..='9') => Some(lowercase),
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'æ' => Some('a'),
        'ç' => Some('c'),
        'è' | 'é' | 'ê' | 'ë' => Some('e'),
        'ì' | 'í' | 'î' | 'ï' => Some('i'),
        'ñ' => Some('n'),
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => Some('o'),
        'ß' => Some('s'),
        'ù' | 'ú' | 'û' | 'ü' => Some('u'),
        'ý' | 'ÿ' => Some('y'),
        _ => None,
    }
}

/// Pushes the three first legal characters to buf.
///
/// The format needs at least two, so a name that normalises to less than that
/// is an [`Error::UnusableName`].
#[inline]
fn shortname(name: &str, buf: &mut String) -> Result<()> {
    let mut pushed = 0;
    for c in name.chars().filter_map(replace_char).take(3) {
        buf.push(c);
        pushed += 1;
    }

    if pushed < 2 {
        return Err(Error::UnusableName(name.to_owned()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::upn::{
        Error, OwnedUserInfo, UserInfo, generate_upn, generate_upn_into,
        generate_upn_with_iteration, is_valid_domain,
    };

    #[test]
    fn illegal_char() {
        let user_info = UserInfo::new("Kasper", vec!["Jønsson"], 2020, "lerpz.com");
        let upn = generate_upn(&user_info).expect("user info has a surname and a valid year");
        assert_eq!(upn, "kasjon.20@lerpz.com");
    }

    #[test]
    fn uppercase_illegal_char() {
        let user_info = UserInfo::new("Søren", vec!["Åström"], 2020, "lerpz.com");
        let upn = generate_upn(&user_info).expect("user info has a surname and a valid year");
        assert_eq!(upn, "sorast.20@lerpz.com");
    }

    #[test]
    fn accented_char() {
        let user_info = UserInfo::new("Éric", vec!["Gonçalves"], 2020, "lerpz.com");
        let upn = generate_upn(&user_info).expect("user info has a surname and a valid year");
        assert_eq!(upn, "erigon.20@lerpz.com");
    }

    #[test]
    fn unmapped_char_is_skipped() {
        let user_info = UserInfo::new("Łukas", vec!["Nowak"], 2020, "lerpz.com");
        let upn = generate_upn(&user_info).expect("user info has a surname and a valid year");
        assert_eq!(upn, "ukanow.20@lerpz.com");
    }

    #[test]
    fn surname_iterations() {
        let user_info = UserInfo::new(
            "Kasper",
            vec!["Sørensen", "Tørkilsen", "Jønsson"],
            2020,
            "lerpz.com",
        );

        let upn = generate_upn_with_iteration(&user_info, 0)
            .expect("user info has three surnames and a valid year");
        assert_eq!(upn, "kasjon.20@lerpz.com");

        let upn = generate_upn_with_iteration(&user_info, 1)
            .expect("user info has three surnames and a valid year");
        assert_eq!(upn, "kastor.20@lerpz.com");

        let upn = generate_upn_with_iteration(&user_info, 2)
            .expect("user info has three surnames and a valid year");
        assert_eq!(upn, "kassor.20@lerpz.com");

        let upn = generate_upn_with_iteration(&user_info, 3)
            .expect("user info has three surnames and a valid year");
        assert_eq!(upn, "kasjon.20@lerpz.com");

        let upn = generate_upn_with_iteration(&user_info, 4)
            .expect("user info has three surnames and a valid year");
        assert_eq!(upn, "kastor.20@lerpz.com");

        let upn = generate_upn_with_iteration(&user_info, 5)
            .expect("user info has three surnames and a valid year");
        assert_eq!(upn, "kassor.20@lerpz.com");
    }

    #[test]
    fn two_letter_names() {
        let user_info = UserInfo::new("bo", vec!["Bi"], 1995, "lerpz.com");

        let upn = generate_upn(&user_info).expect("user info has a surname and a valid year");
        assert_eq!(upn, "bobi.95@lerpz.com");
    }

    #[test]
    fn single_digit_hire_year() {
        let user_info = UserInfo::new("Kasper", vec!["Jonsson"], 2005, "lerpz.com");

        let upn = generate_upn(&user_info).expect("user info has a surname and a valid year");
        assert_eq!(upn, "kasjon.05@lerpz.com");
    }

    #[test]
    fn uppercase_domain() {
        let user_info = UserInfo::new("Kasper", vec!["Jonsson"], 2020, "LERPZ.COM");

        let upn = generate_upn(&user_info).expect("user info has a surname and a valid year");
        assert_eq!(upn, "kasjon.20@lerpz.com");
    }

    #[test]
    fn no_surname() {
        let user_info = UserInfo::new("Kasper", Vec::<&str>::new(), 2020, "lerpz.com");

        let err = generate_upn(&user_info).expect_err("user info has no surname");
        assert!(matches!(err, Error::NoSurname));
    }

    #[test]
    fn name_without_usable_characters() {
        let user_info = UserInfo::new("Kasper", vec!["李"], 2020, "lerpz.com");

        let err = generate_upn(&user_info).expect_err("the surname has no ascii equivalent");
        assert!(matches!(err, Error::UnusableName(name) if name == "李"));
    }

    #[test]
    fn single_character_name() {
        let user_info = UserInfo::new("K", vec!["Jonsson"], 2020, "lerpz.com");

        let err = generate_upn(&user_info).expect_err("the forename is a single character");
        assert!(matches!(err, Error::UnusableName(name) if name == "K"));
    }

    #[test]
    fn appends_to_existing_buffer() {
        let user_info = UserInfo::new("Kasper", vec!["Jønsson"], 2020, "lerpz.com");

        let mut buf = String::from("upn: ");
        generate_upn_into(&user_info, 0, &mut buf).expect("user info has a surname");

        assert_eq!(buf, "upn: kasjon.20@lerpz.com");
    }

    #[test]
    fn failed_generation_leaves_buffer_untouched() {
        let user_info = UserInfo::new("Kasper", vec!["李"], 2020, "lerpz.com");

        let mut buf = String::from("upn: ");
        generate_upn_into(&user_info, 0, &mut buf)
            .expect_err("the surname has no ascii equivalent");

        assert_eq!(buf, "upn: ");
    }

    #[test]
    fn valid_domains() {
        for domain in [
            "lerpz.com",
            "LERPZ.COM",
            "sub.lerpz.co.uk",
            "my-company.com",
            "123.com",
            "localhost",
        ] {
            assert!(is_valid_domain(domain), "{domain} should be accepted");
        }
    }

    #[test]
    fn invalid_domains() {
        for domain in [
            "",
            "not a @ domain",
            "lerpz..com",
            "lerpz.com.",
            ".lerpz.com",
            "-lerpz.com",
            "lerpz-.com",
            "lerpz_com",
            "lerpz.café",
        ] {
            let user_info = UserInfo::new("Kasper", vec!["Jonsson"], 2020, domain);

            let err = generate_upn(&user_info).expect_err("the domain is invalid");
            assert!(matches!(err, Error::InvalidDomain(invalid) if invalid == domain));
        }
    }

    #[test]
    fn domain_length_limits() {
        let longest_label = "a".repeat(63);
        assert!(is_valid_domain(&longest_label));

        let overlong_label = "a".repeat(64);
        assert!(!is_valid_domain(&overlong_label));

        let overlong_domain = [longest_label.as_str(); 4].join(".");
        assert!(!is_valid_domain(&overlong_domain));
    }

    #[test]
    fn invalid_domain_leaves_buffer_untouched() {
        let user_info = UserInfo::new("Kasper", vec!["Jonsson"], 2020, "not a @ domain");

        let mut buf = String::from("upn: ");
        generate_upn_into(&user_info, 0, &mut buf).expect_err("the domain is invalid");

        assert_eq!(buf, "upn: ");
    }

    #[test]
    fn owned_user_info_is_send_and_sync() {
        fn assert_send_sync_static<T: Send + Sync + 'static>() {}
        assert_send_sync_static::<OwnedUserInfo>();
    }

    #[test]
    fn owned_user_info_crosses_thread_boundary() {
        let forename = String::from("Kasper");
        let user_info = UserInfo::new(forename.as_str(), vec!["Jønsson"], 2020, "lerpz.com");
        let user_info = user_info.into_owned();

        let upn = std::thread::spawn(move || generate_upn(&user_info))
            .join()
            .expect("the worker thread does not panic")
            .expect("user info has a surname and a valid year");

        assert_eq!(upn, "kasjon.20@lerpz.com");
    }
}
