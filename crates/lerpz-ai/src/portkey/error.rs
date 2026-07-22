use serde_json::Value;

/// What (broadly) caused an upstream provider error.
///
/// This drives how we treat the error: a request the user themselves triggered
/// (e.g. a prompt rejected by content moderation) is expected and shouldn't be
/// logged as a server fault, whereas a provider- or transport-side failure
/// should.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Caused by the end user's input.
    User,
    /// A provider- or transport-side failure.
    Provider,
}

/// A classified, human-readable upstream provider error.
#[derive(Debug, Clone)]
pub struct UpstreamError {
    /// What broadly caused the error.
    pub kind: ErrorKind,
    /// A single-line, user-safe message.
    pub message: String,
}

impl UpstreamError {
    /// Whether the error was caused by the user's input (e.g. moderation).
    #[inline]
    pub fn is_user(&self) -> bool {
        self.kind == ErrorKind::User
    }

    /// Whether the error was caused by a provider-side failure.
    #[inline]
    pub fn is_provider(&self) -> bool {
        self.kind == ErrorKind::Provider
    }

    /// Creates a provider error with the given message.
    #[inline]
    pub fn provider(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: ErrorKind::Provider,
        }
    }

    /// Creates a provider error with the given message.
    #[inline]
    fn new_provider(message: impl Into<String>) -> Self {
        Self::provider(message)
    }
}

/// Collapses an upstream provider error into a single human-readable line.
///
/// This is a convenience wrapper around [`classify_error`] for callers that
/// only need the message. See [`classify_error`] for details on the shapes we
/// handle.
pub fn humanize_error(raw: &str) -> String {
    classify_error(raw).message
}

/// Extracts and classifies an upstream provider error.
///
/// Portkey passes provider errors straight through, and upstreams don't share a
/// schema. This handles the shapes we've observed in practice:
///
/// - a flat `{ "message": "..." }`
/// - a nested `{ "error": { "message": "..." } }`
/// - a streamed error event `{ "type": "error", "error": { "message": "...",
///   "code": "moderation_blocked" } }` (surfaced by `async-openai` as an opaque
///   "failed to deserialize api response" error, so we look for the embedded
///   JSON object rather than relying on the outer text)
/// - a raw provider HTML page wrapped in `{ "html-message": "<!DOCTYPE html>..." }`
///   (e.g. a Google/Vertex 404), reduced to its `<title>`
///
/// `async-openai` additionally prefixes the body with text such as `Invalid
/// response received from <provider>: `, so we locate the embedded JSON object
/// before parsing. Anything we can't confidently parse is returned unchanged and
/// treated as a provider-side error.
pub fn classify_error(raw: &str) -> UpstreamError {
    let Some(start) = raw.find('{') else {
        return UpstreamError::new_provider(raw);
    };

    let Ok(value) = serde_json::from_str::<Value>(&raw[start..]) else {
        return UpstreamError::new_provider(raw);
    };

    if let Some(message) = value
        .get("message")
        .or_else(|| value.pointer("/error/message"))
        .and_then(Value::as_str)
    {
        return UpstreamError {
            message: message.to_string(),
            kind: classify_kind(&value),
        };
    }

    if let Some(html) = value.get("html-message").and_then(Value::as_str) {
        return UpstreamError::new_provider(summarize_html_error(html));
    }

    UpstreamError::new_provider(raw)
}

/// Decides whether a parsed error payload was caused by the user's input.
///
/// We treat an error as user-caused when the provider's error `type` names the
/// user (e.g. `image_generation_user_error`) or the `code` is a known
/// moderation / content-policy rejection.
fn classify_kind(value: &Value) -> ErrorKind {
    let type_is_user = value
        .pointer("/error/type")
        .or_else(|| value.get("type"))
        .and_then(Value::as_str)
        .is_some_and(|t| t.contains("user_error"));

    let code_is_user = value
        .pointer("/error/code")
        .or_else(|| value.get("code"))
        .and_then(Value::as_str)
        .is_some_and(is_user_error_code);

    if type_is_user || code_is_user {
        ErrorKind::User
    } else {
        ErrorKind::Provider
    }
}

