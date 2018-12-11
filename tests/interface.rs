extern crate apds9960;
use apds9960::{Apds9960, GestureDataThreshold};
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

const DEV_ADDR: u8 = 0x39;

struct Register;
impl Register {
    const ENABLE     : u8 = 0x80;
    const PILT       : u8 = 0x89;
    const PIHT       : u8 = 0x8B;
    const ID         : u8 = 0x92;
    const STATUS     : u8 = 0x93;
    const PDATA      : u8 = 0x9C;
    const GPENTH     : u8 = 0xA0;
    const GPEXTH     : u8 = 0xA1;
    const GCONFIG1   : u8 = 0xA2;
    const GOFFSET_U: u8 = 0xA4;
    const GOFFSET_D: u8 = 0xA5;
    const GOFFSET_L: u8 = 0xA6;
    const GOFFSET_R: u8 = 0xA7;
    const GCONFIG4   : u8 = 0xAB;
    const GFLVL      : u8 = 0xAE;
    const GSTATUS    : u8 = 0xAF;
}
pub struct BitFlags;
impl BitFlags {
    const PON: u8 = 1;
    const PEN: u8 = 1 << 2;
    const GEN: u8 = 1 << 6;
    const PVALID: u8 = 1 << 1;
    const GMODE: u8 = 1;
    const GIEN: u8 = 1 << 1;
    const GVALID: u8 = 1;
    const GFOV: u8 = 1 << 1;
    const GFIFOTH1: u8 = 0b1000_0000;
    const GFIFOTH0: u8 = 0b0100_0000;
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
    ($name:ident, $method:ident, $reg:ident, $value:expr $(,$arg:expr)*) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![Register::$reg, $value])];
            let mut sensor = new(&trans);
            sensor.$method($( $arg ),*).unwrap();
            destroy(sensor);
        }
    };
}

write_test!(can_enable, enable, ENABLE, BitFlags::PON);
write_test!(can_disable, disable, ENABLE, 0);
write_test!(can_enable_proximity, enable_proximity, ENABLE, BitFlags::PEN);
write_test!(can_disable_proximity, disable_proximity, ENABLE, 0);
write_test!(can_enable_gesture, enable_gesture, ENABLE, BitFlags::GEN);
write_test!(can_disable_gesture, disable_gesture, ENABLE, 0);
write_test!(can_enable_gesture_mode, enable_gesture_mode, GCONFIG4, BitFlags::GMODE);
write_test!(can_disable_gesture_mode, disable_gesture_mode, GCONFIG4, 0);
write_test!(en_gesture_int, enable_gesture_interrupts, GCONFIG4, BitFlags::GIEN);
write_test!(dis_gesture_int, disable_gesture_interrupts, GCONFIG4, 0);
write_test!(can_set_gprox_entry_th, set_gesture_proximity_entry_threshold, GPENTH, 0xAB, 0xAB);
write_test!(can_set_gprox_exit_th, set_gesture_proximity_exit_threshold, GPEXTH, 0xAB, 0xAB);

write_test!(set_prox_low_th, set_proximity_low_threshold, PILT, 0xAB, 0xAB);
write_test!(set_prox_high_th, set_proximity_high_threshold, PIHT, 0xAB, 0xAB);

write_test!(set_goffset_u, set_gesture_up_offset, GOFFSET_U, 55, 55);
write_test!(set_goffset_d, set_gesture_down_offset, GOFFSET_D, 55, 55);
write_test!(set_goffset_l, set_gesture_left_offset, GOFFSET_L, 55, 55);
write_test!(set_goffset_r, set_gesture_right_offset, GOFFSET_R, 55, 55);

#[test]
fn can_set_goffset() {
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

macro_rules! read_test {
    ($name:ident, $method:ident, $expected:expr, $($reg:ident, $value:expr),*) => {
        #[test]
        fn $name() {
            let trans = [
                $(
                    I2cTrans::write_read(DEV_ADDR, vec![Register::$reg], vec![$value]),
                )*
            ];
            let mut sensor = new(&trans);
            let value = sensor.$method().unwrap();
            assert_eq!($expected, value);
            destroy(sensor);
        }
    };
}

read_test!(can_read_id, read_device_id, 0xAB, ID, 0xAB);
read_test!(can_read_pvalid_true,  is_proximity_data_valid, true, STATUS, BitFlags::PVALID);
read_test!(can_read_pvalid_false, is_proximity_data_valid, false, STATUS, 0);
read_test!(can_read_gvalid_true,  is_gesture_data_valid, true, GSTATUS, BitFlags::GVALID);
read_test!(can_read_gvalid_false, is_gesture_data_valid, false, GSTATUS, 0);
read_test!(can_read_gfifolvl, read_gesture_data_level, 15, GFLVL, 15);
read_test!(can_read_g_overfl,  has_gesture_data_overflown, true, GSTATUS, BitFlags::GFOV);
read_test!(can_read_g_not_overfl, has_gesture_data_overflown, false, GSTATUS, 0);


read_test!(can_read_prox, read_proximity, 0x12, STATUS, BitFlags::PVALID, PDATA, 0x12);

macro_rules! assert_would_block {
    ($result: expr) => {
        match $result {
            Err(nb::Error::WouldBlock) => (),
            _ => panic!("Would not block."),
        }
    };
}

#[test]
fn cannot_read_prox_if_not_valid() {
    let trans = [I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0])];
    let mut sensor = new(&trans);
    assert_would_block!(sensor.read_proximity());
    destroy(sensor);
}

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
