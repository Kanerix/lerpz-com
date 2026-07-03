//! Error module for endpoint handlers.
//!
//! This module follows the
//! [Problem Details for HTTP APIs](https://datatracker.ietf.org/doc/html/rfc9457)
//! specification.

use std::borrow::Cow;

use axum::{
    Json,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

tokio::task_local! {
    /// The path of the request currently being handled.
    ///
    /// Set by [`crate::middleware::instance::capture_instance`] for the
    /// duration of each request, and read in [`Problem::into_response`] to
    /// automatically populate the [`instance`](Problem::instance) field on any
    /// problem that does not already have one (e.g. problems created via the
    /// `?` operator).
    pub(crate) static REQUEST_INSTANCE: String;
}

/// A type alias for [`Result<T, Problem>`].
///
/// Used by handlers to return a response or a structured error (a problem).
pub type HandlerResult<T, D = ()> = std::result::Result<T, Problem<D>>;

/// Represents a problem returned by a handler.
#[derive(Debug)]
pub struct Problem<D = ()>
where
    D: Serialize + Send + Sync,
{
    problem: Box<ProblemInner<D>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProblemInner<D = ()>
where
    D: Serialize + Send + Sync,
{
    /// The HTTP status code for this problem.
    ///
    /// Corresponds to the `status` member in [RFC 9457 §3.1]. It is not
    /// serialized into the body, as it is instead conveyed by the HTTP
    /// response status line (see [`Problem::into_response`]).
    ///
    /// [RFC 9457 §3.1]: https://datatracker.ietf.org/doc/html/rfc9457#section-3.1
    #[serde(skip)]
    status: StatusCode,
    /// A URI reference identifying the problem type.
    ///
    /// The `type` member from [RFC 9457 §3.1]. Renamed to `type` on the wire,
    /// since `type` is a reserved keyword in Rust. Defaults to `about:blank`
    /// when the problem has no dedicated documentation.
    ///
    /// [RFC 9457 §3.1]: https://datatracker.ietf.org/doc/html/rfc9457#section-3.1
    #[serde(rename = "type")]
    kind: Cow<'static, str>,
    /// A short, human-readable summary of the problem type.
    ///
    /// The `title` member from [RFC 9457 §3.1]. It should stay stable across
    /// occurrences of the same problem type (unlike [`Self::detail`]).
    ///
    /// [RFC 9457 §3.1]: https://datatracker.ietf.org/doc/html/rfc9457#section-3.1
    title: Cow<'static, str>,
    /// A human-readable explanation specific to this occurrence.
    ///
    /// The `detail` member from [RFC 9457 §3.1]. Unlike [`Self::title`], it may
    /// vary between occurrences of the same problem type.
    ///
    /// [RFC 9457 §3.1]: https://datatracker.ietf.org/doc/html/rfc9457#section-3.1
    detail: Cow<'static, str>,
    /// A URI reference identifying this specific occurrence of the problem.
    ///
    /// The `instance` member from [RFC 9457 §3.1]. Usually the path of the
    /// request that failed; it is filled automatically from the request when
    /// not set explicitly. Omitted from the body when [`None`].
    ///
    /// [RFC 9457 §3.1]: https://datatracker.ietf.org/doc/html/rfc9457#section-3.1
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<Cow<'static, str>>,
    /// Additional structured data specific to this problem type.
    ///
    /// A problem type extension member, as described in [RFC 9457 §3.2].
    /// Serialized under an `extension` key rather than as top-level members.
    /// Omitted from the body when [`None`].
    ///
    /// [RFC 9457 §3.2]: https://datatracker.ietf.org/doc/html/rfc9457#section-3.2
    #[serde(skip_serializing_if = "Option::is_none")]
    extension: Option<D>,
    /// A server-side log reference for this occurrence.
    ///
    /// An extension member (in the sense of [RFC 9457 §3.2]) that is not part
    /// of the standard fields. It is sent to the client in place of the actual
    /// source error so that support requests can be correlated with server
    /// logs without leaking sensitive details. Omitted from the body when [`None`].
    ///
    /// [RFC 9457 §3.2]: https://datatracker.ietf.org/doc/html/rfc9457#section-3.2
    #[serde(skip_serializing_if = "Option::is_none")]
    log_id: Option<String>,
    /// The underlying source error, if any.
    ///
    /// Not part of RFC 9457 and never serialized: exposing internal error
    /// details to clients could leak sensitive information. It is used only for
    /// server-side logging (see [`Problem::into_response`]) and is surfaced to
    /// the client indirectly through [`Self::log_id`].
    #[serde(skip)]
    inner: Option<anyhow::Error>,
}

impl<D> Problem<D>
where
    D: Serialize + Send + Sync,
{
    /// Create a new [`Problem`] with status code, title and detail.
    ///
    /// All optional fields are [`None`] by default. These can be set using
    /// methods found on the struct.
    pub fn new(
        status: StatusCode,
        title: impl Into<Cow<'static, str>>,
        detail: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            problem: Box::new(ProblemInner {
                status,
                kind: Cow::from("about:blank"),
                title: title.into(),
                detail: detail.into(),
                instance: None,
                extension: None,
                log_id: None,
                inner: None,
            }),
        }
    }

    /// Create a new [`Problem`] with status code, title and detail, and
    /// fill the [`Self::instance`] field with the path of the request.
    ///
    /// This is a convenience method to create an error that is specific to a
    /// request, so that the client can see which endpoint the problem occurred
    /// on in the error data.
    pub fn new_with_parts(
        status: StatusCode,
        title: impl Into<Cow<'static, str>>,
        detail: impl Into<Cow<'static, str>>,
        p: &Parts,
    ) -> Self {
        Self::new(status, title, detail).fill_instance(p)
    }

    /// A generic unauthorized response.
    ///
    /// This is a generic response for someone that tries to access an
    /// authorized resource without proper authorization.
    pub fn unauthorized() -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            Cow::from("Unauthorized"),
            Cow::from("You are not authorized to access this resource."),
        )
    }

    /// A generic forbidden response.
    ///
    /// This is a generic response for someone that tries to access a forbidden
    /// resource, even though they are authorized.
    pub fn forbidden() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            Cow::from("Forbidden"),
            Cow::from("You do not have permission to access this resource."),
        )
    }

    /// HTTP status code generated by the server for this specific problem.
    pub fn status(&self) -> StatusCode {
        self.problem.status
    }

    /// A URI reference that identifies the problem type.
    ///
    /// This is dereferenced to human-readable documentation for the problem
    /// type. This will be `about::blank` if the error does not have any
    /// documentation.
    pub fn kind(&self) -> &str {
        &self.problem.kind
    }

    /// A short human-readable error summary.
    ///
    /// Short and precise text that gives an indication of what the error is
    /// about. This should not change between occurrences.
    pub fn title(&self) -> &str {
        &self.problem.title
    }

    /// A human-readable detailed error explanation.
    ///
    /// A more detailed description of what went wrong. This is unlike `title`,
    /// allowed to change between occurrences.
    pub fn detail(&self) -> &str {
        &self.problem.detail
    }

    /// A URI reference that is specific to the problem type.
    ///
    /// Does not get sent to the client if it's [`None`]. This is a unique
    /// identifier for the error. This will usually be the endpoint that the
    /// error occurred in.
    pub fn instance(&self) -> Option<&str> {
        self.problem.instance.as_deref()
    }

    /// Returns the extension if one was set.
    ///
    /// Only present on problems that carry extra context beyond the standard
    /// fields. The type must implement [`Serialize`] so that an OpenAPI schema
    /// can be generated for it.
    pub fn extension(&self) -> Option<&D> {
        self.problem.extension.as_ref()
    }

    /// Returns the log ID if one has been set.
    ///
    /// Sent to the client in place of the actual source error, so that
    /// sensitive error details are never leaked. Use [`Self::with_log_id`] to
    /// set a custom value, otherwise one is generated automatically when a
    /// source error is present.
    pub fn log_id(&self) -> Option<&str> {
        self.problem.log_id.as_deref()
    }

    /// Returns whether this problem has an associated source error.
    ///
    /// Source errors are never exposed to the client for security reasons.
    /// If this returns `true`, a log ID should also be present to identify
    /// the error in the logs.
    pub fn has_source(&self) -> bool {
        self.problem.inner.is_some()
    }

    /// Add a kind (also known as type) to the [`Problem`].
    pub fn with_kind(mut self, kind: impl Into<Cow<'static, str>>) -> Self {
        self.problem.kind = kind.into();
        self
    }

    /// Fills the [`Self::instance`] field with the path of the request.
    ///
    /// This is a convenience method that uses [`Parts`] of the request to
    /// fill the [`Self::instance`] field with the URI of the incoming request.
    pub fn fill_instance(self, p: &Parts) -> Self {
        self.with_instance(Cow::Owned(p.uri.path().into()))
    }

    /// Add an instance to the [`Problem`].
    pub fn with_instance(mut self, instance: impl Into<Cow<'static, str>>) -> Self {
        self.problem.instance = Some(instance.into());
        self
    }

    /// Add a custom extension to the [`Problem`].
    pub fn with_extension(mut self, extension: D) -> Self {
        self.problem.extension = Some(extension);
        self
    }

    /// Add a source error to the [`Problem`].
    pub fn with_error<E>(mut self, error: E) -> Self
    where
        E: Into<anyhow::Error>,
    {
        self.problem.inner = Some(error.into());
        self
    }

    /// Set the log ID for the [`Problem`].
    ///
    /// The log ID is automatically set when a source error is present (i.e.
    /// when [`.with_error()`](Self::with_error) was called) and no log ID has
    /// been set manually. Changing this might make it hard or impossible to
    /// track the error, or in other ways break how the error is logged.
    ///
    /// ### Note
    ///
    /// Make sure you use a globally unique identifier for the log ID.
    /// This will default to a UUID if it's missing when turned into a response.
    pub fn with_log_id<U>(mut self, log_id: U) -> Self
    where
        U: Into<String>,
    {
        self.problem.log_id = Some(log_id.into());
        self
    }
}

