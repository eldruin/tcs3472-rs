use crate::{
    BitFlags, Error, Register, RgbCGain, RgbCInterruptPersistence, Tcs3472, DEVICE_ADDRESS,
};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tcs3472",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Tcs3472<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Enable the device (Power ON).
    ///
    /// The device goes to idle state.
    pub async fn enable(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::POWER_ON).await
    }

    /// Disable the device (sleep).
    pub async fn disable(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::POWER_ON).await
    }

    /// Enable the RGB converter.
    pub async fn enable_rgbc(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::RGBC_EN).await
    }

    /// Disable the RGB converter.
    pub async fn disable_rgbc(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::RGBC_EN).await
    }

    /// Enable the RGB converter interrupt generation.
    pub async fn enable_rgbc_interrupts(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::RGBC_INT_EN).await
    }

    /// Disable the RGB converter interrupt generation.
    pub async fn disable_rgbc_interrupts(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::RGBC_INT_EN).await
    }

    /// Enable the wait feature (wait timer).
    pub async fn enable_wait(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::WAIT_EN).await
    }

    /// Disable the wait feature (wait timer).
    pub async fn disable_wait(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::WAIT_EN).await
    }

    async fn write_enable(&mut self, enable: u8) -> Result<(), Error<E>> {
        self.write_register(Register::ENABLE, enable).await?;
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
    ///
    /// See [`enable_wait_long()`](#method.enable_wait_long) and
    ///  [`disable_wait_long()`](#method.disable_wait_long).
    pub async fn set_wait_cycles(&mut self, cycles: u16) -> Result<(), Error<E>> {
        if cycles > 256 || cycles == 0 {
            return Err(Error::InvalidInputData);
        }
        // the value is stored as a two's complement
        self.write_register(Register::WTIME, (256_u16 - cycles) as u8)
            .await
    }

    /// Enable the *wait long* setting.
    ///
    /// The wait time configured with `set_wait_cycles()` is increased by a
    /// factor of 12. See [`set_wait_cycles()`](#method.set_wait_cycles).
    pub async fn enable_wait_long(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG, BitFlags::WLONG).await
    }

    /// Disable the *wait long* setting.
    ///
    /// The wait time configured with `set_wait_cycles()` is used without
    /// multiplication factor. See [`set_wait_cycles()`](#method.set_wait_cycles).
    pub async fn disable_wait_long(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG, 0).await
    }

    /// Set the RGB converter gain.
    pub async fn set_rgbc_gain(&mut self, gain: RgbCGain) -> Result<(), Error<E>> {
        // Register field: AGAIN
        match gain {
            RgbCGain::_1x => self.write_register(Register::CONTROL, 0).await,
            RgbCGain::_4x => self.write_register(Register::CONTROL, 1).await,
            RgbCGain::_16x => self.write_register(Register::CONTROL, 2).await,
            RgbCGain::_60x => self.write_register(Register::CONTROL, 3).await,
        }
    }

    /// Set the number of integration cycles (1-256).
    ///
    /// The actual integration time corresponds to: `number_of_cycles * 2.4ms`.
    pub async fn set_integration_cycles(&mut self, cycles: u16) -> Result<(), Error<E>> {
        if cycles > 256 || cycles == 0 {
            return Err(Error::InvalidInputData);
        }
        // the value is stored as a two's complement
        self.write_register(Register::ATIME, (256_u16 - cycles) as u8)
            .await
    }

    /// Set the RGB converter interrupt clear channel low threshold.
    pub async fn set_rgbc_interrupt_low_threshold(
        &mut self,
        threshold: u16,
    ) -> Result<(), Error<E>> {
        self.write_registers(Register::AILTL, threshold as u8, (threshold >> 8) as u8)
            .await
    }

    /// Set the RGB converter interrupt clear channel high threshold.
    pub async fn set_rgbc_interrupt_high_threshold(
        &mut self,
        threshold: u16,
    ) -> Result<(), Error<E>> {
        self.write_registers(Register::AIHTL, threshold as u8, (threshold >> 8) as u8)
            .await
    }

    /// Set the RGB converter interrupt persistence.
    ///
    /// This controls the RGB converter interrupt generation rate.
    pub async fn set_rgbc_interrupt_persistence(
        &mut self,
        persistence: RgbCInterruptPersistence,
    ) -> Result<(), Error<I2C::Error>> {
        use crate::RgbCInterruptPersistence as IP;
        match persistence {
            IP::Every => self.write_register(Register::APERS, 0).await,
            IP::_1 => self.write_register(Register::APERS, 1).await,
            IP::_2 => self.write_register(Register::APERS, 2).await,
            IP::_3 => self.write_register(Register::APERS, 3).await,
            IP::_5 => self.write_register(Register::APERS, 4).await,
            IP::_10 => self.write_register(Register::APERS, 5).await,
            IP::_15 => self.write_register(Register::APERS, 6).await,
            IP::_20 => self.write_register(Register::APERS, 7).await,
            IP::_25 => self.write_register(Register::APERS, 8).await,
            IP::_30 => self.write_register(Register::APERS, 9).await,
            IP::_35 => self.write_register(Register::APERS, 10).await,
            IP::_40 => self.write_register(Register::APERS, 11).await,
            IP::_45 => self.write_register(Register::APERS, 12).await,
            IP::_50 => self.write_register(Register::APERS, 13).await,
            IP::_55 => self.write_register(Register::APERS, 14).await,
            IP::_60 => self.write_register(Register::APERS, 15).await,
        }
    }

    async fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        let command = BitFlags::CMD | register;
        self.i2c
            .write(DEVICE_ADDRESS, &[command, value])
            .await
            .map_err(Error::I2C)
    }

    async fn write_registers(
        &mut self,
        register: u8,
        value0: u8,
        value1: u8,
    ) -> Result<(), Error<E>> {
        let command = BitFlags::CMD | BitFlags::CMD_AUTO_INC | register;
        self.i2c
            .write(DEVICE_ADDRESS, &[command, value0, value1])
            .await
            .map_err(Error::I2C)
    }
}
