use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use tcs3472::Tcs3472;

pub const DEV_ADDR: u8 = 0x29;

pub struct Register;

#[allow(unused)]
impl Register {
    pub const ENABLE: u8 = 0x00;
    pub const ATIME: u8 = 0x01;
    pub const WTIME: u8 = 0x03;
    pub const AILTL: u8 = 0x04;
    pub const AIHTL: u8 = 0x06;
    pub const APERS: u8 = 0x0C;
    pub const CONFIG: u8 = 0x0D;
    pub const CONTROL: u8 = 0x0F;
    pub const ID: u8 = 0x12;
    pub const STATUS: u8 = 0x13;
    pub const CDATA: u8 = 0x14;
    pub const RDATA: u8 = 0x16;
    pub const GDATA: u8 = 0x18;
    pub const BDATA: u8 = 0x1A;
}

pub struct BitFlags;

#[allow(unused)]
impl BitFlags {
    pub const CMD: u8 = 0b1000_0000;
    pub const CMD_AUTO_INC: u8 = 0b0010_0000;
    pub const POWER_ON: u8 = 0b0000_0001; // PON
    pub const RGBC_EN: u8 = 0b0000_0010; // AEN
    pub const WAIT_EN: u8 = 0b0000_1000; // WEN
    pub const RGBC_INT_EN: u8 = 0b0001_0000; // AIEN
    pub const RGBC_VALID: u8 = 0b0000_0001; // AVALID
    pub const WLONG: u8 = 0b0000_0010;
}

pub fn new(transactions: &[I2cTrans]) -> Tcs3472<I2cMock> {
    Tcs3472::new(I2cMock::new(transactions))
}

pub fn destroy(sensor: Tcs3472<I2cMock>) {
    sensor.destroy().done();
}
