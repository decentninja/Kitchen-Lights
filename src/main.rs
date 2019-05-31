use chrono::prelude::*;
use std::thread;
use std::time::Duration;
use sysfs_gpio::{self, Pin};

mod hue;
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

fn main() {
    let mut state = state::State::new();
    let button = Pin::new(7);
    loop {
        let bridge = wait_break!(
            hue::WrappedBridge::connect(),
            "Philips Hue Bridge",
            continue
        );
        wait_break!(button.export(), "Could not get gpio pin 7", continue);
        println!("Hue connected!");
        loop {
            let value = wait_break!(bridge.magic(), "Philips Hue Bridge", break);
            let clicks = state.set(value);
            if clicks != 0 {
                println!("{} Clicking to {:?}", Local::now(), state);
            }
            for _ in 0..clicks {
                wait_break!(tap(&button), "Tapping", break);
            }
            wait(1000);
        }
    }
}

fn wait(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

fn tap(button: &Pin) -> Result<(), sysfs_gpio::Error> {
    button.set_value(1)?;
    wait(100);
    button.set_value(0)?;
    Ok(())
}
