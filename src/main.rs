#![no_std]
#![no_main]

use panic_halt as _;
use mlx9061x::{Mlx9061x, SlaveAddr};

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

    // grab the current infrared levels from channel 1 
    // and write the out to the serial buffer.
    loop {
        let ir_c1 = ir_sensor.raw_ir_channel1().unwrap();
        let ir_c2 = ir_sensor.raw_ir_channel1().unwrap();

        ufmt::uwriteln!(
            &mut sb, 
            "infrared channel 1: {} | infrared channel 2: {}", 
            ir_c1, ir_c2
        ).unwrap();
    }
}
