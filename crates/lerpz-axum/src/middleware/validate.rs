use std::{borrow::Cow, collections::HashMap};

use axum::{
    Form, Json,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::{Serialize, de::DeserializeOwned};
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

use crate::problem::{HandlerResult, Problem};

/// Validator that validates the inner value.
///
/// This is using the [`validator`] crate.
pub struct Validated<T>(pub T);

/// Error response for validation errors.
///
/// This is used to return validation errors in a structured format. The format
/// is a map of field names to a list of errors for that field.
#[derive(Serialize, Debug, Clone)]
pub struct ErrorResponse {
    pub errors: HashMap<Cow<'static, str>, Errors>,
}

/// Errors in the individual fields.
#[derive(Serialize, Debug, Clone)]
pub struct Errors(Vec<Cow<'static, str>>);

impl From<ValidationErrors> for ErrorResponse {
    fn from(ValidationErrors(errs): ValidationErrors) -> Self {
        Self {
            errors: errs.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl From<ValidationErrorsKind> for Errors {
    fn from(err: ValidationErrorsKind) -> Self {
        match err {
            ValidationErrorsKind::Field(errs) => Errors(
                errs.into_iter()
                    .map(|err| {
                        err.message
                            .unwrap_or_else(|| "Unkown validation error".into())
                    })
                    .collect(),
            ),
            ValidationErrorsKind::List(_errs) => todo!(),
            ValidationErrorsKind::Struct(_errs) => todo!(),
        }
    }
}

impl<S, T> FromRequest<S> for Validated<Json<T>>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Problem<ErrorResponse>;

    async fn from_request(r: Request, s: &S) -> Result<Self, Self::Rejection> {
        let json = Json::<T>::from_request(r, s).await.map_err(unparseable)?;
        validate(&json.0)?;
        Ok(Validated(json))
    }
}

impl<S, T> FromRequest<S> for Validated<Form<T>>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Problem<ErrorResponse>;

    async fn from_request(r: Request, s: &S) -> Result<Self, Self::Rejection> {
        let form = Form::<T>::from_request(r, s).await.map_err(unparseable)?;
        validate(&form.0)?;
        Ok(Validated(form))
    }
}

/// Validates the given data and returns a handler result.
#[inline]
fn validate<T: Validate>(data: T) -> HandlerResult<(), ErrorResponse> {
    data.validate().map_err(|err| {
        Problem::new(
            StatusCode::BAD_REQUEST,
            "Validation failed",
            "Couldn't validate request body.",
        )
        .with_extension(ErrorResponse::from(err))
    })
}

/// Returns a `HandlerError` for a unparseable requests.
#[inline]
fn unparseable<T: std::error::Error>(_: T) -> Problem<ErrorResponse> {
    Problem::new(
        StatusCode::BAD_REQUEST,
        "Unparseable request",
        "Couldn't parse request body.",
    )
}
