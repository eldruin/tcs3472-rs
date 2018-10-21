extern crate tcs3472;
extern crate embedded_hal_mock as hal;
use tcs3472::Tcs3472;

const DEVICE_ADDRESS: u8 = 0x29;

pub struct Register;

#[allow(unused)]
impl Register {
    pub const ENABLE   : u8 = 0x00;
    pub const ATIME    : u8 = 0x01;
    pub const WTIME    : u8 = 0x03;
    pub const CONFIG   : u8 = 0x0D;
    pub const CONTROL  : u8 = 0x0F;
    pub const ID       : u8 = 0x12;
    pub const STATUS   : u8 = 0x13;
    pub const CDATA    : u8 = 0x14;
    pub const RDATA    : u8 = 0x16;
    pub const GDATA    : u8 = 0x18;
    pub const BDATA    : u8 = 0x1A;
}

pub struct BitFlags;

#[allow(unused)]
impl BitFlags {
    pub const CMD          : u8 = 0b1000_0000;
    pub const CMD_AUTO_INC : u8 = 0b0010_0000;
    pub const POWER_ON     : u8 = 0b0000_0001; // PON
    pub const RGBC_EN      : u8 = 0b0000_0010; // AEN
    pub const RGBC_VALID   : u8 = 0b0000_0001; // AVALID
    pub const WLONG        : u8 = 0b0000_0010;
}

pub fn setup<'a>(data: &'a[u8]) -> Tcs3472<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    Tcs3472::new(dev)
}

pub fn check_sent_data(sensor: Tcs3472<hal::I2cMock>, data: &[u8]) {
    let dev = sensor.destroy();
    assert_eq!(dev.get_last_address(), Some(DEVICE_ADDRESS));
    assert_eq!(dev.get_write_data(), &data[..]);
}
