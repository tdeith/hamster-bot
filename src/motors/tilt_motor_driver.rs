#![no_std]

extern crate panic_itm; // panic handler

use core::cell::Cell;
use cortex_m::{
    iprint, iprintln,
    asm::bkpt,
    interrupt::{self, CriticalSection, Mutex},
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
    lsm303dlhc::{self, I16x3},
    led::{Direction, Leds},
};

use crate::{
    core::{
        clocks,
        power,
        GPIO_A,
    },
    imu::imu_driver,
};

pub struct TiltMotorDriver {
    state: TiltMotorDriverStatus,
    position: Option<i8>,
    target: Option<i8>,
}

enum TiltMotorDriverStatus {
    Offline,
    Initializing,
    Zeroing,
    Ready,
}

enum TiltDirection {
    Pos,
    Neg,
}

// TODO: What motor/controller am I using here?
const MS_PER_ENC_TICK: u8 = 40;
const DEG_PER_ENC_TICK: f32 = 3.6;
const POS_MIN_STOP: i8 = -22;
const POS_MAX_STOP: i8 = 22;

static MOTOR_DEVICE: Mutex<Cell<LedDriver>> = Mutex::new(Cell::new(
    TiltMotorDriver {
        state: TiltMotorDriverStatus::Offline,
        position: None,
        target: None,
    }
));

#[interrupt]
pub fn init() {
    clocks::init();
    imu_driver::init();

    interrupt::free(|cs| {
        GPIOA::begin_init();
        GPIOA::write_config(|_, pin| {
            // TODO: What motor/controller am I using here?
        });
        GPIOA::close_init();
        MOTOR_DEVICE.borrow(cs).replace(MotorDriver {
            state: MotorDriverState::Ready,
        });
    });
}

#[interrupt]
pub fn kill() {
    interrupt::free(|cs| {
        GPIOA::write_pins(|gpioa| {
            // TODO: What motor/controller am I using here?
        });
        let power = power::POWER.borrow(cs).as_ptr();
        power.ahbanr.modify(|power| {
            // TODO: What motor/controller am I using here?
        });
    });
}

#[interrupt]
pub fn zero() {
    let deg_per_rad = 180.0 / 3.1416;
    loop {
        interrupt::free(|cs| {
            let motor = MOTOR_DEVICE.borrow(cs).as_ptr();
            let curr_accel = imu_driver::get_accel(cs);
            let curr_angle = ATAN2(curr_accel.x, curr_accel.y);
            if (curr_angle * deg_per_rad) > (DEG_PER_ENC_TICK / 2_f32) {
                move_direction(TiltDirection::Neg, cs);
            }
            else if (curr_angle * deg_per_rad) < (-DEG_PER_ENC_TICK / 2_f32) {
                move_direction(TiltDirection::Pos, cs);
            }
            else {
                motor.replace()
            }
        })
    }
}

#[interrupt]
pub fn set_position(position: f32) {
    let enc_position_target = ((position / DEG_PER_ENC_TICK) as i8)
        .max(POS_MIN_STOP)
        .min(POS_MAX_STOP);

    interrupt::free(|cs| {
        let motor_mtx = MOTOR_DEVICE.borrow(cs);
        let current_pos = motor_mtx.as_ptr().position;
        motor_mtx.replace(
            TiltMotorDriver {
                state: TiltMotorDriverStatus::Ready,
                position: current_pos,
                target: enc_position_target,
            }
        );
    });

    let mut target_met = false;

    loop {

        if target_met {
            break;
        }

        interrupt::free(|cs| {
            let motor = MOTOR_DEVICE.borrow(cs);

            if motor.target == motor.position {
                target_met = true;
                return;
            }
            else {
                let mut direction: TiltDirection;
                let mut next_position: i8;
                if motor.target > motor.position {
                    direction = TiltDirection::Neg;
                    next_position = motor.as_ptr().position - 1;
                } else {
                    direction = TiltDirection::Pos;
                    next_position = motor.as_ptr().position + 1;
                };
                move_direction(direction, cs);
            }

            motor_mtx.replace(
                TiltMotorDriver {
                    state: TiltMotorDriverStatus::Ready,
                    position: next_position,
                    target: motor.as_ptr().target,
                }
            );
        });

        clocks::delay(MS_PER_ENC_TICK);
    }
}

fn move_direction(dir: TiltDirection, cs: &CriticalSection) {
    // TODO: What motor/controller am I using here?
}

#[interrupt]
pub fn reset() {
    set_position(0_f32);
}