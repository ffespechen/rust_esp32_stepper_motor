// Control de un motor paso a paso 28BYJ-48 con driver ULN2003

use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    esp_idf_hal::sys::link_patches();

    let pd = Peripherals::take().unwrap();
    let delay: Delay = Default::default();

    // Constantes de acuerdo a las características del motor
    const PASOS_POR_VUELTA: i32 = 4096;
    const ESPERA_MOTOR: u32 = 1200;

    let mut in1 = PinDriver::output(pd.pins.gpio19).unwrap();
    let mut in2 = PinDriver::output(pd.pins.gpio18).unwrap();
    let mut in3 = PinDriver::output(pd.pins.gpio5).unwrap();
    let mut in4 = PinDriver::output(pd.pins.gpio17).unwrap();

    // Secuencia para medios pasos
    let pasos: [u8; 8] = [
        0b00001000,
        0b00001100,
        0b00000100,
        0b00000110,
        0b00000010,
        0b00000011,
        0b00000001,
        0b00001001
    ];

    loop {

        // Giro horario del motor
        for _i in 1..(PASOS_POR_VUELTA * 2) {
            for paso in pasos {
                println!("{}", paso);
                if paso & 1 == 1 { in1.set_high().unwrap() } else { in1.set_low().unwrap() };
                if paso & 2 == 2 { in2.set_high().unwrap() } else { in2.set_low().unwrap() };
                if paso & 4 == 4 { in3.set_high().unwrap() } else { in3.set_low().unwrap() };
                if paso & 8 == 8 { in4.set_high().unwrap() } else { in4.set_low().unwrap() };
                delay.delay_us(ESPERA_MOTOR);
            }
        }

    }
}
