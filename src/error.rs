use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct RateLimitExceeded;

impl fmt::Display for RateLimitExceeded {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad("rate limit exceeded")
    }
}

impl Error for RateLimitExceeded {}
