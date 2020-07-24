#![no_std]

extern crate panic_itm; // panic handler

use core::cell::RefCell;
use cortex_m::interrupt::{
    self, CriticalSection, Mutex,
};
use f3::{
    hal::{delay::Delay, prelude, time::MonoTimer},
    lsm303dlhc::{I16x3, Sensitivity},
    Lsm303dlhc,
};

enum ClockDriverState {
    Offline,
    Initializing,
    Ready,
}

struct ClockDriver {
    state: ClockDriverState,
    delay: Option<Delay>,
    mono_timer: Option<MonoTimer>,
}

static CLOCK: Mutex<RefCell<LedDriver>> = Mutex::new(RefCell::new(
    ClockDriver {
        state: ClockDriverState::Offline,
        delay: None,
        mono_timer: None,
    }
));


#[interrupt]
pub fn init() {
    interrupt::free(|cs| {
        let clock = CLOCK.borrow(cs);

        if clock.as_ptr().state != ClockDriverState::Offline {
            return;
        }
        clock.replace(ClockDriver {
            state: ClockDriverState::Initializing,
            delay: None,
            mono_timer: None,
        });

        let board_periph = super::peripheral::STM_PERIPHERAL.borrow(cs).as_ptr();
        let mut flash = board_periph.FLASH.constrain();

        let mut cortex_periph = super::peripheral::CORTEX_PERIPHERAL.borrow(cs).as_ptr();
        let mut clocks = cortex_periph.RCC.constrain()
            .cfgr.freeze(&mut flash.acr);

        let delay = Delay::new(&cortex_periph.SYST, clocks);
        let mono_timer = MonoTimer::new(&cortex_periph.DWT, clocks);

        clock.replace(ClockDriver {
            state: ClockDriverState::Ready,
            delay: Some(delay),
            mono_timer: Some(mono_timer),
        });
    });
}

#[interrupt]
pub fn delay(ms: usize) {
    interrupt::free(|cs| {
        CLOCK.borrow(cs).as_ptr()
            .delay()
            .unwrap("Must initialize clock before using delay function")
            .delay_ms(ms);
    });
}

pub fn get_timer(cs: &CriticalSection) -> Result<MonoTimer, String> {
    CLOCK.borrow(cs).as_ptr()
        .mono_timer()
        .ok_or("Must initialize clock before using timer")
}
