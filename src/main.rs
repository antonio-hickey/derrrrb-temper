#![no_std]
#![no_main]

use core::marker::PhantomData;
use panic_halt as _;
mod mlx90614;
pub use crate::mlx90614::wake_mlx90614;
mod types;
pub use crate::types::{ic, Error, SlaveAddr};
mod register_access;

// MLX90614 device driver
#[derive(Debug)]
struct Mlx9061x<I2C, IC> {
    // The concrete I²C device implementation.
    i2c: I2C,
    eeprom_write_delay_ms: u8,
    address: u8,
    _ic: PhantomData<IC>,
}


#[arduino_hal::entry]
fn entry_point() -> ! {
    // pins on the board
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    // serial buffer to write output 
    let mut sb = arduino_hal::default_serial!(peripherals, pins, 57600);

    // prepare sensor's bus controller
    let addr = SlaveAddr::default();
    let i2c = arduino_hal::I2c::new(
        peripherals.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    // Infrared sensor
    let mut ir_sensor = Mlx9061x::new_mlx90614(i2c, addr, 5).unwrap(); 

    // grab the current tempature of the nail in fahrenheit 
    // and write it out to the serial buffer for now, but LCD
    // display next.
    loop {
        let temp = ir_sensor.obj1_temp_f().unwrap();

        // TODO: output tempature to lcd display
        ufmt::uwriteln!(&mut sb, "current nail tempature: {}°F", temp).unwrap();
    }
}
