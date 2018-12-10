extern crate apds9960;
use apds9960::Apds9960;
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

const DEV_ADDR: u8 = 0x39;

struct Register;
impl Register {
    const GFLVL      : u8 = 0xAE;
    const GSTATUS    : u8 = 0xAF;
    const GFIFO_U    : u8 = 0xFC;
}
pub struct BitFlags;
impl BitFlags {
    const GVALID: u8 = 1;
}

fn new(transactions: &[I2cTrans]) -> Apds9960<I2cMock> {
    Apds9960::new(I2cMock::new(&transactions))
}

fn destroy(sensor: Apds9960<I2cMock>) {
    sensor.destroy().done();
}

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
    assert_would_block!(sensor.read_gesture_data(&mut []));
    destroy(sensor);
}
