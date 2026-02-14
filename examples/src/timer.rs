#![cfg_attr(not(test), no_main)]
#![no_std]
extern crate alloc;

use core::time::Duration;

use wipi::{app::App, println, timer::Timer, wipi_main};

pub struct TimerApp {
    _periodic: Timer,
    _oneshot: Timer,
}

impl TimerApp {
    fn new() -> Self {
        let periodic = Timer::periodic(Duration::from_secs(1), || {
            println!("tick");
        });

        let oneshot = Timer::once(Duration::from_secs(5), || {
            println!("5 seconds elapsed");
        });

        Self {
            _periodic: periodic,
            _oneshot: oneshot,
        }
    }
}

impl App for TimerApp {}

#[wipi_main]
pub fn main() -> TimerApp {
    TimerApp::new()
}
