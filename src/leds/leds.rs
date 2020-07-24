#![no_std]

extern crate panic_itm; // panic handler

use core::cell::RefCell;
use cortex_m::{
    iprint, iprintln,
    asm::bkpt,
    interrupt::{
        self, CriticalSection, Mutex,
    },
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

use crate::core::GPIO_E;

enum LedDriverState {
    Offline,
    Ready,
}

struct LedDriver {
    state: LedDriverState,
    leds: Option<Leds>,
}

static LED_DEVICE: Mutex<RefCell<LedDriver>> = Mutex::new(RefCell::new(
    LedDriver {
        state: LedDriverState::Offline,
        leds: None,
    }
));

#[interrupt]
pub fn init() {
    interrupt::free(|cs| {
        GPIO_E::begin_init();
        GPIO_E::write_config(|_, pin| {
            pin.moder8().output();
            pin.moder9().output();
            pin.moder10().output();
            pin.moder11().output();
            pin.moder12().output();
            pin.moder13().output();
            pin.moder14().output();
            pin.moder15().output();
        });
        GPIO_E::close_init();
        let gpioe = GPIO_E::get_device();
        let leds = Leds::new(gpioe);
        LED_DEVICE.borrow(cs).replace(LedDriver {
            state: LedDriverState::Ready,
            leds: Some(leds),
        });
    });
}

#[interrupt]
pub fn set_led_brightness(led_idx: u8, intensity: Option<u8>) {
    let intensity = intensity.unwrap_or(255_u8);
    interrupt::free(|cs| {
        let device = LED_DEVICE.borrow(cs).as_ptr();
        if device.state != LedDriverState::Ready {
            panic!("Leds need to be initialized");
        }
        // TODO: PWM here to actually give intensity.
        device.leds[led_idx].on();
    });
}

#[interrupt]
pub fn reset_led(led_idx: u8) {
    interrupt::free(|cs| {
        let device = LED_DEVICE.borrow(cs).as_ptr();
        if device.state != LedDriverState::Ready {
            panic!("Leds need to be initialized");
        }
        device.leds[led_idx].off();
    });
}
