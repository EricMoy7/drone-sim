use super::{Callback, SimEnvErrors, Timers, FIVE_HERTZ_MS, TEN_HERTZ_MS, TWENTY_HERTZ_MS};

extern crate chrono;
extern crate timer;

pub struct ThreadTimer {
    five_hertz_callbacks: Vec<Callback>,
    ten_hertz_callbacks: Vec<Callback>,
    twenty_hertz_callbacks: Vec<Callback>,
    executing: bool,
}

impl ThreadTimer {
    pub fn init() -> Self {
        ThreadTimer {
            five_hertz_callbacks: Vec::new(),
            ten_hertz_callbacks: Vec::new(),
            twenty_hertz_callbacks: Vec::new(),
            executing: false,
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
            let timer = timer::Timer::new();

            let _five_hertz_guard = {
                let cbs = self.five_hertz_callbacks.clone();
                timer.schedule_repeating(chrono::Duration::milliseconds(FIVE_HERTZ_MS), move || {
                    cbs.iter().for_each(|cb| {
                        cb();
                    });
                })
            };

            let _ten_hertz_guard = {
                let cbs = self.ten_hertz_callbacks.clone();
                timer.schedule_repeating(chrono::Duration::milliseconds(TEN_HERTZ_MS), move || {
                    cbs.iter().for_each(|cb| {
                        cb();
                    });
                })
            };

            let _twenty_hertz_guard = {
                let cbs = self.twenty_hertz_callbacks.clone();
                timer.schedule_repeating(
                    chrono::Duration::milliseconds(TWENTY_HERTZ_MS),
                    move || {
                        cbs.iter().for_each(|cb| {
                            cb();
                        });
                    },
                )
            };

            loop {}
        }
    }
}
