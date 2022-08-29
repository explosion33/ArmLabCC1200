#![allow(dead_code)]

use std::{time::Duration, thread};
use i2cdev::linux::LinuxI2CDevice as I2c;

const IDENT_MSG: &str = "ArmLabCC1200";
const ADDR: u16 = 0x34;
const BACKUP_ADDR: u16 = 0x35;


pub enum ModulationFormat {
    FSK2 = 0x0,
    GFSK2 = 0x1,
    ASK = 0x3,
    FSK4 = 0x4,
    GFSK4 = 0x5,
}

#[derive(Debug)]
pub enum RadioError {
    I2CInitError,
    DeviceDetectError,
    TransmitMsgLen,
    TransmitMsg,
    TransmitError,
    RecieveCmd,
    RecieveReadLen,
    RecieveReadMsg,
    ReadLengthMismatch,
    InvalidArgument,
}

/// Radio (I2C) is a driver for interfacing with STM32 based radios over I2C
/// This library supports I2C communication on all linux based platforms
/// 
/// `Radio::new("I2C_PATH")` to get started
/// 
/// or
/// 
/// `Radio::new_rpi()` for raspberry pi os computers
pub struct Radio {
    pub i2c: I2c,
    packet_wait_delay: u64,
    write_wait_delay: u64,
    
}

impl Radio {    
    /// creates a new Radio object on the given i2c bus and checks
    /// that the device detected is a Radio device
    pub fn new(i2c_path: &str) -> Result<Radio, RadioError> {
        let mut i2c = match I2c::new(i2c_path, ADDR) {
            Ok(n) => {n},
            Err(_) => {
                return Err(RadioError::I2CInitError);                
            }
        };

        if !Radio::check_for_device(&mut i2c) {
            i2c = match I2c::new(i2c_path, BACKUP_ADDR) {
                Ok(n) => n,
                Err(_) => {
                    return Err(RadioError::I2CInitError);   
                }
            };

            if !Radio::check_for_device(&mut i2c) {
                return Err(RadioError::DeviceDetectError);
            }
        }

        Ok(Radio { i2c, packet_wait_delay: 10, write_wait_delay: 10})
    }

    /// creates a new Radio object on the default rpi i2c bus and
    /// checks that the device detected is a Radio device
    pub fn new_rpi() -> Result<Radio, RadioError> {
        return Radio::new("/dev/i2c-1")
    }

    /// switches to the pre-programmed backup address for the Radio
    pub fn use_alt_address(&mut self) -> Result<(), RadioError> {
        self.i2c = match I2c::new("/dev/i2c-1", BACKUP_ADDR) {
            Ok(n) => n,
            Err(_) => {
                return Err(RadioError::I2CInitError);
            }
        };

        Ok(())
    }

    /// sets the delay between requesting a packet from the Radio
    /// and expecting the packet to come through
    /// 
    /// this gives the Radio time to gather the packet before i2c times out
    /// on non clock-stetching devices
    /// 
    /// ## Default
    /// 10 ms
    pub fn set_packet_gather_delay(&mut self, delay: u64) {
        self.packet_wait_delay = delay;
    }

    /// sets the delay between requesting to write a packet to the radio
    /// and sending the packet to the radido
    /// 
    /// this gives the Radio time to ready itself for recieve before i2c times out
    /// on non clock-stetching devices
    /// ## Default
    /// 10 ms
    pub fn set_write_wait_delay(&mut self, delay: u64) {
        self.write_wait_delay = delay;
    }

    /// queries the port to check if the radio is available
    /// 
    /// useful for in-constructor checks
    fn check_for_device(i2c: &mut I2c) -> bool {
        let mut buf: [u8; IDENT_MSG.len()] = [0u8; IDENT_MSG.len()];
        match i2c.read(&mut buf) {
            Ok(_) => {},
            Err(_) => {
                return false;
            },
        };

        return buf == IDENT_MSG.as_bytes();
    }

    /// queries the radio and checks if it is available
    /// 
    /// ## Returns
    /// wheather or not a device was found
    pub fn is_device_available(&mut self) -> bool{
        return Radio::check_for_device(&mut self.i2c);
    }


}

// transmit / recieve
impl Radio {
    /// transmits the given message
    pub fn transmit(&mut self, msg: &[u8]) -> Result<(), RadioError> {

        if msg.len() > u8::MAX as usize {
            return Err(RadioError::InvalidArgument);
        }

        let mut buf: [u8; 5] = [0x01, msg.len() as u8, 0x00, 0x00, 0x00];


        // transmit "transmit" signal 0x01 and number of bytes to expect
        match self.i2c.write(&buf) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::TransmitMsgLen);
            },
        };

        thread::sleep(Duration::from_millis(self.write_wait_delay));

        // transmit message
        match self.i2c.write(&msg) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::TransmitMsg);
            },
        }
        
        Ok(())
    }

    /// gets the most recent packet stored on the Radio
    /// 
    /// ## Returns
    /// Vec/<u8/> with the byte data of the packet
    /// 
    /// empty Vec/<u8/> if no available packet was found
    pub fn get_packet(&mut self) -> Result<Vec<u8>, RadioError> {
        // send read command
        let msg: [u8; 5] = [0x02, 0x00, 0x00, 0x00, 0x00];
        match self.i2c.write(&msg) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::RecieveCmd);
            },
        };

        // give radio time to collect packets
        thread::sleep(Duration::from_millis(self.packet_wait_delay));

        // read number of expected bytes
        let mut buf: [u8; 1] = [0u8; 1];
        match self.i2c.read(&mut buf) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::RecieveReadLen);
            },
        };
        let msg_size: usize = buf[0] as usize; 

        if msg_size == 0 {
            let out: Vec<u8> = Vec::new();
            return Ok(out);
        }

        // initialize &[u8] on the heap
        // this allows for a &[u8] with size msg_size which is not known at compile time
        let mut buf2: Box<[u8]> = vec![0; msg_size].into_boxed_slice();

        match self.i2c.read(&mut buf2) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::RecieveReadMsg);
            },
        };

        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(&buf2);

        Ok(out)
    }

    /// sends a reset command to reset the onboard Radio chip
    pub fn radio_reset(&mut self) -> Result<(), RadioError> {
        match self.i2c.write(&[9,0,0,0,0]) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::TransmitMsgLen);
            },
        };
        Ok(())
    }

    /// sends a reset command to perform a soft reset on the entire board
    pub fn soft_reset(&mut self) -> Result<(), RadioError> {
        match self.i2c.write(&[10,0,0,0,0]) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::TransmitMsgLen);
            },
        };
        Ok(())
    }

    /// performs a hard reset using the boards reset pin
    pub fn reset(&mut self) -> Result<(), RadioError> {
        println!("placeholder function, no action occured");
        Ok(())
    }

}

// change radio settings
impl Radio {
    /// helper function to write the byte data of a f32 over i2c
    fn set_float_val(&mut self, cmd: u8, val: f32) -> Result<(), RadioError> {
        let bytes = val.to_ne_bytes();
        let mut buf: [u8; 5] = [cmd, 0x00, 0x00, 0x00, 0x00];
        buf[1] = bytes[0];
        buf[2] = bytes[1];
        buf[3] = bytes[2];
        buf[4] = bytes[3];

        match self.i2c.write(&buf) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::TransmitError);
            },
        };

        return Ok(());
    }

    /// attempts to set the frequency of the radio
    /// 
    /// there is a chance the radio rejects the value if it is invalid
    pub fn set_frequency(&mut self, frequency: f32) -> Result<(), RadioError> {
        return self.set_float_val(0x03, frequency);
    }

    /// attempts to set the tx gain of the radio
    /// 
    /// there is a chance the radio rejects the value if it is invalid
    pub fn set_power(&mut self, power: f32) -> Result<(), RadioError> {
        return self.set_float_val(0x04, power);
    }

    /// attempts to set the FSK bandwith of the radio
    /// 
    /// there is a chance the radio rejects the value if it is invalid
    pub fn set_deviation(&mut self, deviation: f32) -> Result<(), RadioError> {
        return self.set_float_val(0x05, deviation);
    }

    /// attempts to set the symbol rate of the radio
    /// 
    /// there is a chance the radio rejects the value if it is invalid
    pub fn set_symbol_rate(&mut self, symbol_rate: f32) -> Result<(), RadioError> {
        return self.set_float_val(0x06, symbol_rate);
    }

    /// attempts to set the recieve filter of the radio
    /// 
    /// there is a chance the radio rejects the value if it is invalid
    pub fn set_rx_filter(&mut self, rx_filter: f32) -> Result<(), RadioError> {
        return self.set_float_val(0x07, rx_filter);
    }

    /// sets the modulation mode of the radio
    pub fn set_modulation(&mut self, mode: ModulationFormat) -> Result<(), RadioError> {
        let buf: [u8; 5] = [0x08, mode as u8, 0x00, 0x00, 0x00];

        match self.i2c.write(&buf) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::TransmitError);
            },
        };

        return Ok(());
    }
}
