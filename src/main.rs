#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut motor_latch = pins.d12.into_output();
    let mut motor_enable = pins.d7.into_output();
    let mut motor_data = pins.d8.into_output();
    let mut motor_clk = pins.d4.into_output();

    let mut latch_state = 0;

    // reset begin
    // Това ще е добре да го изнесем в reset функция
    motor_latch.set_low();
    motor_data.set_low();

    for n in 0..=7 {
        motor_clk.set_low();

        // TODO: Това сто про не е правилно 🤔
        if latch_state & 1 << (7 - n) > 0 {
            motor_data.set_high();
        } else {
            motor_data.set_low();
        }

        motor_clk.set_high();
    }

    motor_latch.set_high();
    motor_enable.set_low();
    // reset end

    loop {
        // Forward motor 1
        latch_state |= 1 << 2;
        latch_state &= !(1 << 3);

        // reset begin
        // Това задължително трябва да го изнесем в reset функция
        motor_latch.set_low();
        motor_data.set_low();

        for n in 0..=7 {
            motor_clk.set_low();

            // TODO: Това сто про не е правилно 🤔
            if latch_state & 1 << (7 - n) > 0 {
                motor_data.set_high();
            } else {
                motor_data.set_low();
            }

            motor_clk.set_high();
        }

        motor_latch.set_high();
        motor_enable.set_low();
        // reset end

        // Forward motor 2
        latch_state |= 1 << 1;
        latch_state &= !(1 << 4);

        // reset begin
        // Това задължително трябва да го изнесем в reset функция
        motor_latch.set_low();
        motor_data.set_low();

        for n in 0..=7 {
            motor_clk.set_low();

            // TODO: Това сто про не е правилно 🤔
            if latch_state & 1 << (7 - n) > 0 {
                motor_data.set_high();
            } else {
                motor_data.set_low();
            }

            motor_clk.set_high();
        }

        motor_latch.set_high();
        motor_enable.set_low();
        // reset end

        arduino_hal::delay_ms(5000);

        // Backward motor 1
        // Backward motor 2
        // Release motor 1
        // Release motor 2
    }
}
