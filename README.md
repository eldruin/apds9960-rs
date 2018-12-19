# Rust APDS9960 Digital Proximity, Ambient Light, RGB and Gesture Sensor Driver

[![crates.io](https://img.shields.io/crates/v/apds9960.svg)](https://crates.io/crates/apds9960)
[![Docs](https://docs.rs/apds9960/badge.svg)](https://docs.rs/apds9960)
[![Build Status](https://travis-ci.org/eldruin/apds9960-rs.svg?branch=master)](https://travis-ci.org/eldruin/apds9960-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/apds9960-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/apds9960-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the APDS9960 digital proximity, ambient light, RGB
and gesture sensor, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Enable/disable the sensor. See: `enable()`.
- Enable/disable delay between proximity and / or color / ambient light cycles. See: `enable_wait()`.
- Enable/disable long delay between proximity and / or color / ambient light cycles. See: `enable_wait_long()`.
- Set the waiting time between proximity and / or color / ambient light cycles. See: `set_wait_time()`.
- Force an interrupt. See: `force_interrupt()`.
- Clear all non-gesture interrupts. See: `clear_interrupts()`.
- Proximity:
    - Enable/disable the proximity sensor. See: `enable_proximity()`.
    - Enable/disable proximity interrupt generation. See: `enable_proximity_interrupts()`.
    - Enable/disable proximity saturation interrupt generation. See: `enable_proximity_saturation_interrupts()`.
    - Read the proximity data. See: `read_proximity()`.
    - Check whether the proximity data is valid. See: `is_proximity_data_valid()`.
    - Set the proximity interrupt low/high thresholds. See: `set_proximity_low_threshold()`.
    - Set the proximity offsets. See: `set_proximity_offsets()`.
    - Clear proximity interrupt. See: `clear_proximity_interrupt()`.
- Color / ambient light:
    - Enable/disable the color / ambient light sensor. See: `enable_light()`.
    - Enable/disable ambient light interrupt generation. See: `enable_light_interrupts()`.
    - Enable/disable ambient light saturation interrupt generation. See:  `enable_light_saturation_interrupts()`.
    - Check whether the color / ambient light data is valid. See: `is_light_data_valid()`.
    - Read the color / ambient light data. See: `read_light()`.
    - Set the color / ambient light integration time. See: `set_light_integration_time()`.
    - Set the clear light channel interrupt low/high thresholds. See: `set_light_low_threshold()`.
    - Clear ambient light interrupt. See: `clear_light_interrupt()`.
- Gesture recognition:
    - Enable/disable gesture recognition. See: `enable_gesture()`.
    - Enable/disable gesture mode. See: `enable_gesture_mode()`.
    - Enable/disable gesture interrupts. See: `enable_gesture_interrupts()`.
    - Read whether there is valid gesture data available. See: `is_gesture_data_valid()`.
    - Read the amount of gesture data available. See: `read_gesture_data_level()`.
    - Set the threshold of amount of available gesture data. See: `set_gesture_data_level_threshold()`.
    - Read whether the gesture data has overflown. See: `has_gesture_data_overflown()`.
    - Read the gesture data. See: `read_gesture_data()`.
    - Set the gesture proximity entry/exit thresholds. See: `set_gesture_proximity_entry_threshold()`.
    - Set the gesture offsets. See: `set_gesture_offsets()`.
- Read the device ID. See: `read_device_id()`.

## The device

The APDS-9960 device features advanced gesture detection, proximity detection, digital ambient
light sense (ALS) and color sense (RGBC).

The communication is done through an I2C bidirectional bus.

Datasheet:
- [APDS9960](https://docs.broadcom.com/docs/AV02-4191EN)

## Usage example

Please find additional examples in this repository: [apds9960-examples]

[apds9960-examples]: https://github.com/eldruin/apds9960-examples

```rust
extern crate embedded_hal;
extern crate linux_embedded_hal;
#[macro_use]
extern crate nb;
extern crate apds9960;

use linux_embedded_hal::I2cdev;
use apds9960::Apds9960;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Apds9960::new(dev);
    sensor.enable().unwrap();
    sensor.enable_proximity().unwrap();
    loop {
        let p = block!(sensor.read_proximity()).unwrap();
        println!("Proximity: {}", p);
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

