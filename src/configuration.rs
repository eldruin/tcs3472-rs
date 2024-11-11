use crate::{
    BitFlags, Error, Register, RgbCGain, RgbCInterruptPersistence, Tcs3472, DEVICE_ADDRESS,
};
use embedded_hal::i2c;

impl<I2C> Tcs3472<I2C>
where
    I2C: i2c::I2c,
{
    /// Enable the device (Power ON).
    ///
    /// The device goes to idle state.
    pub fn enable(&mut self) -> Result<(), Error<I2C::Error>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::POWER_ON)
    }

    /// Disable the device (sleep).
    pub fn disable(&mut self) -> Result<(), Error<I2C::Error>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::POWER_ON)
    }

    /// Enable the RGB converter.
    pub fn enable_rgbc(&mut self) -> Result<(), Error<I2C::Error>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::RGBC_EN)
    }

    /// Disable the RGB converter.
    pub fn disable_rgbc(&mut self) -> Result<(), Error<I2C::Error>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::RGBC_EN)
    }

    /// Enable the RGB converter interrupt generation.
    pub fn enable_rgbc_interrupts(&mut self) -> Result<(), Error<I2C::Error>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::RGBC_INT_EN)
    }

    /// Disable the RGB converter interrupt generation.
    pub fn disable_rgbc_interrupts(&mut self) -> Result<(), Error<I2C::Error>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::RGBC_INT_EN)
    }

    /// Enable the wait feature (wait timer).
    pub fn enable_wait(&mut self) -> Result<(), Error<I2C::Error>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::WAIT_EN)
    }

    /// Disable the wait feature (wait timer).
    pub fn disable_wait(&mut self) -> Result<(), Error<I2C::Error>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::WAIT_EN)
    }

    fn write_enable(&mut self, enable: u8) -> Result<(), Error<I2C::Error>> {
        self.write_register(Register::ENABLE, enable)?;
        self.enable = enable;
        Ok(())
    }

    /// Set the number of wait time cycles  (1-256).
    ///
    /// The actual wait time depends on the "*wait long*" setting.
    /// - If *wait long* is disabled, then the wait time corresponds to:
    ///   `number_of_cycles * 2.4ms`.
    /// - If *wait long* is enabled, then the wait time is increased by a
    ///   factor of 12 and therefore corresponds to aproximately:
    ///   `number_of_cycles * 0.029s`.
    /// See [`enable_wait_long()`](#method.enable_wait_long) and
    ///  [`disable_wait_long()`](#method.disable_wait_long).
    pub fn set_wait_cycles(&mut self, cycles: u16) -> Result<(), Error<I2C::Error>> {
        if cycles > 256 || cycles == 0 {
            return Err(Error::InvalidInputData);
        }
        // the value is stored as a two's complement
        self.write_register(Register::WTIME, (256_u16 - cycles) as u8)
    }

    /// Enable the *wait long* setting.
    ///
    /// The wait time configured with `set_wait_cycles()` is increased by a
    /// factor of 12. See [`set_wait_cycles()`](#method.set_wait_cycles).
    pub fn enable_wait_long(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_register(Register::CONFIG, BitFlags::WLONG)
    }

    /// Disable the *wait long* setting.
    ///
    /// The wait time configured with `set_wait_cycles()` is used without
    /// multiplication factor. See [`set_wait_cycles()`](#method.set_wait_cycles).
    pub fn disable_wait_long(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_register(Register::CONFIG, 0)
    }

    /// Set the RGB converter gain.
    pub fn set_rgbc_gain(&mut self, gain: RgbCGain) -> Result<(), Error<I2C::Error>> {
        // Register field: AGAIN
        match gain {
            RgbCGain::_1x => self.write_register(Register::CONTROL, 0),
            RgbCGain::_4x => self.write_register(Register::CONTROL, 1),
            RgbCGain::_16x => self.write_register(Register::CONTROL, 2),
            RgbCGain::_60x => self.write_register(Register::CONTROL, 3),
        }
    }

    /// Set the number of integration cycles (1-256).
    ///
    /// The actual integration time corresponds to: `number_of_cycles * 2.4ms`.
    pub fn set_integration_cycles(&mut self, cycles: u16) -> Result<(), Error<I2C::Error>> {
        if cycles > 256 || cycles == 0 {
            return Err(Error::InvalidInputData);
        }
        // the value is stored as a two's complement
        self.write_register(Register::ATIME, (256_u16 - cycles) as u8)
    }

    /// Set the RGB converter interrupt clear channel low threshold.
    pub fn set_rgbc_interrupt_low_threshold(&mut self, threshold: u16) -> Result<(), Error<I2C::Error>> {
        self.write_registers(Register::AILTL, threshold as u8, (threshold >> 8) as u8)
    }

    /// Set the RGB converter interrupt clear channel high threshold.
    pub fn set_rgbc_interrupt_high_threshold(&mut self, threshold: u16) -> Result<(), Error<I2C::Error>> {
        self.write_registers(Register::AIHTL, threshold as u8, (threshold >> 8) as u8)
    }

    /// Set the RGB converter interrupt persistence.
    ///
    /// This controls the RGB converter interrupt generation rate.
    pub fn set_rgbc_interrupt_persistence(
        &mut self,
        persistence: RgbCInterruptPersistence,
    ) -> Result<(), Error<I2C::Error>> {
        use crate::RgbCInterruptPersistence as IP;
        match persistence {
            IP::Every => self.write_register(Register::APERS, 0),
            IP::_1 => self.write_register(Register::APERS, 1),
            IP::_2 => self.write_register(Register::APERS, 2),
            IP::_3 => self.write_register(Register::APERS, 3),
            IP::_5 => self.write_register(Register::APERS, 4),
            IP::_10 => self.write_register(Register::APERS, 5),
            IP::_15 => self.write_register(Register::APERS, 6),
            IP::_20 => self.write_register(Register::APERS, 7),
            IP::_25 => self.write_register(Register::APERS, 8),
            IP::_30 => self.write_register(Register::APERS, 9),
            IP::_35 => self.write_register(Register::APERS, 10),
            IP::_40 => self.write_register(Register::APERS, 11),
            IP::_45 => self.write_register(Register::APERS, 12),
            IP::_50 => self.write_register(Register::APERS, 13),
            IP::_55 => self.write_register(Register::APERS, 14),
            IP::_60 => self.write_register(Register::APERS, 15),
        }
    }

    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<I2C::Error>> {
        let command = BitFlags::CMD | register;
        self.i2c
            .write(DEVICE_ADDRESS, &[command, value])
            .map_err(Error::I2C)
    }

    fn write_registers(&mut self, register: u8, value0: u8, value1: u8) -> Result<(), Error<I2C::Error>> {
        let command = BitFlags::CMD | BitFlags::CMD_AUTO_INC | register;
        self.i2c
            .write(DEVICE_ADDRESS, &[command, value0, value1])
            .map_err(Error::I2C)
    }
}
