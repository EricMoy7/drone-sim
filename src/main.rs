mod sim_env;

fn periodic_test() -> () {
    println!("Hello!")
}

fn periodic_test2() -> () {
    println!("123")
}

fn periodic_test3() -> () {
    println!("bad")
}

fn main() {
    println!("Hello, I am a drone!");

    let mut timer_fns: sim_env::thread_timer::ThreadTimer =
        sim_env::thread_timer::ThreadTimer::init();

    let _ = timer_fns.register(sim_env::Timers::FiveHertz, periodic_test);
    let _ = timer_fns.register(sim_env::Timers::TenHertz, periodic_test2);
    let _ = timer_fns.register(sim_env::Timers::TwentyHertz, periodic_test3);

    let _ = timer_fns.start_execution();
}
