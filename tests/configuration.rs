mod common;
use crate::common::{destroy, new, BitFlags, Register, DEV_ADDR};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
use tcs3472::{Error, RgbCGain, RgbCInterruptPersistence};

#[test]
fn can_create_and_destroy() {
    let sensor = new(&[]);
    destroy(sensor);
}

macro_rules! enable_disable_test {
    ($name:ident, $first_method:ident, $second_method:ident, $register:ident, $expected:expr) => {
        #[test]
        fn $name() {
            let mut dev = new(&[
                I2cTrans::write(DEV_ADDR, vec![BitFlags::CMD | Register::$register, 0]),
                I2cTrans::write(
                    DEV_ADDR,
                    vec![BitFlags::CMD | Register::$register, $expected],
                ),
            ]);
            dev.$first_method().unwrap();
            dev.$second_method().unwrap();
            destroy(dev);
        }
    };
}
enable_disable_test!(can_enable, disable, enable, ENABLE, BitFlags::POWER_ON);

enable_disable_test!(
    can_enable_rgbc,
    disable_rgbc,
    enable_rgbc,
    ENABLE,
    BitFlags::RGBC_EN
);

enable_disable_test!(
    can_enable_rgbc_ints,
    disable_rgbc_interrupts,
    enable_rgbc_interrupts,
    ENABLE,
    BitFlags::RGBC_INT_EN
);

enable_disable_test!(
    can_enable_wait,
    disable_wait,
    enable_wait,
    ENABLE,
    BitFlags::WAIT_EN
);

enable_disable_test!(
    can_enable_wait_long,
    disable_wait_long,
    enable_wait_long,
    CONFIG,
    BitFlags::WLONG
);

macro_rules! set_rgbc_gain_test {
    ($name:ident, $variant:ident, $expected:expr) => {
        #[test]
        fn $name() {
            let mut dev = new(&[I2cTrans::write(
                DEV_ADDR,
                vec![BitFlags::CMD | Register::CONTROL, $expected],
            )]);
            dev.set_rgbc_gain(RgbCGain::$variant).unwrap();
            destroy(dev);
        }
    };
}

set_rgbc_gain_test!(can_set_rgbc_gain_1x, _1x, 0);
set_rgbc_gain_test!(can_set_rgbc_gain_4x, _4x, 1);
set_rgbc_gain_test!(can_set_rgbc_gain_16x, _16x, 2);
set_rgbc_gain_test!(can_set_rgbc_gain_60x, _60x, 3);

macro_rules! set_invalid_param_test {
    ($name:ident, $method:ident, $value:expr) => {
        #[test]
        fn $name() {
            let mut dev = new(&[]);
            match dev.$method($value) {
                Err(Error::InvalidInputData) => (),
                _ => panic!(),
            }
        }
    };
}

set_invalid_param_test!(cannot_set_ic_0, set_integration_cycles, 0);
set_invalid_param_test!(cannot_set_ic_greater_256, set_integration_cycles, 257);

macro_rules! set_single_param_test {
    ($name:ident, $method:ident, $value:expr, $register:ident, $expected:expr) => {
        #[test]
        fn $name() {
            let mut dev = new(&[I2cTrans::write(
                DEV_ADDR,
                vec![BitFlags::CMD | Register::$register, $expected],
            )]);
            dev.$method($value).unwrap();
            destroy(dev);
        }
    };
}

set_single_param_test!(can_set_ic_1, set_integration_cycles, 1, ATIME, 0xFF);
set_single_param_test!(can_set_ic_10, set_integration_cycles, 10, ATIME, 0xF6);
set_single_param_test!(can_set_ic_256, set_integration_cycles, 256, ATIME, 0x00);

set_single_param_test!(can_set_wc_1, set_wait_cycles, 1, WTIME, 0xFF);
set_single_param_test!(can_set_wc_85, set_wait_cycles, 85, WTIME, 0xAB);
set_single_param_test!(can_set_wc_256, set_wait_cycles, 256, WTIME, 0x00);

macro_rules! set_param_test {
    ($name:ident, $method:ident, $value:expr, $register:ident, $expected0:expr, $expected1:expr) => {
        #[test]
        fn $name() {
            let mut dev = new(&[I2cTrans::write(
                DEV_ADDR,
                vec![
                    BitFlags::CMD | BitFlags::CMD_AUTO_INC | Register::$register,
                    $expected0,
                    $expected1,
                ],
            )]);
            dev.$method($value).unwrap();
            destroy(dev);
        }
    };
}
set_param_test!(
    can_set_rgbc_int_low_th_0,
    set_rgbc_interrupt_low_threshold,
    0,
    AILTL,
    0,
    0
);
set_param_test!(
    can_set_rgbc_int_low_th_1,
    set_rgbc_interrupt_low_threshold,
    1,
    AILTL,
    1,
    0
);
set_param_test!(
    can_set_rgbc_int_low_th_256,
    set_rgbc_interrupt_low_threshold,
    256,
    AILTL,
    0,
    1
);
set_param_test!(
    can_set_rgbc_int_low_th_max,
    set_rgbc_interrupt_low_threshold,
    65535,
    AILTL,
    255,
    255
);

set_param_test!(
    can_set_rgbc_int_high_th_0,
    set_rgbc_interrupt_high_threshold,
    0,
    AIHTL,
    0,
    0
);
set_param_test!(
    can_set_rgbc_int_high_th_1,
    set_rgbc_interrupt_high_threshold,
    1,
    AIHTL,
    1,
    0
);
set_param_test!(
    can_set_rgbc_int_high_th_256,
    set_rgbc_interrupt_high_threshold,
    256,
    AIHTL,
    0,
    1
);
set_param_test!(
    can_set_rgbc_int_high_th_max,
    set_rgbc_interrupt_high_threshold,
    65535,
    AIHTL,
    255,
    255
);

macro_rules! set_int_pers_test {
    ($name:ident, $rate:ident, $expected:expr) => {
        set_single_param_test!(
            $name,
            set_rgbc_interrupt_persistence,
            RgbCInterruptPersistence::$rate,
            APERS,
            $expected
        );
    };
}

set_int_pers_test!(can_set_rgbc_int_pers_every, Every, 0);
set_int_pers_test!(can_set_rgbc_int_pers_1, _1, 1);
set_int_pers_test!(can_set_rgbc_int_pers_2, _2, 2);
set_int_pers_test!(can_set_rgbc_int_pers_3, _3, 3);
set_int_pers_test!(can_set_rgbc_int_pers_5, _5, 4);
set_int_pers_test!(can_set_rgbc_int_pers_10, _10, 5);
set_int_pers_test!(can_set_rgbc_int_pers_15, _15, 6);
set_int_pers_test!(can_set_rgbc_int_pers_20, _20, 7);
set_int_pers_test!(can_set_rgbc_int_pers_25, _25, 8);
set_int_pers_test!(can_set_rgbc_int_pers_30, _30, 9);
set_int_pers_test!(can_set_rgbc_int_pers_35, _35, 10);
set_int_pers_test!(can_set_rgbc_int_pers_40, _40, 11);
set_int_pers_test!(can_set_rgbc_int_pers_45, _45, 12);
set_int_pers_test!(can_set_rgbc_int_pers_50, _50, 13);
set_int_pers_test!(can_set_rgbc_int_pers_55, _55, 14);
set_int_pers_test!(can_set_rgbc_int_pers_60, _60, 15);
