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

    pub fn frame(&self) -> Option<u32> {
        self.current.as_ref().map(|c| c.frame)
    }

    pub fn num_frames(&self) -> u32 {
        self.max
    }

    pub fn animate(&mut self, delta: Duration) {
        self.current = self.current.as_ref().map_or(Some(Default::default()), |frame| {
            let elapsed = frame.elapsed + delta;
            elapsed.checked_sub(self.duration)
                .map_or(Some(frame.elapse(elapsed)), |r| self.next(frame, r))
        });
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
