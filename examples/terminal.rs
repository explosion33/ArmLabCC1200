use std::{time::Duration, thread};

#[cfg(feature="i2clib")]
use ArmlabRadio::radio_i2c::{Radio, ModulationFormat};

#[cfg(not(feature="i2clib"))]
use ArmlabRadio::radio_serial::{Radio, ModulationFormat, get_open_ports, get_radio_ports};


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
    let mut radios = get_radio_ports().unwrap();
    

    let port: String = match radios.len() {
        1 => {
            println!("Found one radio on {}", radios[0]);
            radios[0].clone()
        }
        0 | _ => {
            if radios.len() == 0 {
                println!("Radio could not be automatically detected");
                radios = get_open_ports().unwrap();
            }
            else {
                println!("Multiple radios detected");
            }

            println!("Please select a port: ");
            let mut i: usize = 0;
            for port in &radios {
                println!("\t{}. {}", i, port);
                i += 1;
            }

            loop {
                let res = input!("> ");
                
                let val: usize = match res.parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error \"{}\" is not a valid selection", res);
                        continue;
                    }
                };

                if val >= radios.len() {
                    println!("Error \"{}\" is not a valid selection", res);
                    continue;
                }
                break radios[val].clone();
            }
        }

    };


    #[cfg(feature="i2clib")]
    let mut radio = Radio::new_rpi().expect("Error Creating Radio");
    #[cfg(not(feature="i2clib"))]
    let mut radio = Radio::new(&port).expect("Error Creating Radio");

    loop {
        match input!("> ").as_str() {
            "write" |
            "w" => {
                let msg = input!("Enter Message> ");
                match radio.transmit(msg.as_bytes()) {
                    Ok(_) => {println!("\"{}\"", msg)},
                    Err(_) => {println!("Error transmitting")},
                };
            }
            "read" |
            "r" => {
                match radio.get_packet() {
                    Ok(n) => {
                        match std::str::from_utf8(&n) {
                            Ok(v) => println!("\"{}\"", v),
                            Err(_) => println!("{:#?}", n),
                        };
                    },
                    Err(n) => {println!("Error getting packet | {:?}", n)},
                };
            }
            
            "f" |
            "frequency" => {
                let inp = input!("Enter Message> ");
                let val: f32 = match inp.parse::<f32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid Parameter");
                        continue;
                    },
                };

                match radio.set_frequency(val) {
					Ok(_) => {println!("Value set | {}", val)},
					Err(_) => {println!("Error setting value")},
				}

            },

            "p" |
            "power" => {
                let inp = input!("Enter Value> ");
                let val: f32 = match inp.parse::<f32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid Parameter");
                        continue;
                    },
                };
                match radio.set_power(val) {
					Ok(_) => {println!("Value set | {}", val)},
					Err(_) => {println!("Error setting value")},
				}

            },
            
            "d" |
            "deviation" => {
                let inp = input!("Enter Value> ");
                let val: f32 = match inp.parse::<f32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid Parameter");
                        continue;
                    },
                };
                match radio.set_deviation(val) {
					Ok(_) => {println!("Value set | {}", val)},
					Err(_) => {println!("Error setting value")},
				}

            },
            
            "sr" |
            "symbol rate" => {
                let inp = input!("Enter Value> ");
                let val: f32 = match inp.parse::<f32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid Parameter");
                        continue;
                    },
                };
                match radio.set_symbol_rate(val) {
					Ok(_) => {println!("Value set | {}", val)},
					Err(_) => {println!("Error setting value")},
				}

            },
            
            "rxf" |
            "rx filter" => {
                let inp = input!("Enter Value> ");
                let val: f32 = match inp.parse::<f32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid Parameter");
                        continue;
                    },
                };
                match radio.set_rx_filter(val) {
					Ok(_) => {println!("Value set | {}", val)},
					Err(_) => {println!("Error setting value")},
				}

            },

            "m" |
            "modulation" => {
                println!("\tFSK2  (0)");
                println!("\tGFSK2 (1)");
                println!("\tASK   (3)");
                println!("\tFSK4  (4)");
                println!("\tGFSK4 (5)");
                let mode = match input!("> ").as_str() {
                    "0" | "FSK2" => {ModulationFormat::FSK2},
                    "1" | "GFSK2" => {ModulationFormat::GFSK2},
                    "3" | "ASK" => {ModulationFormat::ASK},
                    "4" | "FSK4" => {ModulationFormat::FSK4},
                    "5" | "GFSK4" => {ModulationFormat::GFSK4},
                    _ => {
                        println!("Invalid Argument");
                        continue;
                    },
                };

                match radio.set_modulation(mode) {
                    Ok(_) => {println!("Value set")},
                    Err(_) => {println!("Error setting value")},
                };
            },

            "rr" |
            "radio reset" => {
                match radio.radio_reset() {
                    Ok(_) => {println!("sent radio reset command")},
					Err(_) => {println!("Error sending command")},
				};
            },

            "rs" |
            "soft reset" => {
                match radio.soft_reset() {
                    Ok(_) => {
                        println!("sent soft reset command, re-initializing");

                        // drops old radio, which closes the serialport
                        // continuously try to re-init port, windows / rust takes a while ~500ms
                        // to re-detect port
                        #[cfg(not(feature="i2clib"))]
                        drop(radio);
                        
                        #[cfg(not(feature="i2clib"))]
                        {
                        radio = loop {
                            match Radio::new_bare(&port) {
                                Ok(n) => {break n;},
                                Err(_) => {},
                            };

                            thread::sleep(Duration::from_millis(50));
                        };

                        println!("reset radio, re-initialized serial coms");
                        }
                    },
					Err(_) => {println!("Error sending command")},
				};
            },

            "rh" |
            "hard reset" => {
                #[cfg(feature="i2clib")]
                {
                    match radio.reset() {
                        Ok(_) => {println!("triggered hardware reset")},
                        Err(_) => {println!("Error performing reset")},
                    };
                }
                #[cfg(not(feature="i2clib"))]
                println!("hard reset is not supported with serial")
            }
            
            "h" |
            "help" => {
                println!("write (w)\n\ttransmits a message");
                println!("read (r)\n\treads a message from radio \"\" if there is none");
                
                println!("frequency (f)\n\tsets the radios operating frequency");
                println!("power (p)\n\tsets the radios TX power");
                println!("deviation (d)\n\tsets the radios FSK deviation");
                println!("symbol rate (sr)\n\tsets the radios symbol rate for TX and RX");
                println!("rx filter (rxf)\n\tsets the RX bandwith filter");
                println!("modulation (m)\n\tsets the radios modulation format");

                println!("radio reset (rr)\n\tattempts to reset the onboard CC1200 radio");
                println!("soft reset (rs)\n\tattempts a software reset of the entire board");
                println!("hard reset (rh)\n\tperforms a hard reset on the entire radio");

                println!("help (h)\n\tshows available commands");
            }
            
            _ => {println!("Invalid Command | \"help\" to see available commands")}
        };
        println!();
    }
}