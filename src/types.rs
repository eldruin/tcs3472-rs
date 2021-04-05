/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data provided.
    InvalidInputData,
}

/// RGB converter gain
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RgbCGain {
    /// 1x gain
    _1x,
    /// 4x gain
    _4x,
    /// 16x gain
    _16x,
    /// 60x gain
    _60x,
}

/// RGB converter interrupt persistence
///
/// This controls the RGB converter interrupt generation rate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RgbCInterruptPersistence {
    /// Every RGBC cycle generates an interrupt.
    Every,
    /// 1 clear channel value out of range.
    _1,
    /// 2 clear channel consecutive values out of range.
    _2,
    /// 3 clear channel consecutive values out of range.
    _3,
    /// 5 clear channel consecutive values out of range.
    _5,
    /// 10 clear channel consecutive values out of range.
    _10,
    /// 15 clear channel consecutive values out of range.
    _15,
    /// 20 clear channel consecutive values out of range.
    _20,
    /// 25 clear channel consecutive values out of range.
    _25,
    /// 30 clear channel consecutive values out of range.
    _30,
    /// 35 clear channel consecutive values out of range.
    _35,
    /// 40 clear channel consecutive values out of range.
    _40,
    /// 45 clear channel consecutive values out of range.
    _45,
    /// 50 clear channel consecutive values out of range.
    _50,
    /// 55 clear channel consecutive values out of range.
    _55,
    /// 60 clear channel consecutive values out of range.
    _60,
}

/// Result of measurement of all channels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AllChannelMeasurement {
    /// Red channel measurement.
    pub red: u16,
    /// Green channel measurement.
    pub green: u16,
    /// Blue channel measurement.
    pub blue: u16,
    /// Clear (unfiltered) channel measurement.
    pub clear: u16,
}
