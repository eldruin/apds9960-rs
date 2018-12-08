extern crate apds9960;
use apds9960::Apds9960;
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

const DEV_ADDR: u8 = 0x39;

struct Register;
impl Register {
    const ENABLE     : u8 = 0x80;
    const ID         : u8 = 0x92;
}
pub struct BitFlags;
impl BitFlags {
    const PON: u8 = 0b0000_0001;
}

fn new(transactions: &[I2cTrans]) -> Apds9960<I2cMock> {
    Apds9960::new(I2cMock::new(&transactions))
}

fn destroy(sensor: Apds9960<I2cMock>) {
    sensor.destroy().done();
}

#[test]
fn can_create() {
    let sensor = new(&[]);
    destroy(sensor);
}

#[test]
fn can_enable() {
    let trans = [I2cTrans::write(DEV_ADDR, vec![Register::ENABLE, BitFlags::PON])];
    let mut sensor = new(&trans);
    sensor.enable().unwrap();
    destroy(sensor);
}

#[test]
fn can_disable() {
    let trans = [I2cTrans::write(DEV_ADDR, vec![Register::ENABLE, 0])];
    let mut sensor = new(&trans);
    sensor.disable().unwrap();
    destroy(sensor);
}

#[test]
fn can_read_device_id() {
    let trans = [I2cTrans::write_read(DEV_ADDR, vec![Register::ID], vec![0xAB])];
    let mut sensor = new(&trans);
    let id = sensor.read_device_id().unwrap();
    assert_eq!(0xAB, id);
    destroy(sensor);
}
