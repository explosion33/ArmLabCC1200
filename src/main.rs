#[cfg(any(linux))]
use crate::radio_i2c::{Radio, ModulationFormat};
#[cfg(any(linux))]
mod radio_i2c;

#[cfg(any(not(linux)))]
use crate::radio_serial::{Radio};
#[cfg(any(not(linux)))]
mod radio_serial;

use std::{thread, time::Duration};

macro_rules! input {
    {} => {{
        input!("")
    }};

    ($a:expr) => {{
        use std::io;
        use std::io::Write;

        print!("{}", $a);
        let _ = io::stdout().flush();

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Error reading from stdin");
        line.trim().to_string()
    }};
}

// linux i2c examples
#[cfg(any(linux))]
fn run_async_coms() {
    let mut radio = Radio::new_rpi().unwrap();
    let mut radio2 = Radio::new_rpi().unwrap();

    radio2.use_alt_address(0x35).expect("could not change address");
    if !radio2.is_device_available() {
        panic!("Radio 2 (0x35) was not found");
    }

    thread::spawn(move || {
        loop {
            radio.transmit("heartbeat".as_bytes()).expect("transmit error");

            let packet = radio.get_packet().expect("Recieve Error");

            match std::str::from_utf8(&packet) {
                Ok(v) => {
                    if v == "heartbeat" {
                        break;
                    }
                },
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        }

        radio.transmit("heartbeat".as_bytes()).expect("transmit error");

        let mut i: i32 = 0;
        loop {
            let msg: String = "test msg ".to_string() +i.to_string().as_str();
            radio.transmit(msg.as_bytes()).expect("transmit error");
            i += 1
        }
    });

    loop {
        radio2.transmit("heartbeat".as_bytes()).expect("transmit error");

        let packet = radio2.get_packet().expect("Recieve Error");

        match std::str::from_utf8(&packet) {
            Ok(v) => {
                if v == "heartbeat" {
                    break;
                }
            },
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    loop {
        let packet = radio2.get_packet().expect("Recieve Error");

        let s = match std::str::from_utf8(&packet) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        if s == "" {
            continue;
        }

        println!("got packet \"{}\"", s);
    }
}

#[cfg(any(linux))]
fn tx(msg: &str, delay_ms: u64) {
    let mut radio = Radio::new_rpi().unwrap();
    loop {
        radio.transmit(msg.as_bytes()).expect("tx failure");

        thread::sleep(Duration::from_millis(delay_ms))
    }
}

fn run_cmd_int(radio: &mut Radio) {
    loop {
        match input!("w or r: ").as_str() {
            "w" => {
                let msg = input!("Enter msg: ");
                radio.transmit(msg.as_bytes()).expect("transmit error");
            },
            "r" => {
                let packet = radio.get_packet().expect("Recieve Error");

                let s = match std::str::from_utf8(&packet) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

                println!("got packet \"{}\"", s);
            },
            "144" => {
                radio.set_frequency(144e6).expect("error");
            }
            _ => {},
        }
    }
}


fn main() {
    //run_async_coms();
    //run_cmd_int();
    //tx("test beacon msg", 100);
    
    //let mut radio = Radio::new_rpi().unwrap();
    let mut radio = Radio::new("COM4").unwrap();

    
    run_cmd_int(&mut radio);

    //radio.set_frequency(101.1).expect("error");
    //radio.set_power(101.1).expect("error");
    //radio.set_deviation(101.1).expect("error");
    //radio.set_symbol_rate(101.1).expect("error");
    //radio.set_rx_filter(101.1).expect("error");
    //radio.set_modulation(ModulationFormat::GFSK4).expect("Error");


}