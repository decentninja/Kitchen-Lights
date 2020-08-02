use hueclient::bridge::Bridge;
use hueclient::HueError;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};

pub enum Error {
    CouldNotFindBridge,
    RegisterError(HueError),
    LightCommunicationError(HueError),
    NoMagicLight,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CouldNotFindBridge => write!(f, "Could not find bridge"),
            Error::RegisterError(e) => write!(f, "When trying to register {:#?}", e),
            Error::LightCommunicationError(e) => write!(f, "When trying to communicate with light {:#?}", e),
            Error::NoMagicLight => write!(f, "No light named \"Magic Light\" identified, please add one"),
        }
    }
}

pub struct WrappedBridge {
    bridge: Bridge,
}

impl WrappedBridge {
    pub fn connect() -> Result<Self, Error> {
        let mut bridge = Bridge::discover().ok_or(Error::CouldNotFindBridge)?;
        match read_user_string() {
            Some(username) => bridge = bridge.with_user(username),
            None => {
                let user_name = bridge.register_user("kitchen_lights#raspberry").map_err(|e| Error::RegisterError(e))?;
                match File::create("user") {
                    Ok(mut file) => {
                        file.write(user_name.as_bytes()).unwrap();
                    },
                    Err(e) => {
                        eprintln!("Could not open \"user\" file to write username. Will continue, but you'll have to click the button again on next boot. {:?}", e);
                    }
                };
            }
        };
        Ok(Self { bridge })
    }

    /// Read the current brightness of the magic light as a value between 0 and 1.
    /// Will return 0 when the light is off.
    pub fn magic(&self) -> Result<f32, Error> {
        let all_lights = self.bridge.get_all_lights().map_err(|e| Error::LightCommunicationError(e))?;
        let magic = all_lights.iter().find(|light| light.light.name == "Magic Light").ok_or(Error::NoMagicLight)?;
        Ok(match (magic.light.state.on, magic.light.state.bri) {
            (true, Some(bri)) => bri as f32 / std::u8::MAX as f32,
            _ => 0.,
        })
    }
}

fn read_user_string() -> Option<String> {
    let filename = "user";
    let mut file = File::open(filename).ok()?;
    let mut user = String::new();
    file.read_to_string(&mut user).ok()?;
    Some(user)
}
