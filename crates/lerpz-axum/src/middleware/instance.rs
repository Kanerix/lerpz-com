//! Middleware that captures the request path for [`Problem`] instances.
//!
//! [`Problem`]: crate::problem::Problem

use std::task::{Context, Poll};

use axum::extract::Request;
use tokio::task::futures::TaskLocalFuture;
use tower::{Layer, Service};

use crate::problem::REQUEST_INSTANCE;

/// [`Layer`] that captures the request path for use as the problem `instance`.
///
/// It stores the current request path in a task-local value for the duration
/// of the request. Any [`Problem`] turned into a response during that request,
/// and which does not already have an `instance` set, will have it filled in
/// automatically with this path.
///
/// This is what makes the `instance` field populate automatically for problems
/// created through the `?` operator, where no request context is otherwise
/// available.
///
/// Add it to a router like any other layer, no `from_fn` wrapping required:
///
/// ```ignore
/// use lerpz_axum::middleware::instance::CaptureInstanceLayer;
///
/// router.layer(CaptureInstanceLayer)
/// ```
///
/// [`Problem`]: crate::problem::Problem
#[derive(Clone, Copy, Debug, Default)]
pub struct CaptureInstanceLayer;

impl<S> Layer<S> for CaptureInstanceLayer {
    type Service = CaptureInstance<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CaptureInstance { inner }
    }
}

/// The [`Service`] produced by [`CaptureInstanceLayer`].
///
/// Wraps an inner service and runs it inside the [`REQUEST_INSTANCE`]
/// task-local scope, set to the request path.
#[derive(Clone, Copy, Debug)]
pub struct CaptureInstance<S> {
    inner: S,
}

impl<S> Service<Request> for CaptureInstance<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = TaskLocalFuture<String, S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let path = req.uri().path().to_owned();
        REQUEST_INSTANCE.scope(path, self.inner.call(req))
    }
}
