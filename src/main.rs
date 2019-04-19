use chrono::prelude::*;
use std::thread;
use std::time::Duration;

mod hue;
mod mindstorm;
mod state;

macro_rules! wait_break {
    ($expression:expr, $explain:expr, $do:expr) => {
        match $expression {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{} error {}, will restart in 5 secs!", $explain, e);
                wait(5000);
                $do
            }
        };
    };
}

macro_rules! wait_break_option {
    ($expression:expr, $explain:expr, $do:expr) => {
        match $expression {
            Some(value) => value,
            None => {
                eprintln!("{} will restart in 5 secs!", $explain);
                wait(5000);
                $do
            }
        };
    };
}

fn main() {
    loop {
        let bridge = wait_break!(
            hue::WrappedBridge::connect(),
            "Philips Hue Bridge",
            continue
        );
        println!("Hue connected!");
        let mut state = state::State::new();
        let mut mindstorm = wait_break_option!(
            mindstorm::Mindstorm::connect(),
            "No Mindstorm Connected",
            continue
        );
        println!("Mindstorm connected!");
        loop {
            let value = wait_break!(bridge.magic(), "Philips Hue Bridge", break);
            let clicks = state.set(value);
            if clicks != 0 {
                wait_break!(tap(&mut mindstorm), "Mindstorm", break);
                println!("{} Clicking to {:?}", Local::now(), state);
            }
            for _ in 0..clicks {
                wait(100);
            }
            wait(1000);
        }
        wait(5000);
    }
}

fn wait(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

fn tap(mindstorm: &mut mindstorm::Mindstorm) -> Result<(), mindstorm::DisconnectError> {
    mindstorm.motor_a(-65, Duration::from_millis(100))?;
    mindstorm.motor_a(10, Duration::from_millis(100))?;
    Ok(())
}
