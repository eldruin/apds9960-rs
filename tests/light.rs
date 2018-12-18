extern crate apds9960;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy, new, BitFlags, Register, DEV_ADDR};

write_test!(enable, enable_light, ENABLE, BitFlags::AEN);
write_test!(disable, disable_light, ENABLE, 0);
write_test!(en_int, enable_light_interrupts, ENABLE, BitFlags::AIEN);
write_test!(dis_int, disable_light_interrupts, ENABLE, 0);
write_test!(set_atime, set_light_integration_time, ATIME, 0x0F, 0x0F);

#[test]
fn set_low_threshold() {
    let trans = [I2cTrans::write(DEV_ADDR, vec![Register::AILTL, 0xCD, 0xAB])];
    let mut sensor = new(&trans);
    sensor.set_light_low_threshold(0xABCD).unwrap();
    destroy(sensor);
}

#[test]
fn set_high_threshold() {
    let trans = [I2cTrans::write(DEV_ADDR, vec![Register::AIHTL, 0xCD, 0xAB])];
    let mut sensor = new(&trans);
    sensor.set_light_high_threshold(0xABCD).unwrap();
    destroy(sensor);
}

read_test!(is_valid,  is_light_data_valid, true, STATUS, BitFlags::AVALID);
read_test!(is_not_valid, is_light_data_valid, false, STATUS, 0);

macro_rules! read_data_test {
    ($name:ident, $method:ident, $expected:expr, $($reg:ident, [$($value:expr),*]),*) => {
        #[test]
        fn $name() {
            let trans = [
                $(
                    I2cTrans::write_read(DEV_ADDR, vec![Register::$reg], vec![$($value,)*]),
                )*
            ];
            let mut sensor = new(&trans);
            let reading = sensor.$method().unwrap();
            assert_eq!($expected, reading);
            destroy(sensor);
        }
    };
}

read_data_test!(
    cdata,
    read_light_clear,
    0xABCD,
    STATUS,
    [BitFlags::AVALID],
    CDATAL,
    [0xCD, 0xAB]
);
read_data_test!(
    rdata,
    read_light_red,
    0xABCD,
    STATUS,
    [BitFlags::AVALID],
    RDATAL,
    [0xCD, 0xAB]
);
read_data_test!(
    gdata,
    read_light_green,
    0xABCD,
    STATUS,
    [BitFlags::AVALID],
    GDATAL,
    [0xCD, 0xAB]
);
read_data_test!(
    bdata,
    read_light_blue,
    0xABCD,
    STATUS,
    [BitFlags::AVALID],
    BDATAL,
    [0xCD, 0xAB]
);

#[test]
fn cannot_read_light_if_not_valid() {
    let trans = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::STATUS],
        vec![0],
    )];
    let mut sensor = new(&trans);
    assert_would_block!(sensor.read_light_clear());
    destroy(sensor);
}
