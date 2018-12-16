extern crate apds9960;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{new, destroy, BitFlags, Register, DEV_ADDR};

#[test]
fn can_create() {
    let sensor = new(&[]);
    destroy(sensor);
}

write_test!(can_enable, enable, ENABLE, BitFlags::PON);
write_test!(can_disable, disable, ENABLE, 0);
write_test!(can_enable_light, enable_light, ENABLE, BitFlags::AEN);
write_test!(can_disable_light, disable_light, ENABLE, 0);

read_test!(can_read_id, read_device_id, 0xAB, ID, 0xAB);
