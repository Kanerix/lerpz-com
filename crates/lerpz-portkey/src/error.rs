use serde_json::Value;

/// Collapses an upstream provider error into a single human-readable line.
///
/// Portkey passes provider errors straight through, and upstreams don't share a
/// schema. This handles the shapes we've observed in practice:
///
/// - a flat `{ "message": "..." }`
/// - a nested `{ "error": { "message": "..." } }`
/// - a raw provider HTML page wrapped in `{ "html-message": "<!DOCTYPE html>..." }`
///   (e.g. a Google/Vertex 404), reduced to its `<title>`
///
/// `async-openai` additionally prefixes the body with text such as `Invalid
/// response received from <provider>: `, so we locate the embedded JSON object
/// before parsing. Anything we can't confidently parse is returned unchanged.
pub fn humanize_error(raw: &str) -> String {
    let Some(start) = raw.find('{') else {
        return raw.to_string();
    };

    let Ok(value) = serde_json::from_str::<Value>(&raw[start..]) else {
        return raw.to_string();
    };

    if let Some(message) = value
        .get("message")
        .or_else(|| value.pointer("/error/message"))
        .and_then(Value::as_str)
    {
        return message.to_string();
    }

    if let Some(html) = value.get("html-message").and_then(Value::as_str) {
        return summarize_html_error(html);
    }

    raw.to_string()
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
    use super::humanize_error;

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
}
