use arduino_hal::{port::{Pin, mode::{Output, Analog, Input}}, hal::port::{PC0, PC1, PC2, PC3, PC4, PD0, PD1}, Adc, Usart, pac::USART0};
use panic_halt as _;

pub(crate) struct Sensors {
    adc: Adc,
    ir1: Pin<Analog, PC0>,
    ir2: Pin<Analog, PC1>,
    ir3: Pin<Analog, PC2>,
    ir4:  Pin<Analog, PC3>,
    ir5: Pin<Analog, PC4>,
    pub(crate) ir1_value: u16,
    pub(crate) ir2_value: u16,
    pub(crate) ir3_value: u16,
    pub(crate) ir4_value: u16,
    pub(crate) ir5_value: u16,
}

impl Sensors {
    pub(crate) fn new(
        adc: Adc,
        ir1: Pin<Analog, PC0>,
        ir2: Pin<Analog, PC1>,
        ir3: Pin<Analog, PC2>,
        ir4: Pin<Analog, PC3>,
        ir5: Pin<Analog, PC4>,
    ) -> Self {    
        return Sensors {
            adc,
            ir1,
            ir2,
            ir3,
            ir4,
            ir5,
            ir1_value: 0,
            ir2_value: 0,
            ir3_value: 0,
            ir4_value: 0,
            ir5_value: 0,
        }
    }

    pub(crate) fn read_values(&mut self, mut serial: &mut Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>) {
        self.ir1_value = self.ir1.analog_read(&mut self.adc);
        self.ir2_value = self.ir2.analog_read(&mut self.adc);
        self.ir3_value = self.ir3.analog_read(&mut self.adc);
        self.ir4_value = self.ir4.analog_read(&mut self.adc);
        self.ir5_value = self.ir5.analog_read(&mut self.adc);

        ufmt::uwriteln!(&mut serial, "Analog Reading = [").unwrap();
        ufmt::uwriteln!(&mut serial, "{}\t", self.ir1_value).unwrap();
        ufmt::uwriteln!(&mut serial, "{}\t", self.ir2_value).unwrap();
        ufmt::uwriteln!(&mut serial, "{}\t", self.ir3_value).unwrap();
        ufmt::uwriteln!(&mut serial, "{}\t", self.ir4_value).unwrap();
        ufmt::uwriteln!(&mut serial, "{}\t", self.ir5_value).unwrap();
        ufmt::uwriteln!(&mut serial, "]").unwrap();
    }
}
