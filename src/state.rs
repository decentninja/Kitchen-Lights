use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Off,
    HalfUp,
    Full,
    HalfDown,
}
use State::*;

#[derive(Deserialize)]
pub struct Config {
    off: f32,
    full: f32
}

impl State {
    pub fn new() -> Self {
        Off
    }

    fn next(self) -> Self {
        match self {
            Off => HalfUp,
            HalfUp => Full,
            Full => HalfDown,
            HalfDown => Off,
        }
    }

    /// Sets that state to a new value and output the number of required clicks to get there.
    pub fn set(&mut self, value: f32, config: &Config) -> u32 {
        let target = if value <= config.off {
            Off
        } else if value <= config.full {
            match self {
                Off | HalfUp => HalfUp,
                Full | HalfDown => HalfDown,
            }
        } else {
            Full
        };
        let mut a = 0;
        while *self != target {
            a += 1;
            *self = self.next();
        }
        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(from: State, value: f32, to: State, should: u32) {
        let mut state = from;
        let config = Config {
            off: 0.2,
            full: 0.8,
        };
        let clicks = state.set(value, &config);
        assert_eq!((from, to, should), (from, state, clicks));
    }

    #[test]
    fn nothing() {
        test(Off, 0., Off, 0);
        test(HalfUp, 0.5, HalfUp, 0);
        test(Full, 1., Full, 0);
        test(HalfDown, 0.5, HalfDown, 0);
    }

    #[test]
    fn random_jumps() {
        test(Off, 1., Full, 2);
        test(Full, 0., Off, 2);
        test(HalfDown, 1., Full, 3);
    }

}
