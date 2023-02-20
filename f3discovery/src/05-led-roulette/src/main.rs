#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let led_count = leds.len();
    let half_period = 100_u16;

    loop {
        for i in 0..led_count {
            let next = (i + 1) % led_count;

            leds[next].on().ok();
            delay.delay_ms(half_period);
            leds[i].off().ok();
            delay.delay_ms(half_period);
        }
    }
}
