
extern crate embedded_hal as hal;
use hal::blocking::i2c;
use super::{ Tcs3472, DEVICE_ADDRESS, Register, BitFlags, RgbCGain, Error };

impl<I2C, E> Tcs3472<I2C>
where
    I2C: i2c::Write<Error = E>
{
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
        self.write_register(Register::ENABLE, enable)?;
        self.enable = enable;
        Ok(())
    }

    /// Set the RGB converter gain
    pub fn set_rgbc_gain(&mut self, gain: RgbCGain) -> Result<(), Error<E>> {
        match gain {
            RgbCGain::_1x  => self.write_register(Register::CONTROL, 0),
            RgbCGain::_4x  => self.write_register(Register::CONTROL, 1),
            RgbCGain::_16x => self.write_register(Register::CONTROL, 2),
            RgbCGain::_60x => self.write_register(Register::CONTROL, 3),
        }
    }

    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        let command = BitFlags::CMD | register;
        self.i2c
            .write(DEVICE_ADDRESS, &[command, value])
            .map_err(Error::I2C)
    }
}
