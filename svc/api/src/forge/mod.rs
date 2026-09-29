use std::time::{Duration, Instant};

use axum::http::StatusCode;
use lerpz_axum::problem::{HandlerResult, Problem};
use reqwest::{Client, Method, RequestBuilder, Response, Url};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

mod error;

use error::{Error, Result};

#[derive(Serialize)]
pub(crate) struct CreateRuntimeRequest {
    pub agent: String,
    pub replicas: i32,
    pub mount_memory: bool,
    pub cpu_limit: Option<String>,
    pub memory_limit: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct CreateVolumeRequest {
    pub agent: String,
}

#[derive(Deserialize)]
pub(crate) struct RuntimeResponse {
    pub agent: String,
    pub replicas: Option<i32>,
    pub ready_replicas: Option<i32>,
    pub memory_volume: Option<String>,
    pub base_url: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub(crate) struct VolumeResponse {
    pub agent: String,
    pub phase: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Clone, Copy)]
enum Resource {
    Runtime,
    Volume,
}

impl Resource {
    fn path(self) -> &'static str {
        match self {
            Self::Runtime => "/api/v1/runtimes",
            Self::Volume => "/api/v1/volumes",
        }
    }
}

#[derive(Clone)]
pub(crate) struct ForgeClient {
    client: Client,
    origin: Url,
}

impl ForgeClient {
    pub(crate) fn new(origin: Url) -> Result<Self> {
        if !matches!(origin.scheme(), "http" | "https")
            || origin.host_str().is_none()
            || !origin.username().is_empty()
            || origin.password().is_some()
            || origin.path() != "/"
            || origin.query().is_some()
            || origin.fragment().is_some()
        {
            return Err(Error::InvalidOrigin);
        }

        let client = Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(60))
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .retry(reqwest::retry::never())
            .build()?;
        Ok(Self { client, origin })
    }

    #[tracing::instrument(
        name = "forge.list_runtimes",
        skip_all,
        fields(operation = "list", resource = "runtime")
    )]
    pub(crate) async fn list_runtimes(
        &self,
        token: &SecretString,
    ) -> HandlerResult<Vec<RuntimeResponse>> {
        let request = self.request(Method::GET, Resource::Runtime, token, None)?;
        Self::json(request, StatusCode::OK).await
    }

    #[tracing::instrument(name = "forge.read_runtime", skip_all, fields(operation = "read", resource = "runtime", agent_name = tracing::field::Empty))]
    pub(crate) async fn read_runtime(
        &self,
        token: &SecretString,
        agent: &str,
    ) -> HandlerResult<Option<RuntimeResponse>> {
        self.read(Resource::Runtime, token, agent).await
    }

    #[tracing::instrument(name = "forge.create_runtime", skip_all, fields(operation = "create", resource = "runtime", agent_name = tracing::field::Empty))]
    pub(crate) async fn create_runtime(
        &self,
        token: &SecretString,
        body: &CreateRuntimeRequest,
    ) -> HandlerResult<RuntimeResponse> {
        validate_agent(&body.agent)?;
        tracing::Span::current().record("agent_name", body.agent.as_str());
        let request = self
            .request(Method::POST, Resource::Runtime, token, None)?
            .json(body);
        Self::json(request, StatusCode::CREATED).await
    }

    #[tracing::instrument(name = "forge.delete_runtime", skip_all, fields(operation = "delete", resource = "runtime", agent_name = tracing::field::Empty))]
    pub(crate) async fn delete_runtime(
        &self,
        token: &SecretString,
        agent: &str,
    ) -> HandlerResult<()> {
        self.delete(Resource::Runtime, token, agent).await
    }

    #[tracing::instrument(
        name = "forge.list_volumes",
        skip_all,
        fields(operation = "list", resource = "volume")
    )]
    pub(crate) async fn list_volumes(
        &self,
        token: &SecretString,
    ) -> HandlerResult<Vec<VolumeResponse>> {
        let request = self.request(Method::GET, Resource::Volume, token, None)?;
        Self::json(request, StatusCode::OK).await
    }

    #[tracing::instrument(name = "forge.read_volume", skip_all, fields(operation = "read", resource = "volume", agent_name = tracing::field::Empty))]
    pub(crate) async fn read_volume(
        &self,
        token: &SecretString,
        agent: &str,
    ) -> HandlerResult<Option<VolumeResponse>> {
        self.read(Resource::Volume, token, agent).await
    }

    #[tracing::instrument(name = "forge.create_volume", skip_all, fields(operation = "create", resource = "volume", agent_name = tracing::field::Empty))]
    pub(crate) async fn create_volume(
        &self,
        token: &SecretString,
        body: &CreateVolumeRequest,
    ) -> HandlerResult<VolumeResponse> {
        validate_agent(&body.agent)?;
        tracing::Span::current().record("agent_name", body.agent.as_str());
        let request = self
            .request(Method::POST, Resource::Volume, token, None)?
            .json(body);
        Self::json(request, StatusCode::CREATED).await
    }

    #[tracing::instrument(name = "forge.delete_volume", skip_all, fields(operation = "delete", resource = "volume", agent_name = tracing::field::Empty))]
    pub(crate) async fn delete_volume(
        &self,
        token: &SecretString,
        agent: &str,
    ) -> HandlerResult<()> {
        self.delete(Resource::Volume, token, agent).await
    }

    async fn read<T: DeserializeOwned>(
        &self,
        resource: Resource,
        token: &SecretString,
        agent: &str,
    ) -> HandlerResult<Option<T>> {
        let request = self.request(Method::GET, resource, token, Some(agent))?;
        let expected = StatusCode::OK;
        let started = Instant::now();
        let response = request
            .send()
            .await
            .map_err(|error| transport_problem(error, expected, None, started))?;
        let actual = response.status();
        if actual == StatusCode::NOT_FOUND {
            tracing::debug!(
                status = actual.as_u16(),
                expected_status = expected.as_u16(),
                elapsed_ms = started.elapsed().as_millis(),
                "forge resource is absent"
            );
            return Ok(None);
        }
        let body = Self::check_response(response, expected, started)?
            .json()
            .await
            .map_err(|error| transport_problem(error, expected, Some(actual), started))?;
        tracing::debug!(
            status = actual.as_u16(),
            expected_status = expected.as_u16(),
            elapsed_ms = started.elapsed().as_millis(),
            "forge request succeeds"
        );
        Ok(Some(body))
    }

    async fn delete(
        &self,
        resource: Resource,
        token: &SecretString,
        agent: &str,
    ) -> HandlerResult<()> {
        let request = self.request(Method::DELETE, resource, token, Some(agent))?;
        let expected = StatusCode::NO_CONTENT;
        let started = Instant::now();
        let response = Self::send(request, expected, started).await?;
        let actual = response.status();
        let body = response
            .bytes()
            .await
            .map_err(|error| transport_problem(error, expected, Some(actual), started))?;
        if !body.is_empty() {
            tracing::error!(
                status = actual.as_u16(),
                expected_status = expected.as_u16(),
                elapsed_ms = started.elapsed().as_millis(),
                error_kind = "unexpected_body",
                "forge returns an unexpected body"
            );
            return Err(gateway_problem().with_error(Error::UnexpectedBody));
        }
        tracing::debug!(
            status = actual.as_u16(),
            expected_status = expected.as_u16(),
            elapsed_ms = started.elapsed().as_millis(),
            "forge request succeeds"
        );
        Ok(())
    }

    fn request(
        &self,
        method: Method,
        resource: Resource,
        token: &SecretString,
        agent: Option<&str>,
    ) -> HandlerResult<RequestBuilder> {
        let mut url = self.origin.clone();
        if let Some(agent) = agent {
            validate_agent(agent)?;
            tracing::Span::current().record("agent_name", agent);
            url.set_path(&format!("{}/{agent}", resource.path()));
        } else {
            url.set_path(resource.path());
        }
        Ok(self
            .client
            .request(method, url)
            .bearer_auth(token.expose_secret()))
    }

    async fn json<T: DeserializeOwned>(
        request: RequestBuilder,
        expected: StatusCode,
    ) -> HandlerResult<T> {
        let started = Instant::now();
        let response = Self::send(request, expected, started).await?;
        let actual = response.status();
        let body = response
            .json()
            .await
            .map_err(|error| transport_problem(error, expected, Some(actual), started))?;
        tracing::debug!(
            status = actual.as_u16(),
            expected_status = expected.as_u16(),
            elapsed_ms = started.elapsed().as_millis(),
            "forge request succeeds"
        );
        Ok(body)
    }

    async fn send(
        request: RequestBuilder,
        expected: StatusCode,
        started: Instant,
    ) -> HandlerResult<Response> {
        let response = request
            .send()
            .await
            .map_err(|error| transport_problem(error, expected, None, started))?;
        Self::check_response(response, expected, started)
    }

    fn check_response(
        response: Response,
        expected: StatusCode,
        started: Instant,
    ) -> HandlerResult<Response> {
        let actual = response.status();
        if actual == expected {
            return Ok(response);
        }

        if actual.is_client_error() {
            tracing::warn!(
                status = actual.as_u16(),
                expected_status = expected.as_u16(),
                elapsed_ms = started.elapsed().as_millis(),
                "forge returns an unexpected status"
            );
        } else {
            tracing::error!(
                status = actual.as_u16(),
                expected_status = expected.as_u16(),
                elapsed_ms = started.elapsed().as_millis(),
                "forge returns an unexpected status"
            );
        }

        // Upstream error bodies can contain cluster details and are never relayed or logged.
        let problem = match actual {
            StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => Problem::new(
                StatusCode::BAD_REQUEST,
                "Invalid agent request",
                "The agent parameters were rejected. Check the agent name and settings.",
            ),
            StatusCode::UNAUTHORIZED => Problem::new(
                actual,
                "Authentication required",
                "Sign in to access agents and agent memory.",
            ),
            StatusCode::FORBIDDEN => Problem::new(
                actual,
                "Agent access denied",
                "You do not have permission to access this agent or its memory.",
            ),
            StatusCode::NOT_FOUND => Problem::new(
                actual,
                "Agent or memory not found",
                "The requested agent or agent memory does not exist or is not accessible to the caller.",
            ),
            StatusCode::CONFLICT => Problem::new(
                actual,
                "Agent or memory conflict",
                "The agent or its memory already exists or changed during the operation. Refresh the agent before retrying.",
            ),
            StatusCode::GATEWAY_TIMEOUT => {
                timeout_problem().with_error(Error::UnexpectedStatus { actual, expected })
            }
            _ => gateway_problem().with_error(Error::UnexpectedStatus { actual, expected }),
        };
        Err(problem)
    }
}

fn validate_agent(agent: &str) -> HandlerResult<()> {
    if agent.is_empty()
        || agent.len() > 40
        || agent.starts_with('-')
        || agent.ends_with('-')
        || !agent
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
    {
        return Err(Problem::new(
            StatusCode::BAD_REQUEST,
            "Invalid agent name",
            "The agent name must contain 1 to 40 lowercase letters, digits or hyphens, and start and end with a letter or digit.",
        ));
    }
    Ok(())
}

fn transport_problem(
    error: reqwest::Error,
    expected: StatusCode,
    actual: Option<StatusCode>,
    started: Instant,
) -> Problem {
    tracing::error!(
        status = actual.map(|status| status.as_u16()),
        expected_status = expected.as_u16(),
        elapsed_ms = started.elapsed().as_millis(),
        is_timeout = error.is_timeout(),
        is_connect = error.is_connect(),
        is_body = error.is_body(),
        is_decode = error.is_decode(),
        is_builder = error.is_builder(),
        "forge request fails"
    );
    if error.is_decode() {
        // Decode errors can include values from the response body.
        return gateway_problem().with_error(Error::InvalidResponse);
    }
    let problem = if error.is_timeout() {
        timeout_problem()
    } else {
        gateway_problem()
    };
    problem.with_error(error.without_url())
}

fn gateway_problem() -> Problem {
    Problem::new(
        StatusCode::BAD_GATEWAY,
        "Agent request failed",
        "The request could not be completed. Check the agent and its memory before retrying.",
    )
}

fn timeout_problem() -> Problem {
    Problem::new(
        StatusCode::GATEWAY_TIMEOUT,
        "Agent request timed out",
        "The request did not complete in time. Check the agent and its memory before retrying.",
    )
}
