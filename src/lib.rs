//! This is a platform agnostic Rust driver for the TCS3472 RGB color light to
//! digital converter with IR filter, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device.
//! - Enable/disable the RGB converter.
//! - Set RGB converter gain.
//! - Enable/disable the RGB converter interrupt generation.
//! - Set the RGB converter interrupt clear channel low/high thresholds.
//! - Set the RGB converter interrupt persistence.
//! - Set the number of integration cycles.
//! - Enable/disable the wait feature.
//! - Set the number of wait time cycles.
//! - Enable/disable the *wait long* setting.
//! - Read status of RGB converter.
//! - Read the clear (unfiltered) channel measurement.
//! - Read the red channel measurement.
//! - Read the green channel measurement.
//! - Read the blue channel measurement.
//! - Read the measurement of all channels at once.
//! - Read the device ID.
//!
//! ## The device
//!
//! The TCS3472 device provides a digital return of red, green, blue (RGB), and
//! clear light sensing values. An IR blocking filter, integrated on-chip and
//! localized to the color sensing photodiodes, minimizes the IR spectral
//! component of the incoming light and allows color measurements to be made
//! accurately. The high sensitivity, wide dynamic range, and IR blocking
//! filter make the TCS3472 an ideal color sensor solution for use under
//! varying lighting conditions and through attenuating materials.
//!
//! The TCS3472 color sensor has a wide range of applications including RGB LED
//! backlight control, solid-state lighting, health/fitness products,
//! industrial process controls and medical diagnostic equipment. In addition,
//! the IR blocking filter enables the TCS3472 to perform ambient light sensing
//! (ALS). Ambient light sensing is widely used in display-based products such
//! as cell phones, notebooks, and TVs to sense the lighting environment and
//! enable automatic display brightness for optimal viewing and power savings.
//! The TCS3472, itself, can enter a lower-power wait state between light
//! sensing measurements to further reduce the average power consumption.
//!
//! Datasheet:
//! - [TCS3472](https://ams.com/documents/20143/36005/TCS3472_DS000390_2-00.pdf)
//!
//! This driver is compatible with the devices TCS34725 and TCS34727.
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then create an instance of the driver.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Enable and read the color measurement
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tcs3472::Tcs3472;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tcs3472::new(dev);
//! sensor.enable().unwrap();
//! sensor.enable_rgbc().unwrap();
//! while !sensor.is_rgbc_status_valid().unwrap() {
//!     // wait for measurement to be available
//! };
//!
//! let clear = sensor.read_clear_channel().unwrap();
//! let red = sensor.read_red_channel().unwrap();
//! let green = sensor.read_green_channel().unwrap();
//! let blue = sensor.read_blue_channel().unwrap();
//!
//! println!("Measurements: clear = {}, red = {}, green = {}, blue = {}",
//!          clear, red, green, blue);
//! ```
//!
//! ### Read all the channels at once
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tcs3472::Tcs3472;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tcs3472::new(dev);
//! sensor.enable().unwrap();
//! sensor.enable_rgbc().unwrap();
//! while !sensor.is_rgbc_status_valid().unwrap() {
//!     // wait for measurement to be available
//! };
//!
//! let measurement = sensor.read_all_channels().unwrap();
//!
//! println!("Measurements: clear = {}, red = {}, green = {}, blue = {}",
//!          measurement.clear, measurement.red, measurement.green,
//!          measurement.blue);
//! ```
//!
//! ### Change the RGB converter gain and integration cycles
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tcs3472::{RgbCGain, Tcs3472};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tcs3472::new(dev);
//! sensor.enable().unwrap();
//! sensor.enable_rgbc().unwrap();
//! sensor.set_rgbc_gain(RgbCGain::_16x).unwrap();
//! sensor.set_integration_cycles(32).unwrap();
//! ```
//!
//! ### Enable wait function and set wait time to 1.008s
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tcs3472::Tcs3472;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tcs3472::new(dev);
//! sensor.enable().unwrap();
//! sensor.enable_rgbc().unwrap();
//! // This results in 35 * 2.4ms * 12 = 1.008s
//! sensor.set_wait_cycles(35).unwrap();
//! sensor.enable_wait_long().unwrap(); // 12x mutiplicator
//! sensor.enable_wait().unwrap(); // actually enable wait timer
//! ```
//!
//! ### Enable and configure RGB converter interrupt generation
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use tcs3472::{RgbCInterruptPersistence, Tcs3472};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tcs3472::new(dev);
//! sensor.enable().unwrap();
//! sensor.enable_rgbc().unwrap();
//! sensor.set_rgbc_interrupt_low_threshold(1024).unwrap();
//! sensor.set_rgbc_interrupt_high_threshold(61440).unwrap();
//! sensor.set_rgbc_interrupt_persistence(RgbCInterruptPersistence::_5).unwrap();
//! sensor.enable_rgbc_interrupts().unwrap();
//! ```
//!
//! ### Using async driver
//!
//! Enable `async` feature in Cargo.toml:
//! ```toml
//! tcs3472 = { version = "", features = ["async"] };
//! ```
//!
//! Using async driver with [Embassy](https://embassy.dev/) framework:
//! ```no_run
//! #[embassy_executor::main]
//! async fn main(_spawner: Spawner) {
//!     let p = embassy_stm32::init(Default::default());
//!     let mut i2c = I2c::new(..);
//!     let mut sensor = Tcs3472::new(i2c);
//!     sensor.enable().await.unwrap();
//!     sensor.enable_rgbc().await.unwrap();
//!     while !sensor.is_rgbc_status_valid().await.unwrap() {
//!         // wait for measurement to be available
//!     };
//!
//!     let measurement = sensor.read_all_channels().await.unwrap();
//!
//!     defmt::info!("Measurements: clear = {}, red = {}, green = {}, blue = {}",
//!          measurement.clear, measurement.red, measurement.green,
//!          measurement.blue);
//! }
//! ```


#![deny(unsafe_code, missing_docs)]
#![no_std]

mod configuration;
mod interface;
use crate::interface::{BitFlags, Register, DEVICE_ADDRESS};
mod reading;
mod types;
pub use crate::types::{AllChannelMeasurement, Error, RgbCGain, RgbCInterruptPersistence};

/// TCS3472 device driver.
#[derive(Debug)]
pub struct Tcs3472<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// Enable register status
    enable: u8,
}

impl<I2C> Tcs3472<I2C> {
    /// Create new instance of the TCS3472 device.
    pub fn new(i2c: I2C) -> Self {
        Tcs3472 { i2c, enable: 0 }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
