#![no_std]

pub use cortex_m::{
    Peripherals as ctxPeripherals,
    interrupt,
    Mutex,
    CriticalSection,
};
pub use stm32f4::GPIOA as GPIOA_REGISTER;

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

static GPIOA: Mutex<RefCell<GPIOA_REGISTER>> =
    Mutex::new(RefCell::new(STM_PERIPHERAL.borrow().GPIOA));

#[interrupt]
pub fn begin_init() {
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow();
        if status_mut != Status::Offline {
            panic!("Cannot re-initialize GPIOA");
        }
        status_mut.replace(Status::Initializing);

        // power on the GPIOA peripheral
        let mut power = POWER.borrow().as_mut_ref();
        power.ahbanr.modify(|_, pin| pin.iopeen().set_bit());
    });
}

#[interrupt]
pub fn write_config(config: F)
    where F: RunOnce<GPIOA, &mut GPIOA>
{
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow();
        if status_mut != Status::Initializing {
            panic!("Must be initializing GPIOA to write config");
        }
        let mut gpioa = GPIOA.borrow(cs).as_ptr();
        gpioa.moder.modify(config);
    });
}

#[interrupt]
pub fn close_init() {
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow();
        if status_mut != Status::Initializing {
            panic!("Not currently initializing GPIOA");
        }
        let mut gpioa = GPIOA.borrow(cs).as_ptr();
        status_mut.replace(Status::Ready);
    });
}

#[interrupt]
pub fn write_pins(f: F)
    where F: FnOnce<&mut GPIOA_REGISTER>
{
    interrupt::free(|cs| {
        let mut status_mut = STATUS.borrow();
        if status_mut != Status::Ready {
            panic!("GPIOA not initialized");
        }
        let mut gpioa = GPIOA.borrow(cs).as_ptr();
        gpioa.odr.write(f);
    });
}

