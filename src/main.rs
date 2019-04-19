mod hue;
mod state;
use std::thread;
use std::time::Duration;


fn main() -> Result<(), Box<std::error::Error>> {
    let bridge = hue::WrappedBridge::connect()?;
    let mut state = state::State::new();
    loop {
        let value = bridge.magic()?;
        let clicks = state.set(value);
        if clicks > 0 {
            println!("Click needed {} to get to {:?}", clicks, state);
        }
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
