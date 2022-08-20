# ArmLabCC1200
Libraries for interfacring with Armstrong Labs CC1200 based radios

<br>

**Download with crates.io**

https://crates.io/crates/ArmlabRadio

## Purpose
ArmLabCC1200 contains a set of libraries for interfacing with STM32 controlled CC1200 radios. An STM32 wired to a CC1200 chip, and flashed with the following [code](https://github.com/explosion33/CC1200stm32) will be able to be controlled by
* any linux device over I2C or SPI
* any device over serial

## Whats Available
* Rust source code to interface with the device over I2C on linux (radio_i2c.rs)
* Rust source code to interface with the device over serial (radio_serial.rs)
* Basic example (main.rs)
* [stm32 source code](https://github.com/explosion33/CC1200stm32)

## Whats Coming
* More exposed features on the radio
* SPI Interfacing options
* Rust Library on crates.io
* Platform independent library for Rust
* Platform independent library for C / C++


## Design
[Latest design specifications](https://ethana.notion.site/CC1200-Radio-06d342126b2041b483d045ed1dcfd178)


## Use
I2C (Linux)
```
use ArmlabRadio::radio_i2c::Radio;

fn main () {
    let mut radio: Radio = Radio::new_rpi().unrwap();
    
    radio.transmit(b"test message");
    let packet = radio.get_packet();

    if packet == "" {
        println!("no message recieved");
    }

    println!("got message: \"{}\"", packet);
}
```

Serial
```
use ArmlabRadio::radio_serial::Radio;

fn main () {
    let mut radio: Radio = Radio::new("COM 4").unrwap();
    
    radio.transmit(b"test message");
    let packet = radio.get_packet();

    if packet == "" {
        println!("no message recieved");
    }

    println!("got message: \"{}\"", packet);
}
```