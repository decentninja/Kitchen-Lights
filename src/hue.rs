use philipshue;
use philipshue::bridge::{self, Bridge};
use std::fs::{File};
use std::io::prelude::*;
use std::io;
use philipshue::errors::{HueError, HueErrorKind, BridgeError};
use std::thread;
use std::time::Duration;

pub struct WrappedBridge {
    bridge: Bridge,
}

impl WrappedBridge {
    pub fn connect() -> Result<Self, HueError> {
        let ip = bridge::discover()?[0].ip().to_owned();
        let bridge = login(&ip)?;
        Ok(Self {
            bridge
        })
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
            })
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
        },
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
            Ok(user) => {
                return user
            },
            Err(HueError(HueErrorKind::BridgeError { error: BridgeError::LinkButtonNotPressed, .. }, _)) => {
                println!("Please, press the link on the bridge. Retrying in 5 seconds.");
                thread::sleep(Duration::from_secs(5));
            },
            Err(e) => {
                panic!("Unexpected error occured: {}", e);
            }
        }
    }
}
