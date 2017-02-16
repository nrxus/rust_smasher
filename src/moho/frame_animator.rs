use std::time::Duration;

#[derive(Default)]
struct Info {
    current: u32,
    elapsed: Option<Duration>,
}

impl Info {
    fn restarted() -> Self {
        Self::new(0, Some(Duration::default()))
    }

    fn elapsed(current: u32, elapsed: Duration) -> Self {
        Self::new(current, Some(elapsed))
    }

    fn new(current: u32, elapsed: Option<Duration>) -> Self {
        Info {
            current: current,
            elapsed: elapsed,
        }
    }
}

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
        let info = self.elapsed
            .map_or(Info::restarted(), |d| self.advance(d + delta));
        self.current = info.current;
        self.elapsed = info.elapsed;
    }

    pub fn num_frames(&self) -> u32 {
        self.max
    }

    fn advance(&self, elapsed: Duration) -> Info {
        elapsed.checked_sub(self.duration)
            .map_or(Info::elapsed(self.current, elapsed), |r| self.next(r))
    }

    fn next(&self, remaining: Duration) -> Info {
        let current = (self.current + 1) % self.max;
        if current > 0 || self.repeat {
            Info::elapsed(current, remaining)
        } else {
            Info::default()
        }
    }
}
