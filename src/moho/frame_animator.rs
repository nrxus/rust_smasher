use std::time::Duration;

#[derive(Clone)]
pub struct FrameAnimator {
    max: u32,
    duration: Duration,
    repeat: bool,
    current: u32,
    elapsed: Option<Duration>,
}

impl FrameAnimator {
    pub fn new(max: u32, duration: Duration, repeat: bool) -> FrameAnimator {
        FrameAnimator {
            max: max,
            duration: duration,
            repeat: repeat,
            current: 0,
            elapsed: None,
        }
    }

    pub fn frame(&self) -> u32 {
        self.current
    }

    pub fn is_active(&self) -> bool {
        self.elapsed.is_some()
    }

    pub fn animate(&mut self, delta: Duration) {
        match self.elapsed {
            None => self.restart(),
            Some(duration) => {
                let elapsed = duration + delta;
                if elapsed >= self.duration {
                    self.advance(elapsed);
                } else {
                    self.elapsed = Some(elapsed);
                }
            }
        }
    }

    pub fn num_frames(&self) -> u32 {
        self.max
    }

    fn advance(&mut self, elapsed: Duration) {
        let remaining = elapsed - self.duration;
        self.current = (self.current + 1) % self.max;
        self.elapsed = if self.current > 0 || self.repeat {
            Some(remaining)
        } else {
            None
        };
    }

    fn restart(&mut self) {
        self.current = 0;
        self.elapsed = Some(Duration::new(0, 0));
    }
}
