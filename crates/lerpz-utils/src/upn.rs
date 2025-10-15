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

/// Errors that can occur when generating a UPN.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid department code")]
    DepartmentError,
    #[error("Hire date is in the future")]
    FutureHireDateError,
    #[error("Formatting error: {0}")]
    FmtError(#[from] std::fmt::Error),
}

/// Information about the user to generate a UPN for.
#[derive(Debug, Clone)]
pub struct UserInfo {
    forename: String,
    surnames: Vec<String>,
    department: String,
    hire_date: chrono::NaiveDate,
    domain: String,
}

/// Generate a UPN based on the provided user information.
///
/// This is a shorthand for [`generate_upn_with_iteration`] with iteration set to 0.
pub fn generate_upn(upn: impl Into<UserInfo>) -> Result<String, Error> {
    generate_upn_with_iteration(upn, 0)
}

/// Generate a UPN based on the provided user information and iteration number.
///
/// The iteration number is used to select which surname to use when the user has
/// multiple surnames. The iteration will start with the rightmost surname and
/// move leftwards as the iteration number increases. If the iteration number
/// exceeds the number of surnames, it will wrap around to the rightmost surname
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

    let forename = validate_characters(upn.forename);
    buf.push_str(shortname(&forename));

    let surname_index = get_surname_index(&upn.surnames, i);
    let surname = validate_characters(upn.surnames[surname_index].to_owned());
    buf.push_str(shortname(&surname));

    buf.push('.');

    let department = validate_characters(upn.department.clone());
    buf.push_str(shortname(&department));

    upn.hire_date.format("%y").write_to(&mut buf)?;

    buf = buf.to_lowercase();

    buf.push('@');
    buf.push_str(&upn.domain);

    Ok(buf)
}

/// Will replace some characters with a short version (e.g. æ,ø,å) equivalent.
///
/// This will also remove any non-alphabetic characters. This also assmumes that
/// all characters are in lowercase. Uppsercase characters will be removed.
pub fn validate_characters(upn: String) -> String {
    upn.to_lowercase()
        .chars()
        .filter_map(|c| match c {
            'å' | 'æ' | 'ä' => Some('a'),
            'ø' | 'ö' => Some('o'),
            _ if ('a'..='z').contains(&c) || c.is_numeric() => Some(c),
            _ => None,
        })
        .collect()
}

/// Gets the lastname index based on the iteration number.
///
/// This will start with the most right surname and move leftwards as the
/// iteration number increases. Then it resets to the rightmost surname and
/// continues.
///
/// ### Note
///
/// The implementation will change in the future to support more usecases.
fn get_surname_index(surnames: &[String], i: usize) -> usize {
    if i == 0 {
        return surnames.len() - 1;
    } else {
        return surnames.len() - 1 - (i % surnames.len());
    }
}

/// Returns the first three characters.
///
/// If the 3 first characters are not available, returns the whole string.
fn shortname<'a>(s: &str) -> &str {
    &s[..if s.len() > 3 { 3 } else { s.len() }]
}

#[cfg(test)]
mod tests {
    use crate::upn::{UserInfo, generate_upn, generate_upn_with_iteration};

    #[test]
    fn basic() {
        let user_info = UserInfo {
            forename: "Kasper".to_owned(),
            surnames: vec!["Jønsson".to_owned()],
            department: "Engineering".to_owned(),
            hire_date: chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
            domain: "lerpz.com".to_owned(),
        };

        let upn = generate_upn(user_info).unwrap();
        assert_eq!(upn, "kasjon.eng20@lerpz.com");
    }

    #[test]
    fn basic_iteration() {
        let user_info = UserInfo {
            forename: "Kasper".to_owned(),
            surnames: vec![
                "George".to_owned(),
                "Henriksen".to_owned(),
                "Jønsson".to_owned(),
            ],
            department: "Engineering".to_owned(),
            hire_date: chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
            domain: "lerpz.com".to_owned(),
        };

        let upn = generate_upn_with_iteration(user_info.clone(), 0).unwrap();
        assert_eq!(upn, "kasjon.eng20@lerpz.com");

        let upn = generate_upn_with_iteration(user_info.clone(), 1).unwrap();
        assert_eq!(upn, "kashen.eng20@lerpz.com");

        let upn = generate_upn_with_iteration(user_info, 2).unwrap();
        assert_eq!(upn, "kasgeo.eng20@lerpz.com");
    }

    #[test]
    fn basic_shortname() {
        let user_info = UserInfo {
            forename: "Bo".to_owned(),
            surnames: vec!["Bi".to_owned()],
            department: "IT".to_owned(),
            hire_date: chrono::NaiveDate::from_ymd_opt(1995, 10, 15).unwrap(),
            domain: "lerpz.com".to_owned(),
        };

        let upn = generate_upn(user_info).unwrap();
        assert_eq!(upn, "bobi.it95@lerpz.com");
    }
}
