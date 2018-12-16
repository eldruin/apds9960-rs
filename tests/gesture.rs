extern crate apds9960;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{new, destroy, BitFlags, Register, DEV_ADDR};

macro_rules! read_data_test {
    ($name:ident, $method:ident, $expected:expr, $data_size:expr, $($reg:ident, [$($value:expr),*]),*) => {
        #[test]
        fn $name() {
            let trans = [
                $(
                    I2cTrans::write_read(DEV_ADDR, vec![Register::$reg], vec![$($value,)*]),
                )*
            ];
            let mut sensor = new(&trans);
            let mut data = [0; $data_size];
            sensor.$method(&mut data).unwrap();
            assert_eq!($expected, data);
            destroy(sensor);
        }
    };
}

read_data_test!(
    array_smaller_than_level,
    read_gesture_data,
    [1, 1, 1],
    3,
    GSTATUS,
    [BitFlags::GVALID],
    GFLVL,
    [1],
    GFIFO_U,
    [1, 1, 1]
);

read_data_test!(
    can_read_gesture_data,
    read_gesture_data,
    [1, 1, 1, 1],
    4,
    GSTATUS,
    [BitFlags::GVALID],
    GFLVL,
    [1],
    GFIFO_U,
    [1, 1, 1, 1]
);

read_data_test!(
    can_read_gesture_multiple_data,
    read_gesture_data,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    15,
    GSTATUS,
    [BitFlags::GVALID],
    GFLVL,
    [4],
    GFIFO_U,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
);


macro_rules! assert_would_block {
    ($result: expr) => {
        match $result {
            Err(nb::Error::WouldBlock) => (),
            _ => panic!("Would not block."),
        }
    };
}

#[test]
fn cannot_read_gesture_if_not_valid() {
    let trans = [I2cTrans::write_read(DEV_ADDR, vec![Register::GSTATUS], vec![0])];
    let mut sensor = new(&trans);
    assert_would_block!(sensor.read_gesture_data(&mut [0; 4]));
    destroy(sensor);
}
