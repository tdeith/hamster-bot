#![no_std]

extern crate panic_itm; // panic handler

use core::cell::Cell;
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
    lsm303dlhc::{self, Accel, I16x3, Sensitivity},
    led::{Direction, Leds},
};

use crate::core::{
    clocks,
    peripheral::STM_PERIPHERAL,
    GPIO_B::GPIO_B, GPIO_E::GPIO_E,
};

enum ImuDriverState {
    Offline,
    Initializing,
    Ready,
}

struct ImuDriver {
    state: ImuDriverState,
    device: Option<Lsm303dlhc>,
}

static IMU_DEVICE: Mutex<RefCell<ImuDriver>> = Mutex::new(RefCell::new(
    ImuDriver {
        state: ImuDriverState::Offline,
        device: None,
    }
));

#[interrupt]
pub fn init() {
    clocks::init();

    interrupt::free(|cs| {
        let imu = IMU_DEVICE.borrow(cs);
        if imu.as_ptr().state != ImuDriverState::Offline {
            return;
        }

        let mut rcc = STM_PERIPHERAL.borrow(cs).as_ptr().rcc.constrain();
        let mut gpio_e = GPIO_E.borrow(cs).as_ptr().split(&mut rcc.ahb);

        let mut nss_pin = gpio_e.pe3
            .into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
        nss_pin.set_high();

        let mut gpio_b = GPIO_B.borrow(cs).as_ptr().split(&mut rcc.ahb);
        let scl_pin = gpio_b.pb6.into_af4(&mut gpio_b.moder, &mut gpio_b.afrl);
        let sda_pin = gpio_b.pb7.into_af4(&mut gpio_b.moder, &mut gpio_b.afrl);

        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        let i2c = I2c::i2c1(dp.I2C1, (scl_pin, sda_pin), 400.khz(), clocks, &mut rcc.apb1);

        let lsm = lsm303dlhc::new(i2c).unwrap();
        lsm.set_accel_sensitivity(Sensitivity::G12).unwrap();
        imu.replace(ImuDriver {
            state: ImuDriverState::Ready,
            device: Some(lsm),
        });
    });
}

#[interrupt]
pub fn read_accel(cs: &CriticalSection) -> Result<Accel, String> {
    let imu = IMU_DEVICE.borrow(cs).as_ptr()
        .device
        .ok_or("Must initialize IMU before use!");
    imu.accel().map_err(|er| format!("Could not read IMU: {}", er))
}
