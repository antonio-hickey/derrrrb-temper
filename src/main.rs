#![no_std]
#![no_main]

use panic_halt as _;
use mlx9061x::{Mlx9061x, SlaveAddr};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let addr = SlaveAddr::default();
    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    let mut sensor = Mlx9061x::new_mlx90614(i2c, addr, 5).unwrap(); 

    loop {
        let t = sensor.raw_ir_channel1().unwrap();
        ufmt::uwriteln!(&mut serial, "temp: {}", t).unwrap();
    }
}
