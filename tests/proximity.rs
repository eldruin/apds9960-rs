extern crate apds9960;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{new, destroy, BitFlags, Register, DEFAULT_CONFIG2, DEV_ADDR};

write_test!(can_enable_proximity, enable_proximity, ENABLE, BitFlags::PEN);
write_test!(can_disable_proximity, disable_proximity, ENABLE, 0);
write_test!(en_prox_int, enable_proximity_interrupts, ENABLE, BitFlags::PIEN);
write_test!(dis_prox_int, disable_proximity_interrupts, ENABLE, 0);
write_test!(en_prox_sat_int, enable_proximity_saturation_interrupts, CONFIG2, BitFlags::PSIEN | DEFAULT_CONFIG2);
write_test!(dis_prox_sat_int, disable_proximity_saturation_interrupts, CONFIG2, DEFAULT_CONFIG2);

write_test!(set_prox_low_th, set_proximity_low_threshold, PILT, 0xAB, 0xAB);
write_test!(set_prox_high_th, set_proximity_high_threshold, PIHT, 0xAB, 0xAB);

write_test!(set_prox_ur_off, set_proximity_up_right_offset, POFFSET_UR, 55, 55);
write_test!(set_prox_fl_off, set_proximity_down_left_offset, POFFSET_DL, 55, 55);

#[test]
fn can_set_poffsets() {
    let data = vec![
        Register::POFFSET_UR,
        55,
        i8::from(-56) as u8,
    ];
    let trans = [I2cTrans::write(DEV_ADDR, data)];
    let mut sensor = new(&trans);
    sensor.set_proximity_offsets(55, -56).unwrap();
    destroy(sensor);
}

read_test!(can_read_pvalid_true,  is_proximity_data_valid, true, STATUS, BitFlags::PVALID);
read_test!(can_read_pvalid_false, is_proximity_data_valid, false, STATUS, 0);

read_test!(can_read_prox, read_proximity, 0x12, STATUS, BitFlags::PVALID, PDATA, 0x12);

#[test]
fn cannot_read_prox_if_not_valid() {
    let trans = [I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0])];
    let mut sensor = new(&trans);
    assert_would_block!(sensor.read_proximity());
    destroy(sensor);
}
