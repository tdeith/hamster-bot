#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::interrupt;


mod core;
mod imu;
mod motors;
mod leds;


#[entry]
fn main() -> ! {
    imu::imu_driver::init();
    motors::tilt_motor_driver::init();
    motors::drive_motor_driver::init();
    leds::leds::init();

    loop {
        interrupt::free(|cs| {
            if imu::imu_driver::read_accel(cs).z > 0.5 {
                leds::leds::set_led_brightness(6_u8, Some(100_u8));
            }
        });
        core::clocks::delay(10);
    }
}
