#![no_std]
#![no_main]

use arduino_hal::{port::{Pin, mode::Output}, hal::{port::{PB4, PD7, PD4, PB0}}, Pins};
use panic_halt as _;

enum Instruction {
    Forward,
    Backward,
    Release
}

enum Motor {
    One,
    Two
}

struct MotorManager {
    motor_latch: Pin<Output, PB4>,
    motor_enable: Pin<Output, PD7>,
    motor_data: Pin<Output, PB0>,
    motor_clk: Pin<Output, PD4>,
    latch_state: i8,
}

impl MotorManager {
    fn new(
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

    fn reset(&mut self) -> () {
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

    fn run(&mut self, instruction: Instruction, motor: i8) -> () {
        let mut a: i8 = 0;
        let mut b: i8 = 0;

        match motor {
            1 => {
                a = 2;
                b = 3;
            },
            2 => {
                a = 1;
                b = 4;
            },
            3 => {
                a = 5;
                b = 7;
            },
            4 => {
                a = 0;
                b = 6;
            },
            _ => {}
        };

        match instruction {
            Instruction::Forward => {
                self.latch_state |= 1 << a;
                self.latch_state &= !(1 << b);
                self.reset();
            },
            Instruction::Backward =>{
                self.latch_state &= !(1 << a);
                self.latch_state |= 1 << b;
                self.reset();
            },
            Instruction::Release => {
                self.latch_state &= !(1 << a);
                self.latch_state &= !(1 << b);
                self.reset();
            },
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let motor_latch = pins.d12.into_output();
    let motor_data = pins.d8.into_output();
    let motor_enable = pins.d7.into_output();
    let motor_clk = pins.d4.into_output();

    let mut motor_manager = MotorManager::new(motor_latch, motor_data, motor_enable, motor_clk);
    motor_manager.reset();
    
    let pin_ir_a1 = pins.a0.into_analog_input(&mut adc);
    let pin_ir_a2 = pins.a1.into_analog_input(&mut adc);
    let pin_ir_a3 = pins.a2.into_analog_input(&mut adc);
    let pin_ir_a4 = pins.a3.into_analog_input(&mut adc);
    let pin_ir_a5 = pins.a4.into_analog_input(&mut adc);

    let mut ir_value_a1 = 0;
    let mut ir_value_a2 = 0;
    let mut ir_value_a3 = 0;
    let mut ir_value_a4 = 0;
    let mut ir_value_a5 = 0;

    loop {
        ufmt::uwriteln!(&mut serial, "Analog Reading = [").unwrap();
        ufmt::uwriteln!(&mut serial, "{}\t", ir_value_a1).unwrap();
        ufmt::uwriteln!(&mut serial, "{}\t", ir_value_a2).unwrap();
        ufmt::uwriteln!(&mut serial, "{}\t", ir_value_a3).unwrap();
        ufmt::uwriteln!(&mut serial, "{}\t", ir_value_a4).unwrap();
        ufmt::uwriteln!(&mut serial, "{}\t", ir_value_a5).unwrap();
        ufmt::uwriteln!(&mut serial, "]").unwrap();

        if ir_value_a2 > 500 && ir_value_a4 > 500 {
            motor_manager.run(Instruction::Forward, 2);
            motor_manager.run(Instruction::Forward, 1);
        } else if ir_value_a2 < 500 && ir_value_a4 > 500 {
            motor_manager.run(Instruction::Release, 2);
            motor_manager.run(Instruction::Forward, 1);
        } else if ir_value_a2 > 500 && ir_value_a4 < 500 {
            motor_manager.run(Instruction::Release, 1);
            motor_manager.run(Instruction::Forward, 2);
        } else if ir_value_a1 < 500 && ir_value_a5 > 500 {
            motor_manager.run(Instruction::Release, 1);
            motor_manager.run(Instruction::Forward, 2);
        } else if ir_value_a1 > 500 && ir_value_a5 < 500 {
            motor_manager.run(Instruction::Release, 2);
            motor_manager.run(Instruction::Forward, 1);
        } else {
            motor_manager.run(Instruction::Release, 2);
            motor_manager.run(Instruction::Release, 1); 
        }

        ir_value_a1 = pin_ir_a1.analog_read(&mut adc);
        ir_value_a2 = pin_ir_a2.analog_read(&mut adc);
        ir_value_a3 = pin_ir_a3.analog_read(&mut adc);
        ir_value_a4 = pin_ir_a4.analog_read(&mut adc);
        ir_value_a5 = pin_ir_a5.analog_read(&mut adc);
    }
}
