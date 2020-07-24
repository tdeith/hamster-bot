#![no_std]

pub use cortex_m::{
    Peripherals as ctxPeripherals,
    interrupt::{self, Mutex},
};
pub use stm32f4::GPIOA as GPIOA_register;

use core::cell::{Cell, RefCell};
use super::{
    peripheral::STM_PERIPHERAL,
    power::POWER,
};
use core::borrow::BorrowMut;


enum Status {
    Offline,
    Initializing,
    Ready,
}

static STATUS: Mutex<Cell<Status>> = Mutex::new(Cell::new(Status::Offline));

pub static GPIO_A: Mutex<RefCell<GPIOA_register>> =
    Mutex::new(RefCell::new(STM_PERIPHERAL.borrow_mut().as_ptr().GPIOA));

#[interrupt]
pub fn begin_init() {
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow(cs);
        if status_mut != Status::Offline {
            panic!("Cannot re-initialize GPIOA");
        }
        status_mut.replace(Status::Initializing);

        // power on the GPIOA peripheral
        let mut power = POWER.borrow(cs).as_mut_ref();
        power.ahbenr.modify(|_, pin| pin.iopeen().set_bit());
    });
}

#[interrupt]
pub fn write_config(config: F)
    where F: RunOnce<GPIOA_register, &mut GPIOA_register>
{
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow(cs);
        if status_mut != Status::Initializing {
            panic!("Must be initializing GPIOA to write config");
        }
        GPIO_A.borrow(cs).as_mut_ref().moder.modify(config);
    });
}

#[interrupt]
pub fn close_init() {
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow(cs);
        if status_mut != Status::Initializing {
            panic!("Not currently initializing GPIOA");
        }
        status_mut.replace(Status::Ready);
    });
}

pub fn get_device() -> &'static mut GPIOA_register {
    let mut status_mut = STATUS.borrow(cs);
    if status_mut != Status::Ready {
        panic!("GPIOA not initialized");
    }
    GPIO_A.borrow(cs).as_mut_ref()
}
