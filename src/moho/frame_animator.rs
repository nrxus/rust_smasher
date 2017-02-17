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

    pub fn started(max: u32, duration: Duration, repeat: bool) -> FrameAnimator {
        FrameAnimator {
            max: max,
            duration: duration,
            repeat: repeat,
            current: Some(Default::default()),
        }
    }

    pub fn start(&mut self) {
        self.current = Some(Default::default());
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_animator() {
        let animator = FrameAnimator::new(3, Duration::from_secs(5), true);
        assert!(animator.frame().is_none());
    }

    #[test]
    fn started_animator() {
        let animator = FrameAnimator::started(3, Duration::from_secs(5), true);
        assert!(animator.frame().is_some());
        assert_eq!(animator.frame().unwrap(), 0);
    }

    #[test]
    fn start() {
        let mut animator = FrameAnimator::new(3, Duration::from_secs(5), true);
        animator.start();
        assert!(animator.frame().is_some());
        assert_eq!(animator.frame().unwrap(), 0);
    }
}
