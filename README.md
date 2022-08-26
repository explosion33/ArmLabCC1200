# ArmLabCC1200
Libraries for interfacring with Armstrong Labs CC1200 based radios

<br>

**Download with crates.io**

https://crates.io/crates/ArmlabRadio

## Purpose
ArmLabCC1200 contains a set of libraries for interfacing with STM32 controlled CC1200 radios. An STM32 wired to a CC1200 chip, and flashed with the following [code](https://github.com/explosion33/CC1200stm32) will be able to be controlled by
* any linux device over I2C
* any device over serial

## Whats Available
* Rust source code to interface with the device over I2C on linux (radio_i2c.rs)
* Rust source code to interface with the device over serial (radio_serial.rs)
* [Library](https://crates.io/crates/ArmlabRadio) published on crates.io
* [stm32 source code](https://github.com/explosion33/CC1200stm32)
* Commands to change basic radio settings
* interactive command line [example](https://github.com/explosion33/ArmLabCC1200/blob/main/examples/terminal.rs)
    * ```cargo run --example terminal```
    * ```cargo run --features i2clib --example terminal```
* interactive continuous rx/tx  [example](https://github.com/explosion33/ArmLabCC1200/blob/main/examples/beacon.rs)
    * ```cargo run --example beacon```
    * ```cargo run --features i2clib --example beacon```
* Serial radio device, auto detection

## Whats Coming
* More exposed features on the radio
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
    
    radio.transmit(b"test message").expect("transmit error");
    let packet = radio.get_packet().expect("read error");

    println!("got message: \"{:?}\"", packet);
}
```

Serial
```
use ArmlabRadio::radio_serial::Radio;

fn main () {
    let mut radio: Radio = Radio::new("COM 4").unrwap();
    
    radio.transmit(b"test message").expect("transmit error");
    let packet = radio.get_packet().expect("read error");

    println!("got message: \"{:?}\"", packet);
}
```

## Speed / Power
### Serial <-> Serial Speed
Using two radios positioned relatively close, one with a 8in whip antenna and the other with a 16in whip antenna I was able to achieve 60000 continuous transmissions containing  on average 13 bytes each at a rate of about 1000 packets in 15 seconds

This leads to a transmit rate of ~7kbps, which would be improved using:

* Faster communication methods (I2C)
* Radio tuning for given antenna
* Antennas matching transmit frequency
* Sending more bytes per given message
    * Reduces extra data communicated via serial for each send / recieve