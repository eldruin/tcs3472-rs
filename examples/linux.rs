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
