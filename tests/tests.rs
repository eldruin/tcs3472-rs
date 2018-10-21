extern crate tcs3472;
extern crate embedded_hal_mock as hal;
use tcs3472::Tcs3472;

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
    const CMD          : u8 = 0b1000_0000;
    const CMD_AUTO_INC : u8 = 0b0010_0000;
    const POWER_ON     : u8 = 0b0000_0001; // PON
    const RGBC_EN      : u8 = 0b0000_0010; // AEN
    const RGBC_VALID   : u8 = 0b0000_0001; // AVALID
}

fn setup<'a>(data: &'a[u8]) -> Tcs3472<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    Tcs3472::new(dev)
}

fn check_sent_data(sensor: Tcs3472<hal::I2cMock>, data: &[u8]) {
    let dev = sensor.destroy();
    assert_eq!(dev.get_last_address(), Some(DEVICE_ADDRESS));
    assert_eq!(dev.get_write_data(), &data[..]);
}

const ENABLE_CMD : u8 = BitFlags::CMD | Register::ENABLE;

#[test]
fn can_enable() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    check_sent_data(dev, &[ENABLE_CMD, BitFlags::POWER_ON]);
}

#[test]
fn can_disable() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    dev.disable().unwrap();
    check_sent_data(dev, &[ENABLE_CMD, 0]);
}

#[test]
fn can_enable_rgbc() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    dev.enable_rgbc().unwrap();
    check_sent_data(dev, &[ENABLE_CMD, BitFlags::POWER_ON | BitFlags::RGBC_EN]);
}

#[test]
fn can_disable_rgbc() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    dev.enable_rgbc().unwrap();
    dev.disable_rgbc().unwrap();
    check_sent_data(dev, &[ENABLE_CMD, BitFlags::POWER_ON & !BitFlags::RGBC_EN]);
}

#[test]
fn can_read_rgbc_status_not_valid() {
    let mut dev = setup(&[0]);
    let is_valid = dev.is_rgbc_status_valid().unwrap();
    assert!(!is_valid);
    check_sent_data(dev, &[BitFlags::CMD | BitFlags::CMD_AUTO_INC | Register::STATUS]);
}

#[test]
fn can_read_rgbc_status_valid() {
    let mut dev = setup(&[BitFlags::RGBC_VALID]);
    let is_valid = dev.is_rgbc_status_valid().unwrap();
    assert!(is_valid);
    check_sent_data(dev, &[BitFlags::CMD | BitFlags::CMD_AUTO_INC | Register::STATUS]);
}

macro_rules! read_channel_test {
    ($name:ident, $method:ident, $register:ident) => {
        #[test]
        fn $name() {
            let mut dev = setup(&[0xCD, 0xAB]);
            let data = dev.$method().unwrap();
            assert_eq!(0xABCD, data);
            check_sent_data(dev, &[BitFlags::CMD | BitFlags::CMD_AUTO_INC | Register::$register]);
        }
    };
}

read_channel_test!(can_read_clear_channel, read_clear_channel, CDATA);
read_channel_test!(can_read_red_channel,   read_red_channel,   RDATA);
read_channel_test!(can_read_green_channel, read_green_channel, GDATA);
read_channel_test!(can_read_blue_channel,  read_blue_channel,  BDATA);
