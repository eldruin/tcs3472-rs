extern crate linux_embedded_hal;
extern crate tcs3472;

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

    let clear = sensor.read_clear_channel().unwrap();
    let red = sensor.read_red_channel().unwrap();
    let green = sensor.read_green_channel().unwrap();
    let blue = sensor.read_blue_channel().unwrap();

    println!(
        "Measurements: clear = {}, red = {}, green = {}, blue = {}",
        clear, red, green, blue
    );
}
