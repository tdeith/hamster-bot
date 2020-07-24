use core::RefCell;
use super::peripheral::STM_PERIPHERAL;

pub static POWER: Mutex<RefCell<ctxPeripherals>> =
    Mutex::new(RefCell::new(STM_PERIPHERAL.borrow().RCC));
