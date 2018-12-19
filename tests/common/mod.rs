use apds9960::Apds9960;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

pub const DEV_ADDR: u8 = 0x39;

pub struct Register;
#[allow(unused)]
impl Register {
    pub const ENABLE     : u8 = 0x80;
    pub const ATIME      : u8 = 0x81;
    pub const WTIME      : u8 = 0x83;
    pub const AILTL      : u8 = 0x84;
    pub const AIHTL      : u8 = 0x86;
    pub const PILT       : u8 = 0x89;
    pub const PIHT       : u8 = 0x8B;
    pub const CONFIG1    : u8 = 0x8D;
    pub const CONFIG2    : u8 = 0x90;
    pub const ID         : u8 = 0x92;
    pub const STATUS     : u8 = 0x93;
    pub const CDATAL     : u8 = 0x94;
    pub const RDATAL     : u8 = 0x96;
    pub const GDATAL     : u8 = 0x98;
    pub const BDATAL     : u8 = 0x9A;
    pub const PDATA      : u8 = 0x9C;
    pub const POFFSET_UR : u8 = 0x9D;
    pub const POFFSET_DL : u8 = 0x9E;
    pub const GPENTH     : u8 = 0xA0;
    pub const GPEXTH     : u8 = 0xA1;
    pub const GCONFIG1   : u8 = 0xA2;
    pub const GOFFSET_U: u8 = 0xA4;
    pub const GOFFSET_D: u8 = 0xA5;
    pub const GOFFSET_L: u8 = 0xA6;
    pub const GOFFSET_R: u8 = 0xA7;
    pub const GCONFIG4   : u8 = 0xAB;
    pub const GFLVL      : u8 = 0xAE;
    pub const GSTATUS    : u8 = 0xAF;
    pub const IFORCE     : u8 = 0xE4;
    pub const PICLEAR    : u8 = 0xE5;
    pub const GFIFO_U    : u8 = 0xFC;
}

pub struct BitFlags;
#[allow(unused)]
impl BitFlags {
    pub const PON: u8 = 1;
    pub const AEN: u8 = 1 << 1;
    pub const PEN: u8 = 1 << 2;
    pub const WEN: u8 = 1 << 3;
    pub const AIEN: u8 = 1 << 4;
    pub const PIEN: u8 = 1 << 5;
    pub const GEN: u8 = 1 << 6;
    pub const WLONG: u8 = 1 << 1;
    pub const CPSIEN: u8 = 1 << 6;
    pub const PSIEN: u8 = 1 << 7;
    pub const AVALID: u8 = 1;
    pub const PVALID: u8 = 1 << 1;
    pub const GMODE: u8 = 1;
    pub const GIEN: u8 = 1 << 1;
    pub const GVALID: u8 = 1;
    pub const GFOV: u8 = 1 << 1;
    pub const GFIFOTH1: u8 = 1 << 7;
    pub const GFIFOTH0: u8 = 1 << 6;
}

#[allow(unused)]
pub const DEFAULT_CONFIG1: u8 = 0x40;

#[allow(unused)]
pub const DEFAULT_CONFIG2: u8 = 1;

pub fn new(transactions: &[I2cTrans]) -> Apds9960<I2cMock> {
    Apds9960::new(I2cMock::new(&transactions))
}

pub fn destroy(sensor: Apds9960<I2cMock>) {
    sensor.destroy().done();
}

#[macro_export]
macro_rules! empty_write_test {
    ($name:ident, $method:ident, $reg:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![Register::$reg])];
            let mut sensor = new(&trans);
            sensor.$method().unwrap();
            destroy(sensor);
        }
    };
}

#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! assert_would_block {
    ($result: expr) => {
        match $result {
            Err(nb::Error::WouldBlock) => (),
            _ => panic!("Would not block."),
        }
    };
}
