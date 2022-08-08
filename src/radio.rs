use std::{time::Duration, thread};
use rppal::i2c::I2c;


#[derive(Debug)]
pub enum RadioError {
    I2CInitError,
    TransmitMsgLen,
    TransmitMsg,
    RecieveCmd,
    RecieveReadLen,
    RecieveReadMsg,
    ReadLengthMismatch,
    InvalidArgument,
}



pub struct Radio {
    i2c: I2c,
    packet_wait_delay: u64,
    
}


impl Radio {
    pub fn new() -> Result<Radio, RadioError> {
        let mut i2c = match I2c::new() {
            Ok(n) => {n},
            Err(_) => {
                return Err(RadioError::I2CInitError);
            }
        };

        match i2c.set_slave_address(0x34) {
            Ok(_) => {},
            Err(_) => {return Err(RadioError::I2CInitError)},   
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

        let mut buf: [u8; 2] = [0x01, 0x00];
        buf[1] = msg.len() as u8;


        // transmit "transmit" signal 0x01 and number of bytes to expect
        match self.i2c.write(&buf) {
            Ok(_) => {},
            Err(n) => {
                return Err(RadioError::TransmitMsgLen);
            },
        };

        // transmit message
        match self.i2c.write(&msg) {
            Ok(_) => {},
            Err(n) => {
                return Err(RadioError::TransmitMsg);
            },
        }
        
        Ok(())
    }

    pub fn get_packet(&mut self) -> Result<Vec<u8>, RadioError> {
        // send read command
        let msg: [u8; 2] = [0x02, 0x00];
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
            Err(n) => {
                return Err(RadioError::RecieveReadMsg);
            },
        };

        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(&buf2);

        Ok(out)
    }
}