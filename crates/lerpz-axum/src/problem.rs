//! Error module for endpoint handlers.
//!
//! This module follows the
//! [Problem Details for HTTP APIs](https://datatracker.ietf.org/doc/html/rfc9457)
//! specification.

use std::{borrow::Cow, panic::Location};

use axum::{
    Json,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "sse")]
use axum::response::sse::Event;

/// The body sent when a [`Problem`] cannot be serialized into an SSE event.
///
/// Only a custom extension can fail to serialize, so this keeps every field
/// that does not depend on one. The [`log_id`](ProblemInner::log_id) matters
/// most, since it is what ties the event to its log line.
#[cfg(feature = "sse")]
#[derive(Serialize)]
struct ProblemWithoutExtension<'a> {
    #[serde(rename = "type")]
    kind: &'a str,
    title: &'a str,
    detail: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_id: Option<&'a str>,
}

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
    /// logs without leaking sensitive details. Set on every server error, and
    /// on client errors that carry a source error. Omitted from the body when
    /// [`None`].
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
    /// Where in the source this problem was constructed.
    ///
    /// Not part of RFC 9457 and never serialized. Captured by the constructors
    /// via `#[track_caller]` and logged alongside the source error, so a log
    /// line points at the code that produced it. It is [`None`] for problems
    /// built by the [`From`] impl, since the `?` operator reaches it through
    /// [`std::ops::FromResidual`], which does not forward the caller location.
    #[serde(skip)]
    location: Option<&'static Location<'static>>,
}

impl<D> Problem<D>
where
    D: Serialize + Send + Sync,
{
    /// Create a new [`Problem`] with status code, title and detail.
    ///
    /// All optional fields are [`None`] by default. These can be set using
    /// methods found on the struct.
    #[track_caller]
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
                location: Some(Location::caller()),
            }),
        }
    }

    /// Create a new [`Problem`] with status code, title and detail, and
    /// fill the [`Self::instance`] field with the path of the request.
    ///
    /// This is a convenience method to create an error that is specific to a
    /// request, so that the client can see which endpoint the problem occurred
    /// on in the error data.
    #[track_caller]
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
    #[track_caller]
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
    #[track_caller]
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
    /// set a custom value, otherwise one is generated automatically when the
    /// problem is turned into a response.
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
    /// The log ID is set automatically when turned into a response, for every
    /// server error and for any problem carrying a source error, unless one has
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

    /// Where in the source this problem was constructed.
    ///
    /// [`None`] for a problem produced by the `?` operator, which reaches the
    /// [`From`] impl through [`std::ops::FromResidual`] and so cannot forward
    /// the caller location.
    pub fn location(&self) -> Option<&'static Location<'static>> {
        self.problem.location
    }

    /// Turns the [`Problem`] into a Server-Sent Events `error` event.
    ///
    /// Use this for a failure that happens once the response has already
    /// started, where the status line is long gone and a [`Response`] is no
    /// longer an option. The event carries the same JSON body that
    /// [`Problem::into_response`] would have sent, so a client can render it
    /// the same way.
    ///
    /// The problem is logged and assigned its log ID here, as described on
    /// [`ProblemInner::record`].
    ///
    /// ### Note
    ///
    /// Two fields do not survive the trip. The
    /// [`status`](Problem::status) is never part of the body, and an event has
    /// no status line to carry it, so a client has to tell the failures apart
    /// by their [`title`](Problem::title). The
    /// [`instance`](Problem::instance) is usually empty, because a stream body
    /// is polled after the handler has returned, by which point the task local
    /// holding the request path is out of scope. The log ID is what ties the
    /// event to its log line.
    #[cfg(feature = "sse")]
    pub fn into_event(mut self) -> Event {
        self.problem.fill_instance_from_request();
        self.problem.record();

        let problem = self.problem.as_ref();

        Event::default()
            .event("error")
            .json_data(problem)
            .unwrap_or_else(|err| {
                tracing::error!(
                    error = %err,
                    log_id = problem.log_id.as_deref(),
                    "cannot serialize a problem into an sse event"
                );
                Event::default()
                    .event("error")
                    .json_data(ProblemWithoutExtension {
                        kind: &problem.kind,
                        title: &problem.title,
                        detail: &problem.detail,
                        instance: problem.instance.as_deref(),
                        log_id: problem.log_id.as_deref(),
                    })
                    .expect("a problem without its extension always serializes")
            })
    }
}

