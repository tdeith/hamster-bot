// Compiler macros to specify that:
#![deny(unsafe_code)]      // Unsafe code will not compile (severely limits hardware faults)
#![no_main]                // Do not use the standard `main` entrypoint. (Calling the function `main` is fine, though.)
#![no_std]                 // Do not bake the `std` library, which is otherwise linked by default

use cortex_m::interrupt;


/*
 These import modules into the main file, making their contents available on compile.
 C-equivalent is including header files, though Rust toolchains gives better
 type-hints via modules.
 */
mod core;
mod imu;
mod motors;
mod leds;


#[entry] // `cortex-m` macro to use this function as the program entrypoint.
fn main() -> ! {

    // Initializing drivers. Drivers are responsible for initializing any of their dependencies.
    imu::imu_driver::init();
    motors::tilt_motor_driver::init();
    motors::drive_motor_driver::init();
    leds::leds::init();

    loop {
        /*
            interrupts are `cortex_m`'s method of providing concurrency, with no other scheduler.
            This pattern allows priority interrupts to take over processing, without necessarily
            waiting for contestable resources to be released.
            Interruptable functions require a context for when the interrupt returns control.
            This context is the `cs` object being dealt with here.
         */
        interrupt::free(|cs| {
            /*
             The syntax you're seeing here is Rust's anonymous-function syntax - `|a| { do_thing(a); }`
             The relevant takeaway is that anything in the body of the `free(...)` call is run
             interruptably.
             */
            if imu::imu_driver::read_accel(cs).z > 0.5 {
                // This is the PoC to make sure IMU is reading properly - shake the chip, LED turns on.
                leds::leds::set_led_brightness(6_u8, Some(100_u8));
            }
        });
        // Throttle main thread to 100hz. Deep-down-internally, this resolves to 10ms of `NoOp`
        core::clocks::delay(10);
    }
}
