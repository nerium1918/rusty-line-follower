use arduino_hal::{port::{Pin, mode::{Output, Input}}, hal::{port::{PD0, PD1}}, Usart, pac::USART0};
use panic_halt as _;
use crate::motor_manager::MotorManager;
use crate::sensors::Sensors;

pub(crate) struct BoardManager {
    pub(crate) motor_manager: MotorManager,
    pub(crate) sensors: Sensors,
    pub(crate) serial: Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>
}

impl BoardManager {
    pub(crate) fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
        let serial = arduino_hal::default_serial!(dp, pins, 57600);
    
        // Motors setup
        let d_pin12 = pins.d12.into_output();
        let d_pin8 = pins.d8.into_output();
        let d_pin7 = pins.d7.into_output();
        let d_pin4 = pins.d4.into_output();

        let mut motor_manager = MotorManager::new(d_pin12, d_pin8, d_pin7, d_pin4);
        motor_manager.reset();

        // Sensors setup
        let a_pin1 = pins.a0.into_analog_input(&mut adc);
        let a_pin2 = pins.a1.into_analog_input(&mut adc);
        let a_pin3 = pins.a2.into_analog_input(&mut adc);
        let a_pin4 = pins.a3.into_analog_input(&mut adc);
        let a_pin5 = pins.a4.into_analog_input(&mut adc);

        let sensors = Sensors::new(adc, a_pin1, a_pin2, a_pin3, a_pin4, a_pin5);

        return BoardManager{
            motor_manager,
            sensors,
            serial
        }
    }
}
