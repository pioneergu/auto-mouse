use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum TimerState {
    Stopped,
    Running,
    Expired,
}

pub struct SimpleTimer {
    duration: Duration,
    start_time: Option<Instant>,
    state: TimerState,
}

impl SimpleTimer {
    pub fn new(minutes: u32) -> Self {
        Self {
            duration: Duration::from_secs(minutes as u64 * 60),
            start_time: None,
            state: TimerState::Stopped,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.state = TimerState::Running;
    }

    pub fn stop(&mut self) {
        self.start_time = None;
        self.state = TimerState::Stopped;
    }

    pub fn is_expired(&mut self) -> bool {
        if self.state != TimerState::Running {
            return false;
        }

        if let Some(start) = self.start_time {
            if start.elapsed() >= self.duration {
                self.state = TimerState::Expired;
                return true;
            }
        }
        false
    }

    pub fn get_remaining_seconds(&self) -> u64 {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            if elapsed < self.duration {
                return (self.duration - elapsed).as_secs();
            }
        }
        0
    }

    pub fn get_state(&self) -> TimerState {
        self.state.clone()
    }

    pub fn set_duration(&mut self, minutes: u32) {
        self.duration = Duration::from_secs(minutes as u64 * 60);
        if self.state == TimerState::Running {
            self.start_time = Some(Instant::now()); // 재시작
        }
    }
}

impl Default for SimpleTimer {
    fn default() -> Self {
        Self::new(60) // 기본 60분
    }
}
