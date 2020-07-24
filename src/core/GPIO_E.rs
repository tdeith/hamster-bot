#![no_std]

pub use cortex_m::{
    Peripherals as ctxPeripherals,
    interrupt,
    Mutex,
};
pub use stm32f4::GPIOE;

use core::{Cell, RefCell};
use super::{
    peripheral::STM_PERIPHERAL,
    power::POWER,
};


enum Status {
    Offline,
    Initializing,
    Ready,
}

static STATUS: Mutex<Cell<Status>> = Mutex::new(Cell::new(Status::Offline));

static GPIOE: Mutex<RefCell<GPIOE>> =
    Mutex::new(RefCell::new(STM_PERIPHERAL.borrow().GPIOE));

#[interrupt]
pub fn begin_init() {
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow();
        if status_mut != Status::Offline {
            panic!("Cannot re-initialize GPIOE");
        }
        status_mut.replace(Status::Initializing);

        // power on the GPIOE peripheral
        let mut power = POWER.borrow().as_mut_ref();
        power.ahbenr.modify(|_, pin| pin.iopeen().set_bit());
    });
}

#[interrupt]
pub fn write_config(config: F)
    where F: RunOnce<GPIOE, &mut GPIOE>
{
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow();
        if status_mut != Status::Initializing {
            panic!("Must be initializing GPIOE to write config");
        }
        let mut gpioe = GPIOE.borrow().as_mut_ref();
        GPIOE.moder.modify(config);
    });
}

#[interrupt]
pub fn close_init() {
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow();
        if status_mut != Status::Initializing {
            panic!("Not currently initializing GPIOE");
        }
        let mut gpioe = GPIOE.borrow().as_mut_ref();
        status_mut.replace(Status::Ready);
    });
}

pub fn get_device() -> &'static mut GPIOE {
    let mut status_mut = STATUS.borrow();
    if status_mut != Status::Ready {
        panic!("GPIOE not initialized");
    }
    GPIOE.borrow().as_mut_ref()
}
