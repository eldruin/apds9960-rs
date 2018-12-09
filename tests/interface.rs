extern crate apds9960;
use apds9960::Apds9960;
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

const DEV_ADDR: u8 = 0x39;

struct Register;
impl Register {
    const ENABLE     : u8 = 0x80;
    const ID         : u8 = 0x92;
    const PDATA      : u8 = 0x9C;
}
pub struct BitFlags;
impl BitFlags {
    const PON: u8 = 0b0000_0001;
    const PEN: u8 = 0b0000_0100;
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

macro_rules! write_test {
    ($name:ident, $method:ident, $reg:ident, $value:expr) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![Register::$reg, $value])];
            let mut sensor = new(&trans);
            sensor.$method().unwrap();
            destroy(sensor);
        }
    };
}

write_test!(can_enable, enable, ENABLE, BitFlags::PON);
write_test!(can_disable, disable, ENABLE, 0);
write_test!(can_enable_proximity, enable_proximity, ENABLE, BitFlags::PEN);
write_test!(can_disable_proximity, disable_proximity, ENABLE, 0);

macro_rules! read_test {
    ($name:ident, $method:ident, $reg:ident, $value:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write_read(DEV_ADDR, vec![Register::$reg], vec![$value])];
            let mut sensor = new(&trans);
            let value = sensor.$method().unwrap();
            assert_eq!($expected, value);
            destroy(sensor);
        }
    };
}

read_test!(can_read_id, read_device_id, ID, 0xAB, 0xAB);
read_test!(can_read_prox, read_proximity, PDATA, 0x12, 0x12);
