#![no_std]
#![no_main]

use motor_manager::{Instruction, Motor};
use panic_halt as _;
use board_manager::BoardManager;
use sensors::Sensors;

mod board_manager;
mod motor_manager;
mod sensors;

#[arduino_hal::entry]
fn main() -> ! {
    let board_manager = BoardManager::new();
    let BoardManager { mut motor_manager, mut sensors, mut serial } = board_manager;

    loop {
        sensors.read_values(&mut serial);
        let Sensors { ir1_value, ir2_value, ir3_value: _, ir4_value, ir5_value, .. } = sensors;

        if ir2_value < 500 && ir4_value > 500 {
            motor_manager.run(Instruction::Forward, Motor::Right);
            motor_manager.run(Instruction::Release, Motor::Left);
        } else if ir2_value > 500 && ir4_value < 500 {
            motor_manager.run(Instruction::Forward, Motor::Left);
            motor_manager.run(Instruction::Release, Motor::Right);
        } else if ir1_value < 500 && ir5_value > 500 {
            motor_manager.run(Instruction::Release, Motor::Left);
            motor_manager.run(Instruction::Forward, Motor::Right);
        } else if ir1_value > 500 && ir5_value < 500 {
            motor_manager.run(Instruction::Release, Motor::Right);
            motor_manager.run(Instruction::Forward, Motor::Left);
        } else if ir2_value > 500 && ir4_value > 500 {
            motor_manager.run(Instruction::Forward, Motor::Right);
            motor_manager.run(Instruction::Forward, Motor::Left);
        } else {
            motor_manager.run(Instruction::Release, Motor::Right);
            motor_manager.run(Instruction::Release, Motor::Left);
        }
    }
}
