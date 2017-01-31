use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct FrameAnimator {
    max: u32,
    duration: Duration,
    repeat: bool,
    current: u32,
    instant: Option<Instant>,
}

impl FrameAnimator {
    pub fn new(max: u32, duration: Duration, repeat: bool) -> FrameAnimator {
        FrameAnimator {
            max: max,
            duration: duration,
            repeat: repeat,
            current: 0,
            instant: None,
        }
    }

    pub fn frame(&self) -> u32 {
        self.current
    }

    pub fn is_active(&self) -> bool {
        self.instant.is_some()
    }

    pub fn animate(&mut self) {
        match self.instant {
            None => self.restart(),
            Some(instant) if instant.elapsed() >= self.duration => self.advance(instant),
            _ => {}
        }
    }

    pub fn num_frames(&self) -> u32 {
        self.max
    }

    fn advance(&mut self, instant: Instant) {
        self.current = (self.current + 1) % self.max;
        self.instant = if self.current > 0 || self.repeat {
            Some(instant + self.duration)
        } else {
            None
        };
    }

    fn restart(&mut self) {
        self.current = 0;
        self.instant = Some(Instant::now());
    }
}
