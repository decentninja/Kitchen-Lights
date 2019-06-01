use chrono::prelude::*;
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, OutputPin};

mod hue;
mod state;
mod config;

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
    let config = match config::read_from_file() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}", e);
            return
        }
    };
    let mut state = state::State::new();
    let mut button = Gpio::new().unwrap().get(config.pin).unwrap().into_output();
    loop {
        let bridge = wait_break!(
            hue::WrappedBridge::connect(),
            "Philips Hue Bridge",
            continue
        );
        println!("Hue connected!");
        loop {
            let value = wait_break!(bridge.magic(), "Philips Hue Bridge", break);
            let clicks = state.set(value, &config.state);
            if clicks != 0 {
                println!("{} Clicking to {:?}", Local::now(), state);
            }
            for _ in 0..clicks {
                tap(&mut button, config.hit_time_ms);
            }
            wait(config.poll_time_ms);
        }
    }
}

fn wait(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

fn tap(button: &mut OutputPin, hit_time: u64) {
    button.set_high();;
    wait(hit_time);
    button.set_low();
    wait(hit_time);
}
