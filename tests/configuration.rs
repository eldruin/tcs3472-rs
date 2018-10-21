extern crate tcs3472;
extern crate embedded_hal_mock as hal;

mod common;
use common::{ setup, check_sent_data, Register, BitFlags };

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
