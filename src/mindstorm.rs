use byteorder::{LittleEndian, WriteBytesExt};
use libusb;
use std::thread;
use std::time::Duration;

pub struct Mindstorm {
    buff: [u8; 1024],
    device: libusb::DeviceHandle,
}

const EP_IN: u8 = 0x81;
const EP_OUT: u8 = 0x01;

#[derive(Debug)]
pub struct DisconnectError;

impl std::fmt::Display for DisconnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mindstorm disconnected")
    }
}

impl Mindstorm {
    pub fn connect() -> Option<Self> {
        let LEGO_VENDOR_ID = 0x0694;
        let EV3_PRODUCT_ID = 0x0005;
        let context = libusb::Context::new().unwrap();
        let mut device = context.open_device_with_vid_pid(LEGO_VENDOR_ID, EV3_PRODUCT_ID)?;
        if device.kernel_driver_active(0).unwrap() {
            device.detach_kernel_driver(0).unwrap();
        }
        device.set_active_configuration(0).unwrap();
        device.claim_interface(0).unwrap();
        let mut buff: [u8; 1024] = [0; 1024];
        device
            .read_interrupt(EP_IN, &mut buff, Duration::from_secs(5))
            .unwrap();
        Some(Mindstorm { buff, device })
    }

    fn command(&mut self, ops: Vec<u8>) -> Result<&[u8], DisconnectError> {
        match self
            .device
            .write_interrupt(EP_OUT, &make_command(ops), Duration::from_secs(5))
        {
            Ok(_) => {}
            Err(libusb::Error::NoDevice) => return Err(DisconnectError),
            Err(e) => panic!("{}", e),
        }
        match self
            .device
            .read_interrupt(EP_IN, &mut self.buff, Duration::from_secs(5))
        {
            Ok(_) => Ok(&self.buff),
            Err(libusb::Error::NoDevice) => return Err(DisconnectError),
            Err(e) => panic!(e),
        }
    }

    fn nop(&mut self) {
        let OpNop = 0x01;
        self.command(vec![OpNop]);
    }

    /// Run motor A for time, power -100 to 100
    pub fn motor_a(&mut self, power: i32, time: Duration) -> Result<(), DisconnectError> {
        let MOTOR_TYPE = 0xA1;
        let MOTOR_SPEED = 0xA5;
        let MOTOR_START = 0xA6;
        let MOTOR_STOP = 0xA3;
        let MOTOR_MEDIUM = 0x08;
        let mut cmd = vec![MOTOR_TYPE, 0, 1, MOTOR_MEDIUM, MOTOR_SPEED, 0, 1];
        lcx(&mut cmd, power);
        cmd.extend_from_slice(&[MOTOR_START, 0, 1]);
        self.command(cmd)?;
        thread::sleep(time);
        self.command(vec![MOTOR_STOP, 0, 1, 0])?;
        Ok(())
    }
}

fn lcx(buff: &mut Vec<u8>, value: i32) {
    if value >= -32 && value < 0 {
        buff.write_i8(0x3F & (value + 64) as i8);
    } else if value >= 0 && value < 32 {
        buff.write_i8(value as i8);
    } else if value >= -127 && value <= 127 {
        buff.push(0x81);
        buff.write_i8(value as i8);
    } else if value >= -32767 && value <= 32767 {
        buff.push(0x82);
        buff.write_i16::<LittleEndian>(value as i16);
    } else {
        buff.push(0x83);
        buff.write_i32::<LittleEndian>(value);
    }
}

fn make_command(ops: Vec<u8>) -> Vec<u8> {
    let mut buff: Vec<u8> = Vec::new();
    buff.write_u16::<LittleEndian>((ops.len() + 5) as u16)
        .unwrap();
    buff.write_u16::<LittleEndian>(42).unwrap();
    let DIRECT_COMMAND_REPLY = 0x00;
    buff.push(DIRECT_COMMAND_REPLY);
    buff.write_u16::<LittleEndian>(0).unwrap();
    buff.extend_from_slice(&ops);
    buff
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn yes() {
        let mut robot = Mindstorm::connect();
        robot.nop();
    }

}
