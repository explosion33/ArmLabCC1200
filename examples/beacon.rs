use std::{time::Duration, thread};

use ArmlabRadio::radio_serial::{Radio, prompt_port};

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


const DELAY: u64 = 1;

fn tx(radio: &mut Radio) {
    let mut i = 0;

    loop {
        let msg: String = "beacon | ".to_string() + i.to_string().as_str();
        match radio.transmit(msg.as_bytes()) {
            Ok(_) => {
                println!("{}", msg);
                i += 1;
            },
            Err(_) => {
                println!("Error sending message");
                radio.sync(6);
            }
        };

        thread::sleep(Duration::from_millis(DELAY));
        
    }

}

fn rx(radio: &mut Radio) {
    loop {
        thread::sleep(Duration::from_millis(DELAY));

        let packet: String = match radio.get_packet() {
            Ok(n) => {
                match std::str::from_utf8(&n) {
                    Ok(v) => v.to_string(),
                    Err(_) => {
                        println!("{:#?}", n);
                        continue;
                    },
                }
            },
            Err(_) => {
                println!("Error recieving message");
                let _ = radio.sync(6);
                continue;
            }
        };

        if packet != "" {
            println!("got \"{}\"", packet);
        }
        
    }
}

fn main() {
    //let mut radio = Radio::new_rpi().expect("Error Creating Radio");
    let mut radio = Radio::new(&prompt_port()).expect("Error Creating Radio");

    println!("\n\t0. tx\n\t1. rx");
    let tx_mode: bool = loop {
        break match input!("> ").as_str() {
            "0" => true,
            "1" => false,
            x => {
                println!("error \"{}\" is not a valid input", x);
                continue;
            }
        }
    };

    if tx_mode {
        tx(&mut radio);
    }

    rx(&mut radio);

   
}