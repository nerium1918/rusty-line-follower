use arduino_hal::{port::{Pin, mode::Output}, hal::port::{PB4, PD7, PD4, PB0}};
use panic_halt as _;

pub(crate) enum Instruction {
    Forward,
    // Backward,
    Release
}

pub(crate) enum Motor {
    Left,
    Right
}

pub(crate) struct MotorManager {
    motor_latch: Pin<Output, PB4>,
    motor_enable: Pin<Output, PD7>,
    motor_data: Pin<Output, PB0>,
    motor_clk: Pin<Output, PD4>,
    latch_state: i8,
}

impl MotorManager {
    pub(crate) fn new(
        motor_latch: Pin<Output, PB4>,
        motor_data: Pin<Output, PB0>,
        motor_enable: Pin<Output, PD7>,
        motor_clk: Pin<Output, PD4>
    ) -> Self {    
        return MotorManager {
            motor_data,
            motor_enable,
            motor_latch,
            motor_clk,
            latch_state: 0
        }
    }

    pub(crate) fn reset(&mut self) -> () {
        self.motor_latch.set_low();
        self.motor_data.set_low();

        for n in 0..=7 {
            self.motor_clk.set_low();
            if self.latch_state & 1 << (7 - n) > 0 {
                self.motor_data.set_high();
            } else {
                self.motor_data.set_low();
            }

            self.motor_clk.set_high();
        }

        self.motor_latch.set_high();
        self.motor_enable.set_low();
    }

    pub(crate) fn run(&mut self, instruction: Instruction, motor: Motor) -> () {
        let a: i8;
        let b: i8;

        match motor {
            Motor::Right => {
                a = 2;
                b = 3;
            },
            Motor::Left => {
                a = 1;
                b = 4;
            }
        };

        match instruction {
            Instruction::Forward => {
                self.latch_state |= 1 << a;
                self.latch_state &= !(1 << b);
                self.reset();
            },
            // Instruction::Backward =>{
            //     self.latch_state &= !(1 << a);
            //     self.latch_state |= 1 << b;
            //     self.reset();
            // },
            Instruction::Release => {
                self.latch_state &= !(1 << a);
                self.latch_state &= !(1 << b);
                self.reset();
            },
        }
    }
}
