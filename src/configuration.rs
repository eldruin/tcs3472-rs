
extern crate embedded_hal as hal;
use hal::blocking::i2c;
use super::{ Tcs3472, DEVICE_ADDRESS, Register, BitFlags, Error };

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
        let command = BitFlags::CMD | Register::ENABLE;
        self.i2c
            .write(DEVICE_ADDRESS, &[command, enable])
            .map_err(Error::I2C)?;
        self.enable = enable;
        Ok(())
    }
}
