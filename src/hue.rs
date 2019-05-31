use philipshue;
use philipshue::bridge::{self, Bridge};
use philipshue::errors::{BridgeError, HueError, HueErrorKind};
use std::fs::File;
use std::io;
use std::fmt;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

pub struct WrappedBridge {
    bridge: Bridge,
}

pub enum SetupError {
    HueError(HueError),
    NoBridges,
    LoginIoError(io::Error)
}

impl std::convert::From<HueError> for SetupError {
    fn from(e: HueError) -> Self {
        SetupError::HueError(e)
    }
}

impl std::convert::From<io::Error> for SetupError {
    fn from(e: io::Error) -> Self {
        SetupError::LoginIoError(e)
    }
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SetupError::HueError(e) => write!(f, "Hue Error! {}", e),
            SetupError::NoBridges => write!(f, "No Bridges found on network!"),
            SetupError::LoginIoError(e) => write!(f, "Error while saving bridge login data to disk! {}", e),
        }
    }
}

impl WrappedBridge {
    pub fn connect() -> Result<Self, SetupError> {
        let bridges = bridge::discover()?;
        if bridges.len() == 0 {
            return Err(SetupError::NoBridges)
        }
        let ip = bridge::discover()?[0].ip().to_owned();
        let bridge = login(&ip)?;
        Ok(Self { bridge })
    }

    /// Read the current brightness of the magic light as a value between 0 and 1.
    /// Will return 0 when the light is off.
    pub fn magic(&self) -> Result<f32, HueError> {
        loop {
            let lights = self.bridge.get_all_lights()?;
            let magic = match lights.iter().find(|light| light.1.name == "Magic Light") {
                Some(magic) => magic.1,
                None => {
                    println!("No light named \"Magic Light\" found. Rename one light to that. Retrying in 5 seconds.");
                    thread::sleep(Duration::from_secs(5));
                    continue;
                }
            };
            return Ok(if magic.state.on {
                magic.state.bri as f32 / std::u8::MAX as f32
            } else {
                0.
            });
        }
    }
}

fn login(ip: &str) -> Result<Bridge, io::Error> {
    let filename = "user.json";
    let user = match File::open(filename) {
        Ok(mut file) => {
            let mut user = String::new();
            file.read_to_string(&mut user)?;
            user
        }
        Err(e) => {
            println!("Tried to read login file. {}", e);
            println!("Will try to register instead");
            let user = register(ip);
            let mut file = File::create(filename)?;
            file.write(user.as_bytes())?;
            user
        }
    };
    Ok(Bridge::new(ip, user))
}

fn register(ip: &str) -> String {
    loop {
        match bridge::register_user(ip, "encrypt.wave@gmail.com") {
            Ok(user) => return user,
            Err(HueError(
                HueErrorKind::BridgeError {
                    error: BridgeError::LinkButtonNotPressed,
                    ..
                },
                _,
            )) => {
                println!("Please, press the link on the bridge. Retrying in 5 seconds.");
                thread::sleep(Duration::from_secs(5));
            }
            Err(e) => {
                panic!("Unexpected error occured: {}", e);
            }
        }
    }
}
