#![no_std]
#![no_main]

use rp_pico::{
    hal::{gpio::PinState, pwm, sio::Sio},
    Pins, XOSC_CRYSTAL_FREQ,
};

use pico_dvi_rs::dvi::serializer::{DviClockPins, DviDataPins};
use pico_dvi_rs::{render::FONT_HEIGHT, start, DISPLAY_HEIGHT};

mod demo;

#[rp_pico::entry]
fn main() -> ! {
    let mut peripherals = rp_pico::hal::pac::Peripherals::take().unwrap();
    let sio = Sio::new(peripherals.SIO);
    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let led_pin = pins.led.into_push_pull_output_in_state(PinState::Low);
    let pwm_slices = pwm::Slices::new(peripherals.PWM, &mut peripherals.RESETS);

    start::<XOSC_CRYSTAL_FREQ, _, _, _, _, _, _, _, _, _, _>((
        led_pin,
        DviDataPins {
            blue_pos: pins.gpio12.into_function(),
            blue_neg: pins.gpio13.into_function(),
            green_pos: pins.gpio18.into_function(),
            green_neg: pins.gpio19.into_function(),
            red_pos: pins.gpio16.into_function(),
            red_neg: pins.gpio17.into_function(),
        },
        DviClockPins {
            clock_pos: pins.gpio14.into_function(),
            clock_neg: pins.gpio15.into_function(),
            pwm_slice: pwm_slices.pwm7,
        },
        sio.fifo,
    ));

    demo::run(DISPLAY_HEIGHT - FONT_HEIGHT);
}