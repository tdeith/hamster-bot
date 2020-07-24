#![no_std]

extern crate panic_itm; // panic handler

use core::cell::Cell;
use cortex_m::{
    iprint, iprintln,
    asm::bkpt,
    interrupt::{self, CriticalSection, Mutex},
    peripheral::{self as cortexPeripherals, ITM},
};
use f3::{
    hal::{
        prelude::*,
        delay::Delay,
        stm32f30x::{
            self,
            i2c1,
            Peripherals as boardPeripherals,
        },
    },
    lsm303dlhc::{self, I16x3},
    led::{Direction, Leds},
};

use crate::core::{
    power,
    GPIOA
};

enum MotorDriverState {
    Offline,
    Initializing,
    Zeroing,
    Ready,
}

struct MotorDriver {
    state: MotorDriverState,
}

static MOTOR_DEVICE: Mutex<RefCell<LedDriver>> = Mutex::new(RefCell::new(
    MotorDriver {
        state: MotorDriverState::Offline,
    }
));

#[interrupt]
pub fn init() {
    interrupt::free(|cs| {
        GPIOA::begin_init();
        GPIOA::write_config(|_, pin| {
            // TODO: What motor/controller am I using here?
        });
        GPIOA::close_init();
        let gpioa = GPIOA::get_device();
        MOTOR_DEVICE.borrow(cs).replace(MotorDriver {
            state: MotorDriverState::Ready,
        });
    });
}

#[interrupt]
pub fn kill() {
    interrupt::free(|cs| {
        GPIOA::write_pins(|gpioa| {
            // TODO: What motor/controller am I using here?
        });
        let power = power::POWER.borrow(cs).as_ptr();
        power.ahbanr.modify(|power| {
            // TODO: What motor/controller am I using here?
        });
    });
}

#[interrupt]
pub fn set_speed(speed: f32) {
    interrupt::free(|cs| {
        let device = MOTOR_DEVICE.borrow(cs).as_ptr();
        if device.state != MotorDriverState::Ready {
            panic!("Motor driver need to be initialized");
        }

        // TODO: What motor/controller am I using here?
    });
}

#[interrupt]
pub fn reset() {
    interrupt::free(|cs| {
        let device = MOTOR_DEVICE.borrow(cs).as_ptr();
        if device.state != LedDriverState::Ready {
            panic!("Motor Driver need to be initialized");
        }
    });
}