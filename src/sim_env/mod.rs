pub mod thread_timer;

pub enum Timers {
    FiveHertz,
    TenHertz,
    TwentyHertz,
}

pub enum SimEnvErrors {
    RegisterFail,
    ExecFail,
}

type Callback = fn();

const FIVE_HERTZ_MS: i64 = 200;
const TEN_HERTZ_MS: i64 = 100;
const TWENTY_HERTZ_MS: i64 = 50;
