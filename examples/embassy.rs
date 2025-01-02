//! Enable the `async` feature in your `Cargo.toml`:
//! ```toml
//! tcs3472 = { version = "0.3.0", features = ["async"] }
//! ```

use embassy_executor::Spawner;
use linux_embedded_hal::I2cdev;
use tcs3472::{RgbCInterruptPersistence, Tcs3472};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Tcs3472::new(dev);
    sensor.enable().await.unwrap();
    sensor.enable_rgbc().await.unwrap();
    while !sensor.is_rgbc_status_valid().await.unwrap() {
        // wait for measurement to be available
    }

    let measurement = sensor.read_all_channels().await.unwrap();

    println!(
        "Measurements: clear = {}, red = {}, green = {}, blue = {}",
        measurement.clear, measurement.red, measurement.green, measurement.blue
    );
}
