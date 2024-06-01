use std::time::{Instant, Duration};
use std::ops::Sub;

pub struct Timer {
    current: Instant,
    time_paused: Duration,
    is_paused: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            current: Instant::now(),
            time_paused: Duration::new(0, 0),
            is_paused: false,
        }
    }
    
    pub fn restart(&mut self) {
        self.current = Instant::now();
        self.time_paused = Duration::new(0, 0);
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    pub fn time(&mut self) -> Instant {
        if self.is_paused {
            self.time_paused = Instant::now().duration_since(self.current);
        } else {
            self.current = Instant::now().sub(self.time_paused);
        }

        self.current
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
}