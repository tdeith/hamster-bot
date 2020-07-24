#![no_std]

pub use cortex_m::{
    Peripherals as ctxPeripherals,
    interrupt::{self, Mutex},
};
pub use stm32f4::GPIOE as GPIOE_register;

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

pub static GPIO_E: Mutex<RefCell<GPIOE_register>> =
    Mutex::new(RefCell::new(STM_PERIPHERAL.borrow_mut().as_ptr().GPIOE));

#[interrupt]
pub fn begin_init() {
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow(cs);
        if status_mut != Status::Offline {
            panic!("Cannot re-initialize GPIOE");
        }
        status_mut.replace(Status::Initializing);

        // power on the GPIOE peripheral
        let mut power = POWER.borrow(cs).as_mut_ref();
        power.ahbenr.modify(|_, pin| pin.iopeen().set_bit());
    });
}

#[interrupt]
pub fn write_config(config: F)
    where F: RunOnce<GPIOE_register, &mut GPIOE_register>
{
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow(cs);
        if status_mut != Status::Initializing {
            panic!("Must be initializing GPIOE to write config");
        }
        GPIO_E.borrow(cs).as_mut_ref().moder.modify(config);
    });
}

#[interrupt]
pub fn close_init() {
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow(cs);
        if status_mut != Status::Initializing {
            panic!("Not currently initializing GPIOE");
        }
        status_mut.replace(Status::Ready);
    });
}

pub fn get_device() -> &'static mut GPIOE_register {
    let mut status_mut = STATUS.borrow(cs);
    if status_mut != Status::Ready {
        panic!("GPIOE not initialized");
    }
    GPIO_E.borrow(cs).as_mut_ref()
}
