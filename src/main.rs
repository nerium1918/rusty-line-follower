#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::simple_pwm::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Black line follower
    let ir1 = pins.a1;
    let ir2 = pins.a2;

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    // Motor one
    let mut en_a = pins.d5.into_output().into_pwm(&timer0);
    let mut motor_aip1 = pins.d2.into_output();
    let mut motor_aip2  = pins.d3.into_output();

    // Motor two
    let mut en_b = pins.d6.into_output().into_pwm(&timer0);
    let mut motor_bip1 = pins.d1.into_output();
    let mut motor_bip2 = pins.d4.into_output();

    loop {
        if ir1.is_high() && ir2.is_high()  {
            // IR will not glow on black line
            motor_aip1.set_low();
            motor_aip2.set_low();
            motor_bip1.set_low();
            motor_aip2.set_low();
            en_a.set_duty(0);
            en_b.set_duty(0);
        } else if ir1.is_low() && ir2.is_low() {
            // IR not on black line
            motor_aip1.set_high();
            motor_aip2.set_low();
            motor_bip1.set_high();
            motor_bip2.set_low();
            en_a.set_duty(200);
            en_b.set_duty(200);
            arduino_hal::delay_ms(100);
        } else if ir1.is_low() && ir2.is_high() {
            // IR not on black line
            motor_aip1.set_high();
            motor_aip2.set_low();
            motor_bip1.set_low();
            motor_bip2.set_high();
            en_a.set_duty(200);
            en_b.set_duty(150);
            arduino_hal::delay_ms(100);
        } else if ir1.is_high() && ir2.is_low() {
            //Tilt robot towards left by stopping the left wheel and moving the right one
            motor_aip1.set_low();
            motor_aip2.set_high();
            motor_bip1.set_high();
            motor_aip2.set_low();
            en_a.set_duty(150);
            en_b.set_duty(200);
            arduino_hal::delay_ms(100);
        } else {
            motor_aip1.set_low();
            motor_aip2.set_low();
            motor_bip1.set_low();
            motor_bip2.set_low();
            en_a.set_duty(0);
            en_b.set_duty(0);
        }
    }
}
