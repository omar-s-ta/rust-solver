use std::time::{Duration, Instant};

/// A lightweight stopwatch for measuring elapsed time between checkpoints.
///
/// Useful for ad-hoc timing of solution phases during local debugging. When
/// enabled (the default), [`checkpoint`](TimeTracker::checkpoint) prints the
/// time since the previous checkpoint to stderr, so it does not interfere with
/// the program's stdout. Call [`disable`](TimeTracker::disable) to silence the
/// output.
pub struct TimeTracker {
    start: Instant,
    debug: bool,
}

impl TimeTracker {
    /// Creates a new tracker, starting the clock at the current instant with
    /// debug printing enabled.
    pub fn new() -> Self {
        TimeTracker {
            start: Instant::now(),
            debug: true,
        }
    }

    /// Returns the time elapsed since the last reset (construction or the most
    /// recent [`checkpoint`](TimeTracker::checkpoint)).
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Turns off the stderr output produced by
    /// [`checkpoint`](TimeTracker::checkpoint). Checkpoints still reset the clock.
    pub fn disable(&mut self) {
        self.debug = false;
    }

    /// Prints the elapsed time since the last reset, labelled with `mark`, then
    /// restarts the clock. Output is suppressed if the tracker has been
    /// [`disable`](TimeTracker::disable)d.
    pub fn checkpoint(&mut self, mark: &str) {
        if self.debug {
            eprintln!("{} : {}ms", mark, self.elapsed().as_millis());
        }
        self.start = Instant::now();
    }
}

impl Default for TimeTracker {
    fn default() -> Self {
        Self::new()
    }
}
