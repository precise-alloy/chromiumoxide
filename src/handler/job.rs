use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::time::Duration;

use tokio::time::{Instant, Sleep, sleep};

use crate::handler::REQUEST_TIMEOUT;

/// A background job run periodically.
#[derive(Debug)]
pub(crate) struct PeriodicJob {
    interval: Duration,
    delay: Pin<Box<Sleep>>,
}

impl PeriodicJob {
    /// Returns `true` if the job is currently not running but ready
    /// to be run, `false` otherwise.
    pub fn poll_ready(&mut self, cx: &mut Context<'_>) -> bool {
        if !Future::poll(self.delay.as_mut(), cx).is_pending() {
            self.delay.as_mut().reset(Instant::now() + self.interval);
            // Poll the reset timer once so it holds this waker. Without it the next run happened
            // only if something else woke the handler: on a quiet connection a command, or a
            // target's initialisation, that timed out was never evicted (review F5).
            let _ = Future::poll(self.delay.as_mut(), cx);
            return true;
        }
        false
    }
    pub fn new(interval: Duration) -> Self {
        Self {
            delay: Box::pin(sleep(interval)),
            interval,
        }
    }
}

impl Default for PeriodicJob {
    fn default() -> Self {
        Self::new(Duration::from_millis(REQUEST_TIMEOUT))
    }
}
