pub(crate) const DEVICE_ADDRESS: u8 = 0x29;

pub(crate) struct Register;

impl Register {
    pub(crate) const ENABLE: u8 = 0x00;
    pub(crate) const ATIME: u8 = 0x01;
    pub(crate) const WTIME: u8 = 0x03;
    pub(crate) const AILTL: u8 = 0x04;
    pub(crate) const AIHTL: u8 = 0x06;
    pub(crate) const APERS: u8 = 0x0C;
    pub(crate) const CONFIG: u8 = 0x0D;
    pub(crate) const CONTROL: u8 = 0x0F;
    pub(crate) const ID: u8 = 0x12;
    pub(crate) const STATUS: u8 = 0x13;
    pub(crate) const CDATA: u8 = 0x14;
    pub(crate) const RDATA: u8 = 0x16;
    pub(crate) const GDATA: u8 = 0x18;
    pub(crate) const BDATA: u8 = 0x1A;
}

pub(crate) struct BitFlags;

impl BitFlags {
    pub(crate) const CMD: u8 = 0b1000_0000;
    pub(crate) const CMD_AUTO_INC: u8 = 0b0010_0000;
    pub(crate) const POWER_ON: u8 = 0b0000_0001; // PON
    pub(crate) const RGBC_EN: u8 = 0b0000_0010; // AEN
    pub(crate) const WAIT_EN: u8 = 0b0000_1000; // WEN
    pub(crate) const RGBC_INT_EN: u8 = 0b0001_0000; // AIEN
    pub(crate) const RGBC_VALID: u8 = 0b0000_0001; // AVALID
    pub(crate) const WLONG: u8 = 0b0000_0010;
}
