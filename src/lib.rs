#[cfg(any(linux))]
pub mod radio_i2c;

#[cfg(any(not(linux)))]
pub mod radio_serial;