mod hue;
mod state;
mod mindstorm;

use std::thread;
use std::time::Duration;


fn main() -> Result<(), Box<std::error::Error>> {
    let bridge = hue::WrappedBridge::connect()?;
    println!("Hue connected!");
    let mut state = state::State::new();
    let mut mindstorm = mindstorm::Mindstorm::connect().expect("No mindstorm connected!");
    println!("Mindstorm connected!");
    loop {
        let value = bridge.magic()?;
        let clicks = state.set(value);
        if clicks != 0 {
            println!("Clicking to {:?}", state);
        }
        for _ in 0..clicks {
            mindstorm.motor_a(-65, Duration::from_millis(100));
            mindstorm.motor_a(10, Duration::from_millis(100));
            thread::sleep(Duration::from_millis(100));
        }
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
