#![no_std]

use core::cell::RefCell;
use super::peripheral::{
    STM_PERIPHERAL,
    ctxPeripherals
};
use cortex_m::interrupt::{self, CriticalSection, Mutex};

pub static POWER: Mutex<RefCell<ctxPeripherals>> =
    Mutex::new(RefCell::new(STM_PERIPHERAL.borrow().RCC));

#[interrupt]
pub fn kill_all() {
    interrupt::disable();
    interrupt::free(|cs| {
        for device in POWER.borrow(cs).as_ptr() {
            device.modify(|_, pin| pin.iopeen().reset_bit());
        }
    })
}
