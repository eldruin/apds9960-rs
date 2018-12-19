extern crate embedded_hal;
extern crate linux_embedded_hal;
#[macro_use]
extern crate nb;
extern crate apds9960;

use linux_embedded_hal::I2cdev;
use apds9960::Apds9960;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Apds9960::new(dev);
    sensor.enable().unwrap();
    sensor.enable_proximity().unwrap();
    loop {
        let p = block!(sensor.read_proximity()).unwrap();
        println!("Proximity: {}", p);
    }
}
