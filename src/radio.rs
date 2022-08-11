use std::{time::Duration, thread};
use rppal::i2c::I2c;

const IDENT_MSG: &str = "ArmLabCC1200";
const ADDR: u16 = 0x34;
const BACKUP_ADDR: u16 = 0x35;

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



pub struct Radio {
    pub i2c: I2c,
    packet_wait_delay: u64,
    
}

#[allow(dead_code)]
impl Radio {
    pub fn new() -> Result<Radio, RadioError> {
        let mut i2c = match I2c::new() {
            Ok(n) => {n},
            Err(_) => {
                return Err(RadioError::I2CInitError);
            }
        };

        // set device address
        match i2c.set_slave_address(ADDR) {
            Ok(_) => {},
            Err(_) => {return Err(RadioError::I2CInitError)},   
        }

        // check if device is on address
        // if its not attempt for backup address
        if !Radio::check_for_device(&mut i2c) {

            match i2c.set_slave_address(BACKUP_ADDR) {
                Ok(_) => {},
                Err(_) => {return Err(RadioError::I2CInitError)},   
            }
            
            if !Radio::check_for_device(&mut i2c) {
                return Err(RadioError::DeviceDetectError);
            }
        }

        Ok(Radio { i2c, packet_wait_delay: 10 })
    }

    pub fn use_alt_address(&mut self, address: u16) -> Result<(), RadioError> {
        match self.i2c.set_slave_address(address) {
            Ok(_) => {Ok(())},
            Err(_) => {Err(RadioError::I2CInitError)},
        }
    }

    pub fn set_packet_gather_delay(&mut self, delay: u64) {
        self.packet_wait_delay = delay;
    }

    pub fn transmit(&mut self, msg: &[u8]) -> Result<(), RadioError> {

        if msg.len() > u8::MAX as usize {
            return Err(RadioError::InvalidArgument);
        }

        let mut buf: [u8; 5] = [0x01, 0x00, 0x00, 0x00, 0x00];
        buf[1] = msg.len() as u8;


        // transmit "transmit" signal 0x01 and number of bytes to expect
        match self.i2c.write(&buf) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::TransmitMsgLen);
            },
        };

        // transmit message
        match self.i2c.write(&msg) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::TransmitMsg);
            },
        }
        
        Ok(())
    }

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
            Ok(n) => {
                if n != 1 {
                    println!("read length mismatch expected 1 byte, read {}", n);
                    return Err(RadioError::ReadLengthMismatch);
                }
            },
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
            Ok(n) => {
                if n != msg_size {
                    println!("read length mismatch expected {} bytes, read {}", msg_size, n);
                    return Err(RadioError::ReadLengthMismatch);
                }
            },
            Err(_) => {
                return Err(RadioError::RecieveReadMsg);
            },
        };

        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(&buf2);

        Ok(out)
    }

    pub fn set_frequency(&mut self, freq: f32) -> Result<(), RadioError> {
        let bytes = freq.to_ne_bytes();
        let mut buf: [u8; 5] = [0x03, 0x00, 0x00, 0x00, 0x00];
        buf[1] = bytes[0];
        buf[2] = bytes[1];
        buf[3] = bytes[2];
        buf[4] = bytes[3];

        // transmit "transmit" signal 0x01 and number of bytes to expect
        match self.i2c.write(&buf) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::TransmitError);
            },
        };

        return Ok(());
    }

    pub fn set_power(&mut self, power: f32) -> Result<(), RadioError> {
        let bytes = power.to_ne_bytes();
        let mut buf: [u8; 5] = [0x04, 0x00, 0x00, 0x00, 0x00];
        buf[1] = bytes[0];
        buf[2] = bytes[1];
        buf[3] = bytes[2];
        buf[4] = bytes[3];

        // transmit "transmit" signal 0x01 and number of bytes to expect
        match self.i2c.write(&buf) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::TransmitError);
            },
        };

        return Ok(());
    }


    fn check_for_device(i2c: &mut I2c) -> bool {
        let mut buf: [u8; IDENT_MSG.len()] = [0u8; IDENT_MSG.len()];
        match i2c.read(&mut buf) {
            Ok(n) => {
                if n != IDENT_MSG.len() {
                    println!("read length mismatch expected {} byte, read {}", IDENT_MSG.len(), n);
                    return false;
                }
            },
            Err(_) => {
                return false;
            },
        };

        return buf == IDENT_MSG.as_bytes();
    }

    pub fn is_device_available(&mut self) -> bool{
        return Radio::check_for_device(&mut self.i2c);
    }


}