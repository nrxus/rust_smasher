use std::time::Duration;

#[derive(Default, Clone)]
struct FrameInfo {
    frame: u32,
    elapsed: Duration,
}

impl FrameInfo {
    fn new(frame: u32, elapsed: Duration) -> Self {
        FrameInfo {
            frame: frame,
            elapsed: elapsed,
        }
    }

    fn elapse(&self, elapsed: Duration) -> Self {
        FrameInfo::new(self.frame, elapsed)
    }
}

#[derive(Clone)]
pub struct FrameAnimator {
    max: u32,
    duration: Duration,
    repeat: bool,
    current: Option<FrameInfo>,
}

impl FrameAnimator {
    pub fn new(max: u32, duration: Duration, repeat: bool) -> FrameAnimator {
        FrameAnimator {
            max: max,
            duration: duration,
            repeat: repeat,
            current: None,
        }
    }

    pub fn frame(&self) -> u32 {
        self.current.as_ref().map_or(0, |i| i.frame)
    }

    pub fn is_active(&self) -> bool {
        self.current.is_some()
    }

    pub fn animate(&mut self, delta: Duration) {
        self.current =
            self.current.as_ref().map_or(Some(FrameInfo::default()), |i| self.advance(i, delta));
    }

    pub fn num_frames(&self) -> u32 {
        self.max
    }

    fn advance(&self, current: &FrameInfo, delta: Duration) -> Option<FrameInfo> {
        let elapsed = current.elapsed + delta;
        elapsed.checked_sub(self.duration)
            .map_or(Some(current.elapse(elapsed)), |r| self.next(current, r))
    }

    fn next(&self, current: &FrameInfo, remaining: Duration) -> Option<FrameInfo> {
        let frame = (current.frame + 1) % self.max;
        if frame > 0 || self.repeat {
            Some(FrameInfo::new(frame, remaining))
        } else {
            None
        }
    }
}
