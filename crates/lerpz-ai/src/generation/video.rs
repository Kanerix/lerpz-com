//! Default video generation.
//!
//! Video providers render asynchronously: a request creates a long-running
//! operation that is polled until it completes. [`start`] kicks off the job and
//! [`VideoJob::poll`] streams it to completion, downloading the finished asset
//! when the provider only exposes a link.

use std::time::Duration;

use async_openai::{Client, config::Config};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use http::HeaderMap;
use serde_json::{Value, json};

use crate::{
    generation::VideoStream,
    portkey::{UpstreamError, classify_error},
};

/// How long to wait between polls of the long-running generation operation.
const POLL_INTERVAL: Duration = Duration::from_secs(10);
/// Upper bound on the number of polls before giving up, so a stuck operation
/// can't keep a stream open forever. At [`POLL_INTERVAL`] this is ~15 minutes,
/// comfortably above Veo's typical render time.
const MAX_POLL_ATTEMPTS: usize = 90;
/// Container format Veo renders to.
const VIDEO_FORMAT: &str = "mp4";

/// Parameters for a video generation request.
#[derive(Debug, Clone)]
pub struct VideoRequest {
    /// Prompt describing the desired video.
    pub prompt: String,
    /// Model (deployment name) to route the request to.
    pub model: String,
    /// Desired aspect ratio, e.g. `16:9` (landscape) or `9:16` (portrait).
    pub aspect_ratio: Option<String>,
    /// Clip length in seconds. Clamped to the range the provider supports.
    pub duration: Option<u16>,
}

/// An event emitted while a video generation job completes.
#[derive(Debug, Clone)]
pub enum VideoEvent {
    /// The finished video, ready to persist.
    Completed {
        /// The decoded video bytes.
        bytes: Vec<u8>,
        /// Container format (e.g. `mp4`).
        format: String,
        /// Output width in pixels.
        width: u32,
        /// Output height in pixels.
        height: u32,
        /// Clip length in seconds.
        duration: u32,
    },
    /// The provider only exposed a link and the asset could not be downloaded,
    /// so it cannot be persisted but can still be played by the caller.
    Link {
        /// A URL the finished video can be played from.
        url: String,
    },
}

/// A created, long-running video generation operation awaiting completion.
pub struct VideoJob {
    http: reqwest::Client,
    headers: HeaderMap,
    operation_url: String,
    /// The provider's operation identifier, useful for logging.
    pub operation_name: String,
    width: u32,
    height: u32,
    duration: u32,
}

/// A video handed back by a completed operation: either an inline (base64)
/// payload or a URL we still have to fetch.
enum VideoPayload {
    /// The video bytes, already decoded.
    Bytes(Vec<u8>),
    /// A URL the video must be downloaded from.
    Uri(String),
}

/// Creates a video generation job.
pub(super) async fn start<C: Config>(
    client: &Client<C>,
    request: VideoRequest,
) -> Result<VideoJob, UpstreamError> {
    let config = client.config();
    let headers = config.headers();
    let generate_url = config.url("/videos:generate");
    let http = reqwest::Client::new();

    let (aspect_ratio, width, height) = resolve_aspect_ratio(request.aspect_ratio.as_deref());
    let duration = resolve_duration(request.duration);

    let body = json!({
        "model": request.model,
        "prompt": request.prompt,
        "aspectRatio": aspect_ratio,
    });

    tracing::debug!(model = %request.model, "creating video generation job");

    let response = http
        .post(&generate_url)
        .headers(headers.clone())
        .json(&body)
        .send()
        .await
        .map_err(|_| UpstreamError::provider("Failed to reach the video generation provider."))?;

    // Portkey passes upstream provider errors straight through, often in a
    // non-OpenAI shape. Surface the classified upstream message instead of an
    // opaque failure.
    let status = response.status();
    if !status.is_success() {
        let text = response.text().await.unwrap_or_default();
        return Err(classify_error(&text));
    }

    let job: Value = response
        .json()
        .await
        .map_err(|_| UpstreamError::provider("The video provider returned an unexpected response."))?;

    let operation_name = extract_operation_name(&job)
        .ok_or_else(|| {
            UpstreamError::provider("The video provider did not return an operation to poll.")
        })?
        .to_string();

    let operation_url = config.url(&operation_path(&operation_name));

    tracing::debug!(%operation_name, model = %request.model, "video generation job created");

    Ok(VideoJob {
        http,
        headers,
        operation_url,
        operation_name,
        width,
        height,
        duration,
    })
}

