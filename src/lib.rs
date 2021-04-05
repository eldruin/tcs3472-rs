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
//! ### Enable and read the color measurement
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tcs3472;
//!
//! use hal::I2cdev;
//! use tcs3472::Tcs3472;
//!
//! # fn main() {
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
//! # }
//! ```
//!
//! ### Read all the channels at once
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tcs3472;
//!
//! use hal::I2cdev;
//! use tcs3472::Tcs3472;
//!
//! # fn main() {
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
//! # }
//! ```
//!
//! ### Change the RGB converter gain and integration cycles
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tcs3472;
//!
//! use hal::I2cdev;
//! use tcs3472::{ Tcs3472, RgbCGain };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tcs3472::new(dev);
//! sensor.enable().unwrap();
//! sensor.enable_rgbc().unwrap();
//! sensor.set_rgbc_gain(RgbCGain::_16x).unwrap();
//! sensor.set_integration_cycles(32).unwrap();
//! # }
//! ```
//!
//! ### Enable wait function and set wait time to 1.008s
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tcs3472;
//!
//! use hal::I2cdev;
//! use tcs3472::Tcs3472;
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tcs3472::new(dev);
//! sensor.enable().unwrap();
//! sensor.enable_rgbc().unwrap();
//! // This results in 35 * 2.4ms * 12 = 1.008s
//! sensor.set_wait_cycles(35).unwrap();
//! sensor.enable_wait_long().unwrap(); // 12x mutiplicator
//! sensor.enable_wait().unwrap(); // actually enable wait timer
//! # }
//! ```
//!
//! ### Enable and configure RGB converter interrupt generation
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate tcs3472;
//!
//! use hal::I2cdev;
//! use tcs3472::{ Tcs3472, RgbCInterruptPersistence };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Tcs3472::new(dev);
//! sensor.enable().unwrap();
//! sensor.enable_rgbc().unwrap();
//! sensor.set_rgbc_interrupt_low_threshold(1024).unwrap();
//! sensor.set_rgbc_interrupt_high_threshold(61440).unwrap();
//! sensor.set_rgbc_interrupt_persistence(RgbCInterruptPersistence::_5).unwrap();
//! sensor.enable_rgbc_interrupts().unwrap();
//! # }
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data provided.
    InvalidInputData,
}

/// RGB converter gain
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RgbCGain {
    /// 1x gain
    _1x,
    /// 4x gain
    _4x,
    /// 16x gain
    _16x,
    /// 60x gain
    _60x,
}

/// RGB converter interrupt persistence
///
/// This controls the RGB converter interrupt generation rate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RgbCInterruptPersistence {
    /// Every RGBC cycle generates an interrupt.
    Every,
    /// 1 clear channel value out of range.
    _1,
    /// 2 clear channel consecutive values out of range.
    _2,
    /// 3 clear channel consecutive values out of range.
    _3,
    /// 5 clear channel consecutive values out of range.
    _5,
    /// 10 clear channel consecutive values out of range.
    _10,
    /// 15 clear channel consecutive values out of range.
    _15,
    /// 20 clear channel consecutive values out of range.
    _20,
    /// 25 clear channel consecutive values out of range.
    _25,
    /// 30 clear channel consecutive values out of range.
    _30,
    /// 35 clear channel consecutive values out of range.
    _35,
    /// 40 clear channel consecutive values out of range.
    _40,
    /// 45 clear channel consecutive values out of range.
    _45,
    /// 50 clear channel consecutive values out of range.
    _50,
    /// 55 clear channel consecutive values out of range.
    _55,
    /// 60 clear channel consecutive values out of range.
    _60,
}

/// Result of measurement of all channels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AllChannelMeasurement {
    /// Red channel measurement.
    pub red: u16,
    /// Green channel measurement.
    pub green: u16,
    /// Blue channel measurement.
    pub blue: u16,
    /// Clear (unfiltered) channel measurement.
    pub clear: u16,
}

const DEVICE_ADDRESS: u8 = 0x29;

struct Register;

impl Register {
    const ENABLE: u8 = 0x00;
    const ATIME: u8 = 0x01;
    const WTIME: u8 = 0x03;
    const AILTL: u8 = 0x04;
    const AIHTL: u8 = 0x06;
    const APERS: u8 = 0x0C;
    const CONFIG: u8 = 0x0D;
    const CONTROL: u8 = 0x0F;
    const ID: u8 = 0x12;
    const STATUS: u8 = 0x13;
    const CDATA: u8 = 0x14;
    const RDATA: u8 = 0x16;
    const GDATA: u8 = 0x18;
    const BDATA: u8 = 0x1A;
}

struct BitFlags;

impl BitFlags {
    const CMD: u8 = 0b1000_0000;
    const CMD_AUTO_INC: u8 = 0b0010_0000;
    const POWER_ON: u8 = 0b0000_0001; // PON
    const RGBC_EN: u8 = 0b0000_0010; // AEN
    const WAIT_EN: u8 = 0b0000_1000; // WEN
    const RGBC_INT_EN: u8 = 0b0001_0000; // AIEN
    const RGBC_VALID: u8 = 0b0000_0001; // AVALID
    const WLONG: u8 = 0b0000_0010;
}

/// TCS3472 device driver.
#[derive(Debug, Default)]
pub struct Tcs3472<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// Enable register status
    enable: u8,
}

mod configuration;
mod reading;

impl<I2C, E> Tcs3472<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Create new instance of the TCS3472 device.
    pub fn new(i2c: I2C) -> Self {
        Tcs3472 { i2c, enable: 0 }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
