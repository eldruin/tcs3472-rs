
extern crate embedded_hal as hal;
use hal::blocking::i2c;
use super::{ Tcs3472, DEVICE_ADDRESS, Register, BitFlags, Error };

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
        let command = BitFlags::CMD | BitFlags::CMD_AUTO_INC | register;
        self.i2c
            .write_read(DEVICE_ADDRESS, &[command], &mut data)
            .map_err(Error::I2C)
    }
}
