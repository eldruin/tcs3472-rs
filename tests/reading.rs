mod common;
use crate::common::{destroy, new, BitFlags, Register, DEV_ADDR};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;

#[test]
fn can_read_rgbc_status_not_valid() {
    let mut dev = new(&[I2cTrans::write_read(
        DEV_ADDR,
        vec![BitFlags::CMD | Register::STATUS],
        vec![0],
    )]);

    assert!(!dev.is_rgbc_status_valid().unwrap());
    destroy(dev);
}

#[test]
fn can_read_rgbc_status_valid() {
    let mut dev = new(&[I2cTrans::write_read(
        DEV_ADDR,
        vec![BitFlags::CMD | Register::STATUS],
        vec![BitFlags::RGBC_VALID],
    )]);
    assert!(dev.is_rgbc_status_valid().unwrap());
    destroy(dev);
}

macro_rules! read_channel_test {
    ($name:ident, $method:ident, $register:ident) => {
        #[test]
        fn $name() {
            let mut dev = new(&[I2cTrans::write_read(
                DEV_ADDR,
                vec![BitFlags::CMD | BitFlags::CMD_AUTO_INC | Register::$register],
                vec![0xCD, 0xAB],
            )]);
            let data = dev.$method().unwrap();
            assert_eq!(0xABCD, data);
            destroy(dev);
        }
    };
}

read_channel_test!(can_read_clear_channel, read_clear_channel, CDATA);
read_channel_test!(can_read_red_channel, read_red_channel, RDATA);
read_channel_test!(can_read_green_channel, read_green_channel, GDATA);
read_channel_test!(can_read_blue_channel, read_blue_channel, BDATA);

#[test]
fn can_read_all_channels_at_once() {
    let mut dev = new(&[I2cTrans::write_read(
        DEV_ADDR,
        vec![BitFlags::CMD | BitFlags::CMD_AUTO_INC | Register::CDATA],
        vec![0x23, 0x01, 0x67, 0x45, 0xAB, 0x89, 0xEF, 0xCD],
    )]);
    let measurement = dev.read_all_channels().unwrap();
    assert_eq!(0x0123, measurement.clear);
    assert_eq!(0x4567, measurement.red);
    assert_eq!(0x89AB, measurement.green);
    assert_eq!(0xCDEF, measurement.blue);
    destroy(dev);
}

#[test]
fn can_read_device_id() {
    let mut dev = new(&[I2cTrans::write_read(
        DEV_ADDR,
        vec![BitFlags::CMD | Register::ID],
        vec![0x44],
    )]);
    assert_eq!(0x44, dev.read_device_id().unwrap());
    destroy(dev);
}
