use rppal::i2c::I2c;

const REG_CONTROL_HUM: u8 = 0xF2;
const OVERSAMPLE_HUM: u8 = 2;

fn main() {
    let mut i2c = I2c::with_bus(1).unwrap();
    match i2c.set_slave_address(0x76) {
        Ok(_) => {println!("Set slave adress.")},
        Err(error) => {println!("Failed to set adress: {:?}", error)}
    }

    // oversample setting for humidity register - page 26
    match i2c.write(&[REG_CONTROL_HUM, OVERSAMPLE_HUM]) {
        Ok(_) => {println!("Set oversample setting for humidity register.")},
        Err(error) => {println!("Failed to set  oversample setting for humidity register: {:?}", error)}
    } 
}