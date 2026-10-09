//! Portkey video generation and polling.
//!
//! Veo uses a different endpoint and poll method from the default video path.

use std::time::Duration;

use async_openai::{Client, config::Config};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use http::{HeaderMap, HeaderValue};
use serde_json::{Value, json};

use crate::{
    generation::{
        Family, UpstreamError, VideoEvent, VideoGeneration, VideoJob, VideoRequest, VideoStream,
        classify_error, humanize_error,
    },
    portkey::PortkeyConfig,
};

const POLL_INTERVAL: Duration = Duration::from_secs(20);
const MAX_POLL_ATTEMPTS: usize = 90;
const VIDEO_FORMAT: &str = "mp4";

impl VideoGeneration for Family {
    type Config = PortkeyConfig;

    async fn generate_video(
        &self,
        client: &Client<Self::Config>,
        request: VideoRequest,
    ) -> Result<VideoJob, UpstreamError> {
        let (aspect_ratio, width, height) = resolve_aspect_ratio(request.aspect_ratio.as_deref());
        let duration = resolve_duration(request.duration);
        let config = client.config();
        let submission = match self {
            Family::Default => default_request(config, request, aspect_ratio),
            Family::Google => veo_request(config, request, aspect_ratio, duration)?,
        };

        let (http, operation_name) = submit_job(
            &submission.headers,
            submission.generate_url,
            submission.body,
        )
        .await?;
        let (poll_url, poll_strategy) = match submission.poll_target {
            PollTarget::Operation => (
                config.url(&operation_path(&operation_name)),
                PollStrategy::Get,
            ),
            PollTarget::FetchPredict(url) => (url, PollStrategy::FetchPredict),
        };
        let poll = PollJob {
            http,
            headers: submission.headers,
            poll_url,
            poll_strategy,
            operation_name: operation_name.clone(),
            width,
            height,
            duration,
        };
        Ok(VideoJob::new(operation_name, poll.poll()))
    }
}

/// How a created operation is polled to completion.
///
/// OpenAI-style providers (e.g. Sora) expose the operation as a resource to
/// `GET`, whereas Vertex AI requires a `POST` to `:fetchPredictOperation` with
/// the operation name in the body.
enum PollStrategy {
    /// `GET` the operation resource at the stored URL.
    Get,
    /// `POST` `:fetchPredictOperation` with `{ operationName }`.
    FetchPredict,
}

struct VideoSubmission {
    headers: HeaderMap,
    generate_url: String,
    body: Value,
    poll_target: PollTarget,
}

enum PollTarget {
    Operation,
    FetchPredict(String),
}

/// A Portkey operation awaiting completion.
struct PollJob {
    http: reqwest::Client,
    headers: HeaderMap,
    poll_url: String,
    poll_strategy: PollStrategy,
    operation_name: String,
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

/// Builds the default (OpenAI-style) video submission.
///
/// Providers such as Azure's Sora accept `/videos:generate` with the model in
/// the request body.
fn default_request(
    config: &PortkeyConfig,
    request: VideoRequest,
    aspect_ratio: &str,
) -> VideoSubmission {
    let body = json!({
        "model": request.model,
        "prompt": request.prompt,
        "aspectRatio": aspect_ratio,
    });

    tracing::debug!(model = %request.model, "creating video generation job");

    VideoSubmission {
        headers: config.headers(),
        generate_url: config.url("/videos:generate"),
        body,
        poll_target: PollTarget::Operation,
    }
}

/// Builds a video submission for Google's Vertex AI (Veo).
///
/// Veo has no OpenAI-style `/videos:generate` endpoint; it uses Vertex AI's
/// native long-running `:predictLongRunning` action, reached through Portkey as
/// a custom-host pass-through. The provider integration slug (kept with its
/// leading `@`) lets Portkey attach the Vertex credentials, so no auth is sent
/// here. The follow-up poll uses `:fetchPredictOperation`.
fn veo_request(
    config: &PortkeyConfig,
    request: VideoRequest,
    aspect_ratio: &str,
    duration: u32,
) -> Result<VideoSubmission, UpstreamError> {
    let vertex = request.vertex.as_ref().ok_or_else(|| {
        UpstreamError::provider("missing Vertex configuration for Veo generation")
    })?;

    let mut headers = config.headers();

    let (provider, model) = split_model_reference(&request.model);
    if let Some(provider) = provider {
        let value = provider
            .parse()
            .map_err(|_| UpstreamError::provider("invalid video provider slug"))?;
        headers.insert("x-portkey-provider", value);
    }

    // Vertex's native endpoints aren't Portkey gateway routes, so proxy them to
    // the regional Vertex host. This header rides along on the poll too, since
    // the `VideoJob` reuses these headers.
    let host = HeaderValue::from_str(&vertex.custom_host)
        .map_err(|_| UpstreamError::provider("invalid Vertex custom host"))?;
    headers.insert("x-portkey-custom-host", host);

    let base = format!(
        "/projects/{}/locations/{}/publishers/google/models/{model}",
        vertex.project_id, vertex.location
    );
    let body = json!({
        "instances": [{ "prompt": request.prompt }],
        "parameters": {
            "aspectRatio": aspect_ratio,
            "durationSeconds": duration,
        },
    });

    tracing::debug!(model = %model, "creating Veo video generation job");

    Ok(VideoSubmission {
        headers,
        generate_url: config.url(&format!("{base}:predictLongRunning")),
        body,
        poll_target: PollTarget::FetchPredict(config.url(&format!("{base}:fetchPredictOperation"))),
    })
}

/// Submits a create request and returns the HTTP client plus the operation name
/// to poll. Shared across both provider request shapes.
async fn submit_job(
    headers: &HeaderMap,
    generate_url: String,
    body: Value,
) -> Result<(reqwest::Client, String), UpstreamError> {
    let http = reqwest::Client::new();

    let response = http
        .post(&generate_url)
        .headers(headers.clone())
        .json(&body)
        .send()
        .await
        .map_err(|_| UpstreamError::provider("Failed to reach the video generation provider."))?;

    let status = response.status();
    if !status.is_success() {
        return Err(match response.text().await {
            Ok(body) if !body.trim().is_empty() => classify_error(&body),
            Ok(_) => UpstreamError::provider(format!(
                "The video provider returned HTTP {status} with an empty body."
            )),
            Err(err) => UpstreamError::provider(format!(
                "The video provider returned HTTP {status} and its body could not be read: {err}"
            )),
        });
    }

    let job: Value = response.json().await.map_err(|_| {
        UpstreamError::provider("The video provider returned an unexpected response.")
    })?;

    let operation_name = extract_operation_name(&job)
        .ok_or_else(|| {
            UpstreamError::provider("The video provider did not return an operation to poll.")
        })?
        .to_string();

    tracing::debug!(%operation_name, "queueing video generation job");

    Ok((http, operation_name))
}

impl PollJob {
    fn poll(self) -> VideoStream {
        Box::pin(async_stream::stream! {
            let mut attempts = 0usize;

            loop {
                tokio::time::sleep(POLL_INTERVAL).await;

                attempts += 1;
                if attempts > MAX_POLL_ATTEMPTS {
                    yield Err(UpstreamError::provider("Video generation timed out."));
                    break;
                }

                let request = match self.poll_strategy {
                    PollStrategy::Get => {
                        self.http.get(&self.poll_url).headers(self.headers.clone())
                    }
                    PollStrategy::FetchPredict => self
                        .http
                        .post(&self.poll_url)
                        .headers(self.headers.clone())
                        .json(&json!({ "operationName": self.operation_name })),
                };

                let poll = match request.send().await {
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
                    continue;
                }

                if let Some(error) = operation.get("error") {
                    yield Err(classify_error(&error.to_string()));
                    break;
                }

                if let Some(response) = operation.get("response")
                    && response
                        .get("raiMediaFilteredCount")
                        .and_then(Value::as_u64)
                        .unwrap_or(0)
                        > 0
                {
                    let reason = response
                        .pointer("/raiMediaFilteredReasons/0")
                        .and_then(Value::as_str)
                        .unwrap_or("The video was blocked by the safety filter.");
                    tracing::warn!(operation_name = %self.operation_name, "safety filter blocks video");
                    yield Err(UpstreamError::provider(reason.to_string()));
                    break;
                }

                let bytes = match extract_video(&operation) {
                    Some(VideoPayload::Bytes(bytes)) => bytes,
                    Some(VideoPayload::Uri(uri)) => {
                        match download_video(&self.http, &self.headers, &uri).await {
                            Ok(bytes) => bytes,
                            Err(err) => {
                                tracing::error!(operation_name = %self.operation_name, "failed to download video: {err}");
                                yield Ok(VideoEvent::Link { url: uri });
                                break;
                            }
                        }
                    }
                    None => {
                        tracing::error!(operation_name = %self.operation_name, response = %operation, "completed operation has no video");
                        yield Err(UpstreamError::provider("The video provider returned no video."));
                        break;
                    }
                };

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

/// Split an `@<provider>/<model>` catalog reference into its provider slug and
/// bare model name.
///
/// The provider slug keeps its leading `@` so Portkey resolves it to the saved
/// integration (and its credentials); e.g. `@google-us-central1/veo-3.1` ->
/// (`@google-us-central1`, `veo-3.1`). References without a `/` are returned
/// unchanged with no provider.
fn split_model_reference(model: &str) -> (Option<&str>, &str) {
    match model.split_once('/') {
        Some((provider, model)) => (Some(provider), model),
        None => (None, model),
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
            humanize_error(&body)
        ));
    }

    Ok(response
        .bytes()
        .await
        .map_err(|err| err.to_string())?
        .to_vec())
}
