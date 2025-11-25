//! Utilities for generating User Principal Names (UPNs).
//!
//! A UPN is an internet-style login name for user based identities. It is
//! commonly used in Microsoft environments, but can be used in other contexts
//! as well. This will often look like a mail address, but does not have to be
//! one.
//!
//! ### Globally unique identifier (GUID)
//!
//! Format: `<x><y>.<z><w>@<domain>`
//!
//! Where:
//! - **x** = first 3 letters of the user's first name
//! - **y** = first 3 letters of the user's last/middle name
//! - **z** = the user's department
//! - **w** = the last 2 digits of the year the user was employed
//! - **domain** = the company's email domain
//!
//! Examples:
//! - Kasper Jønsson, Engineer - 15/11/2020 @ lerpz.com -> kasjon.eng20@lerpz.com

use std::sync::Arc;

/// Errors that can occur when generating a UPN.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid department code")]
    DepartmentError,
    #[error("Hire date is in the future")]
    FutureHireDateError,
    #[error(transparent)]
    FmtError(#[from] std::fmt::Error),
}

/// User information to generate a UPN for.
#[derive(Debug, Clone)]
pub struct UserInfo {
    forename: Arc<str>,
    surnames: Arc<[Arc<str>]>,
    department: Arc<str>,
    hire_date: chrono::NaiveDate,
    domain: Arc<str>,
}

impl UserInfo {
    pub fn new(
        forename: impl Into<Arc<str>>,
        surnames: impl Into<Arc<[Arc<str>]>>,
        department: impl Into<Arc<str>>,
        hire_date: impl Into<chrono::NaiveDate>,
        domain: impl Into<Arc<str>>,
    ) -> UserInfo {
        Self {
            forename: forename.into(),
            surnames: surnames.into(),
            department: department.into(),
            hire_date: hire_date.into(),
            domain: domain.into(),
        }
    }
}

/// Generate a UPN based on the provided user information.
///
/// This is a shorthand for [`generate_upn_with_iteration`] with iteration set to 0.
pub fn generate_upn(upn: impl Into<UserInfo>) -> Result<String, Error> {
    generate_upn_with_iteration(upn, 0)
}

/// Generate a UPN based on the provided user information and iteration number.
///
/// The iteration number is used to select which surname to use when the user
/// has multiple surnames. The iterator will start with the rightmost surname
/// and move leftwards as the iterations increas. If the iteration number
/// exceeds the number of surnames, it will wrap around to the rightmost surname.
///
/// ### Example:
///
/// ```rust
/// let user_info = UserInfo {
///     forename: "Kasper".to_owned(),
///     surnames: vec!["Jønsson".to_owned()],
///     department: "Engineering".to_owned(),
///     hire_date: chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
///     domain: "lerpz.com".to_owned(),
/// };
///
/// let upn = generate_upn_with_iteration(user_info, 0).unwrap();
/// assert_eq!(upn, "kasjon.eng20@lerpz.com");
/// ```
pub fn generate_upn_with_iteration(upn: impl Into<UserInfo>, i: usize) -> Result<String, Error> {
    let upn: UserInfo = upn.into();
    let cap = 13 + upn.domain.len();
    let mut buf = String::with_capacity(cap);

    buf.push_str(&shortname(upn.forename.as_ref()));

    let surname = get_surname(&upn.surnames, i);
    buf.push_str(&shortname(surname));

    buf.push('.');

    buf.push_str(&shortname(&upn.department));

    upn.hire_date.format("%y").write_to(&mut buf)?;

    buf = buf.to_lowercase();

    buf.push('@');
    buf.push_str(&upn.domain);

    Ok(buf)
}

/// Replace non-allowed characters with an allowed equivalent.
#[inline]
pub fn replace_char(c: char) -> Option<char> {
    match c {
        _ if ('a'..='z').contains(&c) || c.is_numeric() => Some(c),
        'å' | 'æ' | 'ä' => Some('a'),
        'ø' | 'ö' => Some('o'),
        _ => None,
    }
}

/// Gets the the surnames based on the iteration.
///
/// This will start with the most right surname and move leftwards as the
/// iteration number increases. Then it resets to the rightmost surname and
/// continues.
#[inline]
fn get_surname<T>(surnames: &[T], iteration: usize) -> &T {
    match iteration {
        0 => &surnames[surnames.len() - 1],
        i => &surnames[surnames.len() - 1 - (i % surnames.len())],
    }
}

/// Returns the first three characters (shortname) of the name.
#[inline]
fn shortname(name: &str) -> String {
    let mut shortname = String::with_capacity(3);
    for c in name.chars() {
        if let Some(c) = replace_char(c) {
            shortname.push(c);
        }
        if shortname.len() == 3 {
            break;
        }
    }
    shortname
}

#[cfg(test)]
mod tests {
    use crate::upn::{UserInfo, generate_upn, generate_upn_with_iteration};

    #[test]
    fn basic() {
        let user_info = UserInfo::new(
            "Kasper",
            vec!["Jønsson".into()],
            "Engineering",
            chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
            "lerpz.com",
        );

        let upn = generate_upn(user_info).unwrap();
        assert_eq!(upn, "kasjon.eng20@lerpz.com");
    }

    #[test]
    fn surname_iterations() {
        let user_info = UserInfo::new(
            "Kasper",
            vec!["Sørensen".into(), "Tørkilsen".into(), "Jønsson".into()],
            "Engineering",
            chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
            "lerpz.com",
        );

        let upn = generate_upn_with_iteration(user_info.clone(), 0).unwrap();
        assert_eq!(upn, "kasjon.eng20@lerpz.com");

        let upn = generate_upn_with_iteration(user_info.clone(), 1).unwrap();
        assert_eq!(upn, "kastok.eng20@lerpz.com");

        let upn = generate_upn_with_iteration(user_info.clone(), 2).unwrap();
        assert_eq!(upn, "kassor.eng20@lerpz.com");

        let upn = generate_upn_with_iteration(user_info.clone(), 3).unwrap();
        assert_eq!(upn, "kasjon.eng20@lerpz.com");

        let upn = generate_upn_with_iteration(user_info.clone(), 4).unwrap();
        assert_eq!(upn, "kastok.eng20@lerpz.com");

        let upn = generate_upn_with_iteration(user_info.clone(), 5).unwrap();
        assert_eq!(upn, "kasson.eng20@lerpz.com");
    }

    #[test]
    fn two_letter_names() {
        let user_info = UserInfo::new(
            "Bo",
            vec!["Bi".into()],
            "IT",
            chrono::NaiveDate::from_ymd_opt(1995, 10, 15).unwrap(),
            "lerpz.com",
        );

        let upn = generate_upn(user_info).unwrap();
        assert_eq!(upn, "bobi.it95@lerpz.com");
    }
}
