extern crate apds9960;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy, new, BitFlags, Register, DEFAULT_CONFIG1, DEV_ADDR};

#[test]
fn can_create() {
    let sensor = new(&[]);
    destroy(sensor);
}

write_test!(can_enable, enable, ENABLE, BitFlags::PON);
write_test!(can_disable, disable, ENABLE, 0);

read_test!(can_read_id, read_device_id, 0xAB, ID, 0xAB);

write_test!(can_enable_wait, enable_wait, ENABLE, BitFlags::WEN);
write_test!(can_disable_wait, disable_wait, ENABLE, 0);
write_test!(
    en_wlong,
    enable_wait_long,
    CONFIG1,
    DEFAULT_CONFIG1 | BitFlags::WLONG
);
write_test!(dis_wlong, disable_wait_long, CONFIG1, DEFAULT_CONFIG1);
write_test!(set_wtime, set_wait_time, WTIME, 0x0F, 0x0F);

empty_write_test!(force_int, force_interrupt, IFORCE);
empty_write_test!(clear_ints, clear_interrupts, AICLEAR);
