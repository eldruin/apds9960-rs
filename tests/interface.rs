extern crate apds9960;
use apds9960::GestureDataThreshold;
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

write_test!(can_enable_gesture, enable_gesture, ENABLE, BitFlags::GEN);
write_test!(can_disable_gesture, disable_gesture, ENABLE, 0);
write_test!(can_enable_gesture_mode, enable_gesture_mode, GCONFIG4, BitFlags::GMODE);
write_test!(can_disable_gesture_mode, disable_gesture_mode, GCONFIG4, 0);
write_test!(en_gesture_int, enable_gesture_interrupts, GCONFIG4, BitFlags::GIEN);
write_test!(dis_gesture_int, disable_gesture_interrupts, GCONFIG4, 0);
write_test!(can_set_gprox_entry_th, set_gesture_proximity_entry_threshold, GPENTH, 0xAB, 0xAB);
write_test!(can_set_gprox_exit_th, set_gesture_proximity_exit_threshold, GPEXTH, 0xAB, 0xAB);

write_test!(set_goffset_u, set_gesture_up_offset, GOFFSET_U, 55, 55);
write_test!(set_goffset_d, set_gesture_down_offset, GOFFSET_D, 55, 55);
write_test!(set_goffset_l, set_gesture_left_offset, GOFFSET_L, 55, 55);
write_test!(set_goffset_r, set_gesture_right_offset, GOFFSET_R, 55, 55);

#[test]
fn can_set_goffsets() {
    let data = vec![
        Register::GOFFSET_U,
        55,
        i8::from(-56) as u8,
        100,
        i8::from(-101) as u8,
    ];
    let trans = [I2cTrans::write(DEV_ADDR, data)];
    let mut sensor = new(&trans);
    sensor.set_gesture_offsets(55, -56, 100, -101).unwrap();
    destroy(sensor);
}

read_test!(can_read_id, read_device_id, 0xAB, ID, 0xAB);
read_test!(can_read_gvalid_true,  is_gesture_data_valid, true, GSTATUS, BitFlags::GVALID);
read_test!(can_read_gvalid_false, is_gesture_data_valid, false, GSTATUS, 0);
read_test!(can_read_gfifolvl, read_gesture_data_level, 15, GFLVL, 15);
read_test!(can_read_g_overfl,  has_gesture_data_overflown, true, GSTATUS, BitFlags::GFOV);
read_test!(can_read_g_not_overfl, has_gesture_data_overflown, false, GSTATUS, 0);

macro_rules! set_gdata_level_th_test {
    ($name:ident, $variant:ident, $value:expr) => {
        write_test!(
            $name,
            set_gesture_data_level_threshold,
            GCONFIG1,
            $value,
            GestureDataThreshold::$variant
        );
    };
}
set_gdata_level_th_test!(set_gdata_level_th1, Th1, 0);
set_gdata_level_th_test!(set_gdata_level_th4, Th4, BitFlags::GFIFOTH0);
set_gdata_level_th_test!(set_gdata_level_th8, Th8, BitFlags::GFIFOTH1);
set_gdata_level_th_test!(set_gdata_level_th16, Th16, BitFlags::GFIFOTH1 | BitFlags::GFIFOTH0);
