use linux_embedded_hal::{Delay, I2cdev};
use bme280::BME280;

fn main() {

    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);

    bme280.init().unwrap();

    let measurements = bme280.measure().unwrap();

    println!("Relative Humidity = {}%", measurements.humidity);
    println!("Temperature = {} deg C", measurements.temperature);
    println!("Pressure = {} pascals", measurements.pressure);
}