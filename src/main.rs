use linux_embedded_hal::{Delay, I2cdev};
use bme280::BME280;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);

    bme280.init().unwrap();

    let measurements = bme280.measure().unwrap();

    println!("{:?}", args);

    match args[1].as_str() {
        "temp" => println!("{}", measurements.temperature),
        "pressure" => println!("{}", measurements.pressure),
        "humidity" => println!("{}", measurements.humidity),
        _ => {}
    }
}