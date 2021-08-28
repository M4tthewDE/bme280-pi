use linux_embedded_hal::{Delay, I2cdev};
use bme280::BME280;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);

    bme280.init().unwrap();

    let measurements = bme280.measure().unwrap();

    let mut file = File::create("/home/alarm/services/node_exporter/bme280.prom").unwrap();
    file.write_all(format!("# HELP bme_current_temp current temperature recorded by bme280\n# TYPE bme_current_temp gauge\nbme_current_temp {}\n# HELP bme_current_hum current humidity recorded by bme280\n# TYPE bme_current_hum gauge\nbme_current_hum {}
        ", measurements.temperature, measurements.humidity).as_bytes()).unwrap();

    file.flush().unwrap();

    match args[1].as_str() {
        "temp" => println!("{:.2}", measurements.temperature),
        "pressure" => println!("{:.2}", measurements.pressure),
        "humidity" => println!("{:.2}", measurements.humidity),
        _ => {}
    }
}