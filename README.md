# Rust TCS3472 RGB Color Light to Digital Converter with IR Filter Driver

[![crates.io](https://img.shields.io/crates/v/tcs3472.svg)](https://crates.io/crates/tcs3472)
[![Docs](https://docs.rs/tcs3472/badge.svg)](https://docs.rs/tcs3472)
[![Build Status](https://github.com/eldruin/tcs3472-rs/workflows/Build/badge.svg)](https://github.com/eldruin/tcs3472-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/tcs3472-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/tcs3472-rs?branch=master)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.65+-blue.svg)

This is a platform agnostic Rust driver for the TCS3472 RGB color light to
digital converter with IR filter, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Enable/disable the device.
- Enable/disable the RGB converter.
- Set RGB converter gain.
- Enable/disable the RGB converter interrupt generation.
- Set the RGB converter interrupt clear channel low/high thresholds.
- Set the RGB converter interrupt persistence.
- Set the number of integration cycles.
- Enable/disable the wait feature.
- Set the number of wait time cycles.
- Enable/disable the *wait long* setting.
- Read status of RGB converter.
- Read the clear (unfiltered) channel measurement.
- Read the red channel measurement.
- Read the green channel measurement.
- Read the blue channel measurement.
- Read the measurement of all channels at once.
- Read the device ID.

## The device
The TCS3472 device provides a digital return of red, green, blue (RGB), and
clear light sensing values. An IR blocking filter, integrated on-chip and
localized to the color sensing photodiodes, minimizes the IR spectral
component of the incoming light and allows color measurements to be made
accurately. The high sensitivity, wide dynamic range, and IR blocking
filter make the TCS3472 an ideal color sensor solution for use under
varying lighting conditions and through attenuating materials.

The TCS3472 color sensor has a wide range of applications including RGB LED
backlight control, solid-state lighting, health/fitness products,
industrial process controls and medical diagnostic equipment. In addition,
the IR blocking filter enables the TCS3472 to perform ambient light sensing
(ALS). Ambient light sensing is widely used in display-based products such
as cell phones, notebooks, and TVs to sense the lighting environment and
enable automatic display brightness for optimal viewing and power savings.
The TCS3472, itself, can enter a lower-power wait state between light
sensing measurements to further reduce the average power consumption.

Datasheet:
- [TCS3472](https://ams.com/documents/20143/36005/TCS3472_DS000390_2-00.pdf)

This driver is compatible with the devices TCS34725 and TCS34727.

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use linux_embedded_hal::I2cdev;
use tcs3472::Tcs3472;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Tcs3472::new(dev);
    sensor.enable().unwrap();
    sensor.enable_rgbc().unwrap();
    while !sensor.is_rgbc_status_valid().unwrap() {
        // wait for measurement to be available
    }
    let m = sensor.read_all_channels().unwrap();
    println!(
        "Measurements: clear = {}, red = {}, green = {}, blue = {}",
        m.clear, m.red, m.green, m.blue
    );
}
```

This driver also supports the `embedded-hal-async` traits if the `async` feature is enabled in the `Cargo.toml` file:

```toml
tcs3472 = { version = "0.3.0", features = ["async"] }
```

Example how it looks like when using the [Embassy](https://embassy.dev/) framework:

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    // embassy i2c setup details omitted
    let mut i2c = I2c::new(..);
    let mut sensor = Tcs3472::new(i2c);
    sensor.enable().await.unwrap();
    sensor.enable_rgbc().await.unwrap();
    while !sensor.is_rgbc_status_valid().await.unwrap() {
        // wait for measurement to be available
    };
    let m = sensor.read_all_channels().await.unwrap();
    defmt::info!(
        "Measurements: clear = {}, red = {}, green = {}, blue = {}",
        m.clear, m.red, m.green, m.blue
    );
}
```

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.81.0 and up. It *might*
compile with older versions but that may change in any new patch release.

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/tcs3472-rs/issues).

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
