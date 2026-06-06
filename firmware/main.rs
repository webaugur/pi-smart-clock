#![no_std]
#![no_main]

mod alloc;

use ::core::future::Future;
use ::core::pin::pin;
use ::core::task::{Context, Poll};
use ::core::panic::PanicInfo;

use pico_dvi_rs::dvi::serializer::{DviClockPins, DviDataPins};
use pi_smart_clock::drivers::platform::Platform;
use pi_smart_clock::layout::Layout;
use pi_smart_clock::platform::rp2040::PicoDviPlatform;
use pi_smart_clock::runtime::SmartClockState;
use rp2040_hal::gpio::PinState;
use rp2040_hal::{self, pwm, sio::Sio};

const XOSC_HZ: u32 = 12_000_000;

fn init_heap() {
    static mut HEAP_MEM: [u8; alloc::HEAP_SIZE] = [0; alloc::HEAP_SIZE];
    unsafe {
        alloc::init_heap(&mut *::core::ptr::addr_of_mut!(HEAP_MEM));
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(core::task::Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return output;
        }
    }
}

fn start_dvi_sock() {
    let mut pac = unsafe { rp2040_hal::pac::Peripherals::steal() };
    let sio_hal = Sio::new(pac.SIO);
    let pwm_slices = pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio_hal.gpio_bank0,
        &mut pac.RESETS,
    );

    let led = pins.led.into_push_pull_output_in_state(PinState::Low);

    pico_dvi_rs::start::<XOSC_HZ, _, _, _, _, _, _, _, _, _, _>((
        led,
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
        sio_hal.fifo,
    ));
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cortex_m_rt::entry]
fn main() -> ! {
    init_heap();
    let _layout = Layout::init(
        pico_dvi_rs::DISPLAY_WIDTH,
        pico_dvi_rs::DISPLAY_HEIGHT,
    );

    start_dvi_sock();

    let mut platform = PicoDviPlatform::new();
    block_on(async {
        platform.init().await.ok();
        let mut state = SmartClockState::new();
        state.init(&mut platform).await.ok();
        loop {
            state.tick(&mut platform).await;
            platform.present().await;
            platform.delay(16).await;
        }
    })
}