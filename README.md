# ArmLabCC1200
Libraries for interfacring with Armstrong Labs CC1200 based radios

## Purpose
ArmLabCC1200 contains a set of libraries for interfacing with STM32 controlled CC1200 radios

an STM32 wired to a CC1200 chip, and flashed with the following [code](https://github.com/explosion33/CC1200stm32) will be able to be controlled by
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
