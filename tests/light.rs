extern crate apds9960;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{new, destroy, BitFlags, Register, DEV_ADDR};

write_test!(can_enable_light, enable_light, ENABLE, BitFlags::AEN);
write_test!(can_disable_light, disable_light, ENABLE, 0);