impl<D> IntoResponse for Problem<D>
where
    D: Serialize + Send + Sync,
{
    /// Converts a [`Problem`] into a [`Response`].
    ///
    /// This automatically logs errors using [`tracing`] and sets the
    /// log ID so that the error can be tracked.
    fn into_response(mut self) -> Response {
        let problem = self.problem.as_mut();

        if problem.instance.is_none() {
            let _ = REQUEST_INSTANCE.try_with(|path| {
                problem.instance = Some(Cow::Owned(path.clone()));
            });
        }

        if let Some(err) = problem.inner.as_ref() {
            let log_id = problem
                .log_id
                .get_or_insert_with(|| Uuid::new_v4().to_string());

            if problem.status.is_server_error() {
                tracing::error!(
                    instance = ?&problem.instance.as_deref(),
                    log_id = %log_id,
                    server_error = %err,
                    "A server error occurred"
                );
            } else {
                tracing::info!(
                    instance = ?&problem.instance.as_deref(),
                    log_id = %log_id,
                    client_error = %problem.title,
                    message = %problem.detail,
                    "A client error occurred"
                );
            }
        }

        (
            problem.status,
            [("Content-Type", "application/problem+json")],
            Json(self.problem),
        )
            .into_response()
    }
}

impl<D: Serialize + Send + Sync> Default for Problem<D> {
    /// Default implementation for [`Problem`].
    ///
    /// This is used when no specific error is available, and a generic error
    /// message is returned. It is assumed to be an internal server error.
    fn default() -> Self {
        Self {
            problem: Box::default(),
        }
    }
}

