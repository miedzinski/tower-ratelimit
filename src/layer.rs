use super::{Rate, RateLimit};
use redis::aio::MultiplexedConnection;
use std::time::Duration;
use tower::Layer;

#[derive(Debug, Clone)]
pub struct RateLimitLayer {
    rate: Rate,
    connection: MultiplexedConnection,
    key: String,
}

impl RateLimitLayer {
    pub fn new(
        rate: u32,
        window: Duration,
        connection: MultiplexedConnection,
        key: String,
    ) -> Self {
        RateLimitLayer {
            rate: Rate::new(rate, window),
            connection,
            key,
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimit<S>;

    fn layer(&self, service: S) -> Self::Service {
        RateLimit::new(
            service,
            self.rate,
            self.connection.clone(),
            self.key.clone(),
        )
    }
}
