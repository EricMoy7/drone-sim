use super::{Callback, SimEnvErrors, Timers, FIVE_HERTZ_MS, TEN_HERTZ_MS, TWENTY_HERTZ_MS};
use timer::Timer;
extern crate chrono;
extern crate timer;

pub struct ThreadTimer {
    five_hertz_callbacks: Vec<Callback>,
    five_hertz_guard: Option<timer::Guard>,
    ten_hertz_callbacks: Vec<Callback>,
    ten_hertz_guard: Option<timer::Guard>,
    twenty_hertz_callbacks: Vec<Callback>,
    twenty_hertz_guard: Option<timer::Guard>,
    executing: bool,
    timer: Timer,
}

impl ThreadTimer {
    pub fn init() -> Self {
        ThreadTimer {
            five_hertz_callbacks: Vec::new(),
            five_hertz_guard: None,
            ten_hertz_callbacks: Vec::new(),
            ten_hertz_guard: None,
            twenty_hertz_callbacks: Vec::new(),
            twenty_hertz_guard: None,
            executing: false,
            timer: Timer::new(),
        }
    }

    pub fn register(&mut self, timer: Timers, cb: Callback) -> Result<(), SimEnvErrors> {
        if self.executing {
            Err(SimEnvErrors::RegisterFail)
        } else {
            match timer {
                Timers::FiveHertz => self.five_hertz_callbacks.push(cb),
                Timers::TenHertz => self.ten_hertz_callbacks.push(cb),
                Timers::TwentyHertz => self.twenty_hertz_callbacks.push(cb),
            }
            Ok(())
        }
    }

    pub fn start_execution(&mut self) -> Result<(), SimEnvErrors> {
        if self.executing {
            Err(SimEnvErrors::ExecFail)
        } else {
            self.executing = true;

            self.five_hertz_guard = exec_repeating_fn(
                &self.timer,
                FIVE_HERTZ_MS,
                self.five_hertz_callbacks.clone(),
            );

            self.ten_hertz_guard =
                exec_repeating_fn(&self.timer, TEN_HERTZ_MS, self.ten_hertz_callbacks.clone());

            self.twenty_hertz_guard = exec_repeating_fn(
                &self.timer,
                TWENTY_HERTZ_MS,
                self.twenty_hertz_callbacks.clone(),
            );
            Ok(())
        }
    }

    pub fn stop_execution(self) -> Result<(), SimEnvErrors> {
        if !self.executing {
            Err(SimEnvErrors::StopFail)
        } else {
            drop(self);
            Ok(())
        }
    }
}
fn exec_repeating_fn(timer: &Timer, schedule: i64, cbs: Vec<Callback>) -> Option<timer::Guard> {
    let guard = timer.schedule_repeating(chrono::Duration::milliseconds(schedule), move || {
        cbs.iter().for_each(|cb| {
            cb();
        });
    });
    Some(guard)
}
