use std::time::{Duration, Instant};

const WORK_DURATION: Duration = Duration::from_secs(25 * 60);
const SHORT_BREAK_DURATION: Duration = Duration::from_secs(5 * 60);
const LONG_BREAK_DURATION: Duration = Duration::from_secs(15 * 60);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Self::Work => "Work",
            Self::ShortBreak => "Short break",
            Self::LongBreak => "Long break",
        }
    }

    fn duration(self) -> Duration {
        match self {
            Self::Work => WORK_DURATION,
            Self::ShortBreak => SHORT_BREAK_DURATION,
            Self::LongBreak => LONG_BREAK_DURATION,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerState {
    Ready,
    Running { deadline: Instant },
    Paused,
    Complete { next_phase: Phase },
}

pub struct Timer {
    phase: Phase,
    state: TimerState,
    remaining: Duration,
    completed_work_sessions: u32,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        Self {
            phase: Phase::Work,
            state: TimerState::Ready,
            remaining: WORK_DURATION,
            completed_work_sessions: 0,
        }
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn state(&self) -> TimerState {
        self.state
    }

    pub fn remaining(&self) -> Duration {
        self.remaining
    }

    pub fn completed_work_sessions(&self) -> u32 {
        self.completed_work_sessions
    }

    pub fn start(&mut self, now: Instant) {
        if let TimerState::Complete { next_phase } = self.state {
            self.phase = next_phase;
            self.remaining = next_phase.duration();
        }

        if matches!(
            self.state,
            TimerState::Ready | TimerState::Paused | TimerState::Complete { .. }
        ) {
            self.state = TimerState::Running {
                deadline: now + self.remaining,
            };
        }
    }

    pub fn pause(&mut self, now: Instant) {
        self.update(now);
        if let TimerState::Running { deadline } = self.state {
            self.remaining = deadline.saturating_duration_since(now);
            self.state = TimerState::Paused;
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Ready;
        self.remaining = self.phase.duration();
    }

    pub fn skip(&mut self) {
        self.phase = match self.state {
            TimerState::Complete { next_phase } => next_phase,
            _ => self.select_next_phase(false),
        };
        self.state = TimerState::Ready;
        self.remaining = self.phase.duration();
    }

    pub fn update(&mut self, now: Instant) {
        let TimerState::Running { deadline } = self.state else {
            return;
        };

        self.remaining = deadline.saturating_duration_since(now);
        if self.remaining.is_zero() {
            if self.phase == Phase::Work {
                self.completed_work_sessions += 1;
            }
            self.state = TimerState::Complete {
                next_phase: self.select_next_phase(true),
            };
        }
    }

    fn select_next_phase(&self, completed: bool) -> Phase {
        if self.phase == Phase::Work {
            if completed && self.completed_work_sessions % 4 == 0 {
                Phase::LongBreak
            } else {
                Phase::ShortBreak
            }
        } else {
            Phase::Work
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Phase, Timer, TimerState};
    use std::time::{Duration, Instant};

    #[test]
    fn countdown_pauses_and_resumes_with_remaining_time() {
        let start = Instant::now();
        let mut timer = Timer::new();

        timer.start(start);
        timer.update(start + Duration::from_secs(60));
        assert_eq!(timer.remaining(), Duration::from_secs(24 * 60));

        timer.pause(start + Duration::from_secs(60));
        timer.start(start + Duration::from_secs(5 * 60));
        timer.update(start + Duration::from_secs(5 * 60 + 30));

        assert_eq!(timer.remaining(), Duration::from_secs(23 * 60 + 30));
        assert!(matches!(timer.state(), TimerState::Running { .. }));
    }

    #[test]
    fn reset_restores_the_current_phase_duration() {
        let start = Instant::now();
        let mut timer = Timer::new();

        timer.start(start);
        timer.update(start + Duration::from_secs(90));
        timer.reset();

        assert_eq!(timer.remaining(), Duration::from_secs(25 * 60));
        assert_eq!(timer.state(), TimerState::Ready);
    }

    #[test]
    fn completing_work_offers_a_short_break_and_waits_for_start() {
        let start = Instant::now();
        let mut timer = Timer::new();

        timer.start(start);
        timer.update(start + Duration::from_secs(25 * 60));

        assert_eq!(timer.phase(), Phase::Work);
        assert_eq!(
            timer.state(),
            TimerState::Complete {
                next_phase: Phase::ShortBreak
            }
        );

        timer.start(start + Duration::from_secs(25 * 60));
        assert_eq!(timer.phase(), Phase::ShortBreak);
        assert_eq!(timer.remaining(), Duration::from_secs(5 * 60));
        assert!(matches!(timer.state(), TimerState::Running { .. }));
    }

    #[test]
    fn pausing_at_the_deadline_completes_the_work_session() {
        let start = Instant::now();
        let mut timer = Timer::new();

        timer.start(start);
        timer.pause(start + Duration::from_secs(25 * 60));

        assert_eq!(
            timer.state(),
            TimerState::Complete {
                next_phase: Phase::ShortBreak
            }
        );
        assert_eq!(timer.completed_work_sessions(), 1);
    }

    #[test]
    fn fourth_completed_work_session_offers_a_long_break() {
        let start = Instant::now();
        let mut timer = Timer::new();
        let mut now = start;

        for session in 0..4 {
            timer.start(now);
            now += Duration::from_secs(25 * 60);
            timer.update(now);
            if session < 3 {
                timer.start(now);
                now += Duration::from_secs(5 * 60);
                timer.update(now);
            }
        }

        assert_eq!(
            timer.state(),
            TimerState::Complete {
                next_phase: Phase::LongBreak
            }
        );
    }

    #[test]
    fn skip_moves_to_the_next_phase_without_completing_a_work_session() {
        let mut timer = Timer::new();
        timer.skip();

        assert_eq!(timer.phase(), Phase::ShortBreak);
        assert_eq!(timer.state(), TimerState::Ready);
        assert_eq!(timer.completed_work_sessions(), 0);
    }
}