impl VideoJob {
    /// Polls the operation until the video is ready, then yields it.
    pub fn poll(self) -> VideoStream {
        Box::pin(async_stream::stream! {
            tracing::trace!(operation_name = %self.operation_name, "starting video generation poll");
            let mut attempts = 0usize;

            loop {
                tokio::time::sleep(POLL_INTERVAL).await;

                attempts += 1;
                if attempts > MAX_POLL_ATTEMPTS {
                    tracing::error!(operation_name = %self.operation_name, "video generation timed out");
                    yield Err(UpstreamError::provider("Video generation timed out."));
                    break;
                }

                let poll = match self.http.get(&self.operation_url).headers(self.headers.clone()).send().await {
                    Ok(poll) => poll,
                    Err(err) => {
                        yield Err(classify_error(&err.to_string()));
                        break;
                    }
                };

                let status = poll.status();
                if !status.is_success() {
                    let text = poll.text().await.unwrap_or_default();
                    yield Err(classify_error(&text));
                    break;
                }

                let operation: Value = match poll.json().await {
                    Ok(operation) => operation,
                    Err(_) => {
                        yield Err(UpstreamError::provider("The video provider returned an unexpected response."));
                        break;
                    }
                };

                if !operation.get("done").and_then(Value::as_bool).unwrap_or(false) {
                    tracing::trace!(operation_name = %self.operation_name, attempts, "video generation in progress");
                    continue;
                }

                if let Some(error) = operation.get("error") {
                    yield Err(classify_error(&error.to_string()));
                    break;
                }

                let bytes = match extract_video(&operation) {
                    Some(VideoPayload::Bytes(bytes)) => bytes,
                    Some(VideoPayload::Uri(uri)) => {
                        tracing::trace!(operation_name = %self.operation_name, "downloading completed video");
                        match download_video(&self.http, &self.headers, &uri).await {
                            Ok(bytes) => bytes,
                            Err(err) => {
                                // We couldn't fetch the bytes to persist, but the
                                // provider still gave us a playable link. Hand it
                                // to the caller so the render isn't lost.
                                tracing::error!(operation_name = %self.operation_name, "failed to download video: {err}");
                                yield Ok(VideoEvent::Link { url: uri });
                                break;
                            }
                        }
                    }
                    None => {
                        tracing::error!(operation_name = %self.operation_name, response = %operation, "completed operation had no video");
                        yield Err(UpstreamError::provider("The video provider returned no video."));
                        break;
                    }
                };

                tracing::trace!(operation_name = %self.operation_name, bytes = bytes.len(), "obtained completed video");

                yield Ok(VideoEvent::Completed {
                    bytes,
                    format: VIDEO_FORMAT.to_string(),
                    width: self.width,
                    height: self.height,
                    duration: self.duration,
                });
                break;
            }
        })
    }
}

/// Map a requested aspect ratio to the value the provider expects plus the
/// matching output pixel dimensions.
fn resolve_aspect_ratio(aspect_ratio: Option<&str>) -> (&'static str, u32, u32) {
    match aspect_ratio.map(str::trim) {
        Some("16:9") | Some("landscape") => ("16:9", 1280, 720),
        _ => ("9:16", 720, 1280),
    }
}

/// Clamp a requested duration to the range the provider supports.
fn resolve_duration(duration: Option<u16>) -> u32 {
    duration.unwrap_or(8).clamp(4, 8) as u32
}

/// Pull the operation identifier out of a `POST /videos:generate` response.
///
/// Providers name the field inconsistently, so try the shapes we've seen.
fn extract_operation_name(value: &Value) -> Option<&str> {
    ["name", "operationName", "operation", "id"]
        .into_iter()
        .find_map(|key| value.get(key).and_then(Value::as_str))
}

/// Turn an operation name into a poll path.
///
/// The provider may return a bare id, a fully-qualified `operations/{id}`, or a
/// longer resource path; only prefix `operations/` when it isn't already part
/// of the name.
fn operation_path(name: &str) -> String {
    let trimmed = name.trim_start_matches('/');
    if trimmed.starts_with("operations/") || trimmed.contains("/operations/") {
        format!("/{trimmed}")
    } else {
        format!("/operations/{trimmed}")
    }
}

/// Locate the video object inside a completed operation's `response`.
///
/// Veo (and its various gateways) nest the generated video differently
/// depending on API surface, so probe the shapes we've observed.
fn find_video_object(operation: &Value) -> Option<&Value> {
    [
        "/response/generatedVideos/0/video",
        "/response/generatedVideos/video",
        "/response/generateVideoResponse/generatedSamples/0/video",
        "/response/generatedSamples/0/video",
        "/response/videos/0",
        "/generatedVideos/0/video",
    ]
    .into_iter()
    .find_map(|pointer| operation.pointer(pointer))
}

/// Extract a playable video from a completed operation.
///
/// Prefers inline base64 bytes (no extra round-trip) and falls back to a URL
/// the caller must download.
fn extract_video(operation: &Value) -> Option<VideoPayload> {
    let video = find_video_object(operation)?;

    for key in ["videoBytes", "bytesBase64Encoded", "b64_json", "data"] {
        if let Some(encoded) = video.get(key).and_then(Value::as_str)
            && let Ok(bytes) = BASE64.decode(encoded)
        {
            return Some(VideoPayload::Bytes(bytes));
        }
    }

    for key in ["uri", "url", "downloadUri", "videoUri"] {
        if let Some(uri) = video.get(key).and_then(Value::as_str) {
            return Some(VideoPayload::Uri(uri.to_string()));
        }
    }

    None
}

/// Download the finished video from the URL the provider returned.
///
/// The gateway headers are forwarded in case the asset lives behind the same
/// authenticated gateway.
async fn download_video(
    http: &reqwest::Client,
    headers: &HeaderMap,
    uri: &str,
) -> Result<Vec<u8>, String> {
    let response = http
        .get(uri)
        .headers(headers.clone())
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!(
            "download failed ({status}): {}",
            crate::portkey::humanize_error(&body)
        ));
    }

    Ok(response
        .bytes()
        .await
        .map_err(|err| err.to_string())?
        .to_vec())
}
