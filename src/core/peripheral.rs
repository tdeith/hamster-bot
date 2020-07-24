use core::RefCell;
pub use f3::hal::stm32f30x::Peripherals as stmPeripherals;
pub use cortex_m::{
    Peripherals as ctxPeripherals,
    Mutex,
};

pub static STM_PERIPHERAL: Mutex<RefCell<stmPeripherals>> =
    Mutex::new(RefCell::new(stmPeripherals::take().unwrap()));

pub static CTX_PERIPHERAL: Mutex<RefCell<ctxPeripherals>> =
    Mutex::new(RefCell::new(ctxPeripherals::take().unwrap()));
