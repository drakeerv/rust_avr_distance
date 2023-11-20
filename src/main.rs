#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;

fn num_to_bytes(num: u16) -> [u8; 5] {
    if num == 0 { return [0, 0, 0, 0, 48] }

    let mut bytes = [0u8; 5];
    let mut num = num;
    let mut i = 0;
    while num > 0 {
        bytes[i] = (num % 10) as u8 + 48;
        num /= 10;
        i += 1;
    }
    bytes
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut trig = pins.d9.into_output();
    let echo = pins.d10.into_pull_up_input();

    loop {
        trig.set_high();
        arduino_hal::delay_ms(10);
        trig.set_low();

        let mut pulse_start: u16 = 0;

        while echo.is_low() {
            arduino_hal::delay_us(1);

            if pulse_start == 65535 { break }
            pulse_start += 1;
        }

        let mut pulse_end: u16 = 0;

        while echo.is_high() {
            arduino_hal::delay_us(1);

            if pulse_end == 65535 { break }
            pulse_end += 1;
        }

        let distance = if pulse_end > pulse_start {
            ((pulse_end - pulse_start) / 29) / 2
        } else {
            0
        };

        serial.write_str("Distance:").unwrap();
        
        let bytes = num_to_bytes(distance);
        for i in (0..bytes.len()).rev() {
            serial.write_byte(bytes[i]);
        }

        serial.write_str("\r\n").unwrap();
    }
}