impl<D> ProblemInner<D>
where
    D: Serialize + Send + Sync,
{
    /// Fills [`Self::instance`] with the path of the request being handled.
    ///
    /// Does nothing when an instance is already set, or when the task local is
    /// out of scope because the problem is built outside the handler.
    fn fill_instance_from_request(&mut self) {
        if self.instance.is_some() {
            return;
        }

        let _ = REQUEST_INSTANCE.try_with(|path| {
            self.instance = Some(Cow::Owned(path.clone()));
        });
    }

    /// Assigns a log ID and writes the log line for this occurrence.
    ///
    /// Every server error is recorded, whether or not a source error was
    /// attached, so that any 5xx a client receives can be found again by its
    /// log ID. Client errors are recorded only when they carry a source error,
    /// since the rest are expected and already described by their own fields.
    fn record(&mut self) {
        let is_server_error = self.status.is_server_error();
        if !is_server_error && self.inner.is_none() {
            return;
        }

        let log_id = self
            .log_id
            .get_or_insert_with(|| Uuid::new_v4().to_string());

        if is_server_error {
            // The alternate form walks the whole cause chain, where the default
            // one stops at the outermost error.
            let source = self.inner.as_ref().map(|err| format!("{err:#}"));
            tracing::error!(
                instance = self.instance.as_deref(),
                log_id = %log_id,
                location = self.location.map(|location| location.to_string()),
                title = %self.title,
                detail = %self.detail,
                server_error = source,
                "responding with a server error"
            );
        } else {
            tracing::info!(
                instance = self.instance.as_deref(),
                log_id = %log_id,
                client_error = %self.title,
                message = %self.detail,
                "responding with a client error"
            );
        }
    }
}

impl<D> IntoResponse for Problem<D>
where
    D: Serialize + Send + Sync,
{
    /// Converts a [`Problem`] into a [`Response`].
    ///
    /// Logs the problem and assigns its log ID on the way out, as described on
    /// [`ProblemInner::record`].
    fn into_response(mut self) -> Response {
        let problem = self.problem.as_mut();

        problem.fill_instance_from_request();
        problem.record();

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
            location: None,
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
    /// can be located in server logs. Always set on a server error.
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

        let error_detail = handler_error
            .extension()
            .expect("extension was attached by the builder");

        assert_eq!(error_detail.field, extension.field);

        let response = handler_error.into_response();

        assert!(response.status().is_client_error());
    }

    #[test]
    fn test_error_to_handler_result() {
        let example_handler = || -> HandlerResult<i32> { Ok("abc".parse::<i32>()?) };

        let handler_error = example_handler().expect_err("\"abc\" is not a valid i32");

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
            .expect_err("\"abc\" is not a valid i32")
            .with_log_id("example_log_id");
        let handler_error_two = example_handler_two()
            .expect_err("\"xyz\" is not a valid f64")
            .with_log_id("example_log_id");
        let handler_error_three = example_handler_three().expect_err("\"qwe\" is not a valid i16");

        assert!(handler_error_one.log_id().is_some());
        assert!(handler_error_two.log_id().is_some());
        assert!(handler_error_three.log_id().is_none()); // `log_id` is set when turned into a response.
        assert_eq!(handler_error_one.log_id(), handler_error_two.log_id())
    }

    #[test]
    fn server_error_is_recorded_without_a_source() {
        let mut problem = Problem::<()>::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            "Something went wrong",
        );

        assert!(problem.log_id().is_none());

        problem.problem.record();

        assert!(
            problem.log_id().is_some(),
            "a 5xx must be traceable even when no source error was attached"
        );
    }

    #[test]
    fn client_error_is_not_recorded_without_a_source() {
        let mut problem = Problem::<()>::new(StatusCode::NOT_FOUND, "Not Found", "No such thing.");

        problem.problem.record();

        assert!(problem.log_id().is_none());
    }

    #[test]
    fn constructor_captures_the_call_site() {
        let problem = Problem::<()>::unauthorized();
        let location = problem
            .location()
            .expect("constructors capture the caller location");

        assert!(location.file().ends_with("problem.rs"));
    }

    #[test]
    fn question_mark_operator_captures_no_call_site() {
        let example_handler = || -> HandlerResult<i32> { Ok("abc".parse::<i32>()?) };
        let problem = example_handler().expect_err("\"abc\" is not a valid i32");

        assert!(problem.location().is_none());
    }

    #[test]
    #[cfg(feature = "sse")]
    fn event_carries_the_problem_and_its_log_id() {
        let problem = Problem::<()>::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            "Something went wrong",
        )
        .with_error(Error::Random);

        let event = format!("{:?}", problem.into_event());

        assert!(event.contains("event: error"));
        assert!(event.contains(r#"\"title\":\"Internal Server Error\""#));
        assert!(event.contains("log_id"));
        assert!(
            !event.contains("random error"),
            "the source error must never reach the client"
        );
    }
}
