#![allow(dead_code)]

use serialport::SerialPort;

use std::time::Duration;

const IDENT_MSG: &str = "ArmLabCC1200";

pub enum ModulationFormat {
    FSK2 = 0x0,
    GFSK2 = 0x1,
    ASK = 0x3,
    FSK4 = 0x4,
    GFSK4 = 0x5,
}

#[derive(Debug)]
pub enum RadioError {
    PortOpenError,
    DevciceDetectError,
    InvalidArgument,
    ReadLenError,
    ReadError,
    WriteError,
    WriteLenError,
}

pub struct Radio {
    pub port: Box<dyn SerialPort>,
}

// init
impl Radio {
    pub fn new(path: &str) -> Result<Radio, RadioError> {
        let mut port = match serialport::new(path, 115200)
            .timeout(Duration::from_millis(500))
            .open() {
                Ok(n) => n,
                Err(_) => {return Err(RadioError::PortOpenError)}
        };


        if !Radio::check_for_device(&mut port) {
            return Err(RadioError::DevciceDetectError);
        }

        Ok(Radio {port})
    }

    pub fn is_device_available(&mut self) -> bool {
        return Radio::check_for_device(&mut self.port);
    }

    fn check_for_device(port: &mut Box<dyn SerialPort>) -> bool {
        Radio::clear_read(port);
        let cmd: [u8; 6] = [0,0,0,0,0, '\n' as u8];
        match port.write_all(&cmd) {
            Ok(_) => {},
            Err(_) => {
                return false;
            }
        };

        let mut buf: [u8; IDENT_MSG.len()] = [0u8; IDENT_MSG.len()];
        match port.read_exact(&mut buf) {
            Ok(_) => {},
            Err(_) => {
                return false
            },
        };

        if buf != IDENT_MSG.as_bytes() {
            for i in 0..IDENT_MSG.len() {
                println!("{}, {}", buf[i], IDENT_MSG.as_bytes()[i]);
            }
            return false;
        }

        return true;
    }

    fn clear_read(port: &mut Box<dyn SerialPort>) -> bool {
        match port.clear(serialport::ClearBuffer::Input) {
            Ok(_) => {true},
            Err(_) => {false},
        }
        
    }

    fn sync(&mut self) -> bool {
        let res = Radio::clear_read(&mut self.port);
        res
    }

}

// transmit / recieve
impl Radio {
    fn write_bytes(&mut self, data: &[u8]) -> Result<(), RadioError> {
        // create temp vector to hold both message and newline
        let mut middle: Vec<u8> = vec![0; 0];
        middle.extend_from_slice(data);
        middle.push('\n' as u8);

        // code should always be safe, we are trimming to the data that was added
        // plus we check for the condition where it would fail
        unsafe {
            if data.len() + 1 <= middle.capacity() {
                middle.set_len(data.len() + 1);
            }
        }

        let buf: Box<[u8]> = middle.into_boxed_slice();

        match self.port.write_all(&buf) {
            Ok(_) => Ok(()),
            Err(_) => Err(RadioError::WriteError),
        }
    }

    pub fn transmit(&mut self, msg: &[u8]) -> Result<(), RadioError> {
        self.sync();
        if msg.len() > u8::MAX as usize {
            return Err(RadioError::InvalidArgument);
        }

        let cmd:[u8; 5] = [0x1, msg.len() as u8, 0x0, 0x0, 0x0];
        match self.write_bytes(&cmd) {
            Ok(_) => {},
            Err(_) => {return Err(RadioError::WriteError)},
        };

        
        match self.write_bytes(&msg) {
            Ok(_) => {},
            Err(_) => {return Err(RadioError::WriteError)},
        };

        let mut buf: [u8; 100] = [0; 100];

        self.port.read(&mut buf);

        for val in buf {
            if val == 0 {
                break;
            }
            print!("{} ", val);
        }
        println!();

        Ok(())
    }

    pub fn get_packet(&mut self) -> Result<Vec<u8>, RadioError> {
        let cmd: [u8; 5] = [0x2, 0x0, 0x0, 0x0, 0x0];
        self.write_bytes(&cmd).unwrap();

        
        let mut buf: [u8; 1] = [0x0];
        self.port.read_exact(&mut buf).unwrap();
        let msg_size = buf[0] as usize;

        if msg_size == 0 {
            return Ok(Vec::new());
        }
        

        let mut buf2: Box<[u8]> = vec![0; msg_size].into_boxed_slice();
        self.port.read_exact(&mut buf2).unwrap();

        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(&buf2);

        Ok(out)
    }
}

impl Radio {
    fn set_float_val(&mut self, cmd: u8, val: f32) -> Result<(), RadioError> {
        let bytes = val.to_ne_bytes();
        let mut buf: [u8; 5] = [cmd, 0x00, 0x00, 0x00, 0x00];
        buf[1] = bytes[0];
        buf[2] = bytes[1];
        buf[3] = bytes[2];
        buf[4] = bytes[3];

        match self.write_bytes(&buf) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::WriteError);
            },
        };

        return Ok(());
    }

    pub fn set_frequency(&mut self, frequency: f32) -> Result<(), RadioError> {
        return self.set_float_val(0x03, frequency);
    }

    pub fn set_power(&mut self, power: f32) -> Result<(), RadioError> {
        return self.set_float_val(0x04, power);
    }

    pub fn set_deviation(&mut self, deviation: f32) -> Result<(), RadioError> {
        return self.set_float_val(0x05, deviation);
    }

    pub fn set_symbol_rate(&mut self, symbol_rate: f32) -> Result<(), RadioError> {
        return self.set_float_val(0x06, symbol_rate);
    }

    pub fn set_rx_filter(&mut self, rx_filter: f32) -> Result<(), RadioError> {
        return self.set_float_val(0x07, rx_filter);
    }

    pub fn set_modulation(&mut self, mode: ModulationFormat) -> Result<(), RadioError> {
        let buf: [u8; 5] = [0x08, mode as u8, 0x00, 0x00, 0x00];

        match self.write_bytes(&buf) {
            Ok(_) => {},
            Err(_) => {
                return Err(RadioError::WriteError);
            },
        };

        return Ok(());
    }
}
