use super::{redis::Redis, Rate, ResponseFuture};
use redis::aio::MultiplexedConnection;
use std::error::Error;
use std::task::{Context, Poll};
use std::time::SystemTime;
use tower::{BoxError, Service};

#[derive(Debug, Clone)]
pub struct RateLimit<S> {
    inner: S,
    rate: Rate,
    redis: Redis,
}

impl<S> RateLimit<S> {
    pub fn new(inner: S, rate: Rate, connection: MultiplexedConnection, key: String) -> Self {
        RateLimit {
            inner,
            rate,
            redis: Redis::new(connection, key),
        }
    }
}

impl<S, Request> Service<Request> for RateLimit<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = Box<dyn Error + Send + Sync>;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let window_size = self.rate.window().as_secs();
        let current_window = (now / window_size) * window_size;
        let previous_window = (now / window_size - 1) * window_size;
        let previous_window_ratio =
            ((window_size - (now - current_window)) as f64) / (window_size as f64);

        let query_future = {
            let mut redis = self.redis.clone();
            Box::pin(async move { redis.query(previous_window, current_window).await })
        };
        let increment_future = {
            let mut redis = self.redis.clone();
            Box::pin(async move { redis.increment(current_window, window_size).await })
        };
        let response_future = self.inner.call(req);

        ResponseFuture::new(
            query_future,
            increment_future,
            response_future,
            self.rate,
            previous_window_ratio,
        )
    }
}