impl<D> Default for ProblemInner<D>
where
    D: Serialize + Send + Sync,
{
    fn default() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            kind: Cow::from("about:blank"),
            title: "Something went wrong".into(),
            detail: "If this issue persists, please contact an administrator.".into(),
            instance: None,
            extension: None,
            log_id: None,
            inner: None,
        }
    }
}

impl<E, D> From<E> for Problem<D>
where
    E: Into<anyhow::Error>,
    D: Serialize + Send + Sync,
{
    /// Turns any error into a [`Problem`].
    ///
    /// This uses the default implementation defined in [`Problem::default`].
    fn from(value: E) -> Self {
        Self {
            problem: Box::new(ProblemInner {
                inner: Some(value.into()),
                ..Default::default()
            }),
        }
    }
}

/// A structured error response following [RFC 9457 Problem Details](https://www.rfc-editor.org/rfc/rfc9457).
///
/// All error responses from this API use this shape with
/// `Content-Type: application/problem+json`.
///
/// ## Example
///
/// ```json
/// {
///   "type": "about:blank",
///   "title": "Unauthorized",
///   "detail": "You are not authorized to access this resource.",
///   "instance": "/api/v1/chats",
///   "log_id": "01948a62-f94e-7d36-b5ef-70a9b764b2e0"
/// }
/// ```
#[cfg(feature = "oapi")]
#[derive(utoipa::ToSchema)]
#[allow(dead_code)]
#[schema(title = "Problem")]
pub struct ProblemSchema<D = ()>
where
    D: utoipa::ToSchema,
{
    /// A URI that identifies the problem type.
    ///
    /// Dereferences to human-readable documentation when available.
    /// Defaults to `about:blank` when no specific documentation exists.
    #[schema(rename = "type", example = "about:blank")]
    kind: String,
    /// A short, stable summary of the problem type.
    ///
    /// Does not change between occurrences of the same error kind.
    #[schema(example = "Unauthorized")]
    title: String,
    /// A human-readable explanation of this specific occurrence of the problem.
    #[schema(example = "You are not authorized to access this resource.")]
    detail: String,
    /// The URI of the endpoint where the problem occurred.
    ///
    /// Included when the server can identify the specific resource that caused
    /// the error. Omitted otherwise.
    #[schema(nullable, example = "/api/v1/chats")]
    instance: Option<String>,
    /// Additional structured data specific to this error type.
    ///
    /// Only present on errors that carry extra context beyond the standard
    /// fields. Omitted on all standard error responses.
    #[schema(nullable, value_type = serde_json::Value)]
    extension: Option<D>,
    /// A server-side log reference for this error occurrence.
    ///
    /// When present, include this ID in any support request so the error
    /// can be located in server logs. Only set for unexpected server errors.
    #[schema(nullable, example = "01948a62-f94e-7d36-b5ef-70a9b764b2e0")]
    log_id: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Serialize, Clone)]
    struct Extension {
        field: String,
    }

    #[derive(thiserror::Error, Debug)]
    enum Error {
        #[error("this is a random error")]
        Random,
    }

    #[test]
    fn test_error_builder_pattern() {
        let extension = Extension {
            field: String::from("This is a random error."),
        };

        let handler_error: Problem<Extension> = Problem::new(
            StatusCode::BAD_REQUEST,
            "Bad Request",
            "Something went wrong",
        )
        .with_error(Error::Random)
        .with_extension(extension.clone());

        assert!(handler_error.has_source());
        assert!(handler_error.extension().is_some());
        assert!(handler_error.log_id().is_none()); // `log_id` is set when turned into a response.

        let error_detail = handler_error.extension().unwrap();

        assert_eq!(error_detail.field, extension.field);

        let response = handler_error.into_response();

        assert!(response.status().is_client_error());
    }

    #[test]
    fn test_error_to_handler_result() {
        let example_handler = || -> HandlerResult<i32> { Ok("abc".parse::<i32>()?) };

        let handler_error = example_handler().unwrap_err();

        assert!(handler_error.status().is_server_error());
        assert!(handler_error.has_source());
        assert!(handler_error.log_id().is_none()); // `log_id` is set when turned into a response.
    }

    #[test]
    fn test_with_log_id() {
        let example_handler_one = || -> HandlerResult<i32> { Ok("abc".parse::<i32>()?) };
        let example_handler_two = || -> HandlerResult<f64> { Ok("xyz".parse::<f64>()?) };
        let example_handler_three = || -> HandlerResult<i16> { Ok("qwe".parse::<i16>()?) };

        let handler_error_one = example_handler_one()
            .unwrap_err()
            .with_log_id("example_log_id");
        let handler_error_two = example_handler_two()
            .unwrap_err()
            .with_log_id("example_log_id");
        let handler_error_three = example_handler_three().unwrap_err();

        assert!(handler_error_one.log_id().is_some());
        assert!(handler_error_two.log_id().is_some());
        assert!(handler_error_three.log_id().is_none()); // `log_id` is set when turned into a response.
        assert_eq!(handler_error_one.log_id(), handler_error_two.log_id())
    }
}
