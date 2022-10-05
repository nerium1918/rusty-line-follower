#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    
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

        arduino_hal::delay_ms(1000);

        ir_value_a1 = pin_ir_a1.analog_read(&mut adc);
        ir_value_a2 = pin_ir_a2.analog_read(&mut adc);
        ir_value_a3 = pin_ir_a3.analog_read(&mut adc);
        ir_value_a4 = pin_ir_a4.analog_read(&mut adc);
        ir_value_a5 = pin_ir_a5.analog_read(&mut adc);
    }
}