/// Whether an error `code` denotes a user-caused rejection (moderation, content
/// policy, or an otherwise invalid prompt) rather than a provider fault.
fn is_user_error_code(code: &str) -> bool {
    matches!(
        code,
        "moderation_blocked"
            | "content_policy_violation"
            | "content_filter"
            | "content_filtered"
            | "invalid_prompt"
    )
}

/// Reduces a provider HTML error page to its `<title>` (e.g. `Error 404 (Not Found)`),
/// falling back to a generic message when no title is present.
fn summarize_html_error(html: &str) -> String {
    let title = extract_between(html, "<title>", "</title>")
        .map(|title| title.trim().trim_end_matches("!!1").trim())
        .filter(|title| !title.is_empty());

    match title {
        Some(title) => format!("The provider returned an error page: {title}."),
        None => "The provider returned an unexpected HTML error page.".to_string(),
    }
}

#[inline]
fn extract_between<'a>(haystack: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = haystack.find(open)? + open.len();
    let end = haystack[start..].find(close)? + start;
    Some(&haystack[start..end])
}

#[cfg(test)]
mod tests {
    use super::{ErrorKind, classify_error, humanize_error};

    #[test]
    fn extracts_flat_message_from_prefixed_body() {
        let raw = r#"Invalid response received from vertex-ai: {"status":"failure","message":"imageGenerate is not supported by vertex-ai"}"#;
        assert_eq!(
            humanize_error(raw),
            "imageGenerate is not supported by vertex-ai"
        );
    }

    #[test]
    fn extracts_nested_error_message() {
        let raw = r#"{"error":{"message":"model not found","type":"invalid_request_error"}}"#;
        assert_eq!(humanize_error(raw), "model not found");
    }

    #[test]
    fn summarizes_html_error_page() {
        let raw = r#"{"html-message":"<!DOCTYPE html>\n<html lang=en>\n  <title>Error 404 (Not Found)!!1</title>\n  <p><b>404.</b></html>"}"#;
        assert_eq!(
            humanize_error(raw),
            "The provider returned an error page: Error 404 (Not Found)."
        );
    }

    #[test]
    fn falls_back_for_html_without_title() {
        let raw = r#"{"html-message":"<!DOCTYPE html><html><body>boom</body></html>"}"#;
        assert_eq!(
            humanize_error(raw),
            "The provider returned an unexpected HTML error page."
        );
    }

    #[test]
    fn returns_plain_text_unchanged() {
        let raw = "connection reset by peer";
        assert_eq!(humanize_error(raw), "connection reset by peer");
    }

    #[test]
    fn returns_input_when_json_is_malformed() {
        let raw = "unexpected: {not valid json";
        assert_eq!(humanize_error(raw), raw);
    }

    #[test]
    fn extracts_moderation_message_from_stream_error_event() {
        let raw = r#"failed to deserialize api response: error:unknown variant `error`, expected `image_generation.partial_image` or `image_generation.completed` at line 1 column 15 content:{"type":"error","error":{"type":"image_generation_user_error","code":"moderation_blocked","message":"Your request was rejected by the safety system.","param":null},"sequence_number":0}"#;
        let err = classify_error(raw);
        assert_eq!(
            err.message,
            "Your request was rejected by the safety system."
        );
        assert_eq!(err.kind, ErrorKind::User);
        assert!(err.is_user());
    }

    #[test]
    fn classifies_moderation_code_as_user_error() {
        let raw = r#"{"error":{"message":"blocked","code":"content_policy_violation"}}"#;
        assert_eq!(classify_error(raw).kind, ErrorKind::User);
    }

    #[test]
    fn classifies_unknown_provider_error_as_provider() {
        let raw = r#"{"error":{"message":"upstream timeout","type":"server_error"}}"#;
        assert_eq!(classify_error(raw).kind, ErrorKind::Provider);
    }

    #[test]
    fn classifies_plain_text_as_provider() {
        assert_eq!(classify_error("connection reset").kind, ErrorKind::Provider);
    }
}
