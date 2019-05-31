use failure::Fail;
use ron;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

use super::state;

#[derive(Deserialize)]
pub struct Config {
    pub pin: u64,
    pub poll_time_ms: u64,
    pub state: state::Config,
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Could not open config.ron: {}", _0)]
    File(std::io::Error),
    #[fail(display = "Could not parse config.ron: {}", _0)]
    Format(ron::de::Error),
}

pub fn read_from_file() -> Result<Config, Error> {
    let filename = "config.ron";
    let file = File::open(filename).map_err(Error::File)?;
    let reader = BufReader::new(file);
    ron::de::from_reader(reader).map_err(Error::Format)
}
