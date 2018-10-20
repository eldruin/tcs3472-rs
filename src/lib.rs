//! This is a platform agnostic Rust driver for the TCS3472 RGB color light to
//! digital converter with IR filter, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device.
//! - Enable/disable the RGB converter.
//! - Read status of RGB converter.
//! - Read the clear (unfiltered) channel measurement.
//! - Read the red channel measurement.
//! - Read the green channel measurement.
//! - Read the blue channel measurement.
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
}

const DEVICE_ADDRESS: u8 = 0x29;

struct Register;

impl Register {
    const ENABLE   : u8 = 0x00;
    const STATUS   : u8 = 0x13;
    const CDATA    : u8 = 0x14;
    const RDATA    : u8 = 0x16;
    const GDATA    : u8 = 0x18;
    const BDATA    : u8 = 0x1A;
}

struct BitFlags;

impl BitFlags {
    const CMD        : u8 = 0b1000_0000;
    const POWER_ON   : u8 = 0b0000_0001; // PON
    const RGBC_EN    : u8 = 0b0000_0010; // AEN
    const RGBC_VALID : u8 = 0b0000_0001; // AVALID
}

/// TCS3472 device driver.
#[derive(Debug, Default)]
pub struct Tcs3472<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// Enable register status
    enable: u8
}

impl<I2C, E> Tcs3472<I2C>
where
    I2C: i2c::Write<Error = E>
{
    /// Create new instance of the TCS3472 device.
    pub fn new(i2c: I2C) -> Self {
        Tcs3472 {
            i2c,
            enable: 0
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Enable the device (Power ON).
    ///
    /// The device goes to idle state.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::POWER_ON)
    }

    /// Disable the device (sleep).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::POWER_ON)
    }

    /// Enable the RGB converter.
    pub fn enable_rgbc(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::RGBC_EN)
    }

    /// Disable the RGB converter.
    pub fn disable_rgbc(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::RGBC_EN)
    }

    fn write_enable(&mut self, enable: u8) -> Result<(), Error<E>> {
        let command = BitFlags::CMD | Register::ENABLE;
        self.i2c
            .write(DEVICE_ADDRESS, &[command, enable])
            .map_err(Error::I2C)?;
        self.enable = enable;
        Ok(())
    }
}

impl<I2C, E> Tcs3472<I2C>
where
    I2C: i2c::WriteRead<Error = E>
{
    /// Check whether the RGB converter status is valid.
    ///
    /// Indicates that the RGBC channels have completed an integration cycle.
    pub fn is_rgbc_status_valid(&mut self) -> Result<bool, Error<E>> {
        let mut status = [0]; 
        self.read_register(Register::STATUS, &mut status)?;
        Ok((status[0] & BitFlags::RGBC_VALID) != 0)
    }

    /// Read the clear (unfiltered) channel measurement data.
    pub fn read_clear_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::CDATA)
    }

    /// Read the red channel measurement data.
    pub fn read_red_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::RDATA)
    }

    /// Read the green channel measurement data.
    pub fn read_green_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::GDATA)
    }

    /// Read the blue channel measurement data.
    pub fn read_blue_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::BDATA)
    }

    fn read_channel(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut cdata = [0; 2]; 
        self.read_register(register, &mut cdata)?;
        Ok((cdata[1] as u16) << 8 | cdata[0] as u16)
    }

    fn read_register(&mut self, register: u8, mut data: &mut [u8]) -> Result<(), Error<E>> {
        let command = BitFlags::CMD | register;
        self.i2c
            .write_read(DEVICE_ADDRESS, &[command], &mut data)
            .map_err(Error::I2C)
    }
}
