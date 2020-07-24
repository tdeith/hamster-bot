
extern crate panic_itm; // panic handler

pub use cortex_m::asm::{bkpt, nop};
pub use cortex_m_rt::entry;
pub use f3::{
    hal::stm32f30x::{rcc, tim6},
    led::Leds,
};

use f3::hal::{
    prelude::*,
    stm32f30x::{self, RCC, TIM6},
};

enum DriverState {
    Offline,
    Ready,
}

struct ClockDriver {
    state: DriverState,
}

static CLOCK: Mutex<Cell<LedDriver>> = Mutex::new(Cell::new(
    DriverState {
        state: DriverState::Offline,
    }
));


#[interrupt]
pub fn init() -> () {
    interrupt::free(|cs| {

        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = stm32f30x::Peripherals::take().unwrap();

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
        let mut nss = gpioe
            .pe3
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        nss.set_high();

        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
        let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
        let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

        let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

        let lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

        let delay = Delay::new(cp.SYST, clocks);
        let mono_timer = MonoTimer::new(cp.DWT, clocks);

        (lsm303dlhc, delay, mono_timer, cp.ITM)
    });
}


let mut rcc = dp.RCC.constrain();
let clocks = rcc.cfgr.freeze(&mut flash.acr);
let delay = Delay::new(cp.SYST, clocks);
let mono_timer = MonoTimer::new(cp.DWT, clocks);

let dp = stm32f30x::Peripherals::take().unwrap();

let mut flash = board_periph.FLASH.constrain();
let mut rcc = sys_periph.RCC.constrain();


let clocks = rcc.cfgr.freeze(&mut flash.acr);