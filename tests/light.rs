extern crate apds9960;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{new, destroy, BitFlags, Register, DEV_ADDR};

write_test!(enable, enable_light, ENABLE, BitFlags::AEN);
write_test!(disable, disable_light, ENABLE, 0);

read_test!(is_valid,  is_light_data_valid, true, STATUS, BitFlags::AVALID);
read_test!(is_not_valid, is_light_data_valid, false, STATUS, 0);
