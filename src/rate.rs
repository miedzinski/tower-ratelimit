use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Rate {
    rate: u32,
    window: Duration,
}

impl Rate {
    pub fn new(rate: u32, window: Duration) -> Self {
        Rate { rate, window }
    }

    pub(crate) fn rate(&self) -> u32 {
        self.rate
    }

    pub(crate) fn window(&self) -> Duration {
        self.window
    }
}
