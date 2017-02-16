use std::time::{Duration, Instant};

#[derive(Debug,Clone)]
pub struct GameTime {
    pub total: Duration,
    pub since_update: Duration,
}

pub struct Timer {
    start: Instant,
    last_update: Instant,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Timer {
            start: now.clone(),
            last_update: now,
        }
    }

    pub fn update(&mut self) {
        self.last_update = Instant::now();
    }

    pub fn game_time(&self) -> GameTime {
        let now = Instant::now();
        GameTime {
            total: now - self.start,
            since_update: now - self.last_update,
        }
    }
}
