use crate::radio::{Radio, RadioError};
mod radio;

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


fn main() {
    let mut radio = Radio::new().unwrap();

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
            _ => {},
        }
    }
}