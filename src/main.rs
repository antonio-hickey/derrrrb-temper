#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    // Define peripherals (digital pins)
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is the LED marked "L" on my board
    let mut led = pins.d13.into_output();

    // Tell the board to set led pin to high volatage (pulled to supply voltage)
    led.set_high(); 

    // Toggle led, to prove the program is running on the board
    loop {
        led.toggle();
        arduino_hal::delay_ms(10000);
        led.toggle();
        arduino_hal::delay_ms(1000);
        led.toggle();
        arduino_hal::delay_ms(1000);
        led.toggle();
        arduino_hal::delay_ms(8000);
    }
}
