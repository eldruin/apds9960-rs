use apds9960::Apds9960;
#[cfg(target_os = "linux")]
use linux_embedded_hal::I2cdev;

#[cfg(not(target_os = "linux"))]
fn main() {
}

#[cfg(target_os = "linux")]
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
