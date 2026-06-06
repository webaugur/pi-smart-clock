//! RP2040 bit-banged DVI (Pico DVI Sock pinout).
#![no_std]

extern crate alloc;

pub mod clock;
pub mod dvi;
pub mod link;
pub mod render;
pub mod scanlist;

use core::arch::global_asm;
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

use cortex_m::peripheral::NVIC;
use dvi::dma::DmaChannelList;
use rp2040_hal::{
    dma::{Channel, DMAExt, CH0, CH1, CH2, CH3, CH4, CH5},
    gpio::{FunctionSioOutput, Pin, PinId, PullDown},
    multicore::{Multicore, Stack},
    pac::{self, interrupt, Interrupt},
    pwm::{self, ValidPwmOutputPin},
    sio::SioFifo,
    watchdog::Watchdog,
};

use crate::{
    clock::init_clocks,
    dvi::{
        core1_main,
        dma::DmaChannels,
        serializer::{DviClockPins, DviDataPins, DviSerializer},
        timing::VGA_TIMING,
        DviInst,
    },
    render::{init_4bpp_palette, init_display_swapcell, render_line, GLOBAL_PALETTE},
};

global_asm! {
    include_str!("pre_init.asm"),
    options(raw)
}

pub const DISPLAY_WIDTH: u32 = 640;
pub const DISPLAY_HEIGHT: u32 = 480;

struct DviChannels;
impl DmaChannelList for DviChannels {
    type Ch0 = Channel<CH0>;
    type Ch1 = Channel<CH1>;
    type Ch2 = Channel<CH2>;
    type Ch3 = Channel<CH3>;
    type Ch4 = Channel<CH4>;
    type Ch5 = Channel<CH5>;
}

struct DviInstWrapper(UnsafeCell<MaybeUninit<DviInst<DviChannels>>>);
unsafe impl Sync for DviInstWrapper {}

static DVI_INST: DviInstWrapper = DviInstWrapper(UnsafeCell::new(MaybeUninit::uninit()));
static mut CORE1_STACK: Stack<2048> = Stack::new();
static mut FIFO: MaybeUninit<SioFifo> = MaybeUninit::uninit();

const PALETTE: &[u32] = &[
    0x000000, 0xffffff, 0x9d9d9d, 0xe06f8b, 0xbe2633, 0x493c2b, 0xa46422, 0xeb8931, 0xf7e26b,
    0xa3ce27, 0x44891a, 0x2f484e, 0x1b2632, 0x005784, 0x31a2f2, 0xb2dcef,
];

/// Overclock, start DVI scanout on core 1, and prepare the display-list swap cell.
pub fn start<const XOSC_HZ: u32, LED, RedPos, RedNeg, GreenPos, GreenNeg, BluePos, BlueNeg, SliceId, ClockPos, ClockNeg>(
    pins: (
        Pin<LED, FunctionSioOutput, PullDown>,
        DviDataPins<RedPos, RedNeg, GreenPos, GreenNeg, BluePos, BlueNeg>,
        DviClockPins<SliceId, ClockPos, ClockNeg>,
        SioFifo,
    ),
) where
    LED: PinId,
    RedPos: PinId + Send + 'static,
    RedNeg: PinId + Send + 'static,
    GreenPos: PinId + Send + 'static,
    GreenNeg: PinId + Send + 'static,
    BluePos: PinId + Send + 'static,
    BlueNeg: PinId + Send + 'static,
    SliceId: pwm::SliceId + Send + 'static,
    ClockPos: PinId + ValidPwmOutputPin<SliceId, pwm::A> + Send + 'static,
    ClockNeg: PinId + ValidPwmOutputPin<SliceId, pwm::B> + Send + 'static,
{
    start_scanout::<XOSC_HZ, _, _, _, _, _, _, _, _, _, _>(pins, true);
}

/// Start DVI when system clocks are already configured (e.g. by `embassy_rp::init`).
pub fn start_scanout<const XOSC_HZ: u32, LED, RedPos, RedNeg, GreenPos, GreenNeg, BluePos, BlueNeg, SliceId, ClockPos, ClockNeg>(
    pins: (
        Pin<LED, FunctionSioOutput, PullDown>,
        DviDataPins<RedPos, RedNeg, GreenPos, GreenNeg, BluePos, BlueNeg>,
        DviClockPins<SliceId, ClockPos, ClockNeg>,
        SioFifo,
    ),
    configure_clocks: bool,
) where
    LED: PinId,
    RedPos: PinId + Send + 'static,
    RedNeg: PinId + Send + 'static,
    GreenPos: PinId + Send + 'static,
    GreenNeg: PinId + Send + 'static,
    BluePos: PinId + Send + 'static,
    BlueNeg: PinId + Send + 'static,
    SliceId: pwm::SliceId + Send + 'static,
    ClockPos: PinId + ValidPwmOutputPin<SliceId, pwm::A> + Send + 'static,
    ClockNeg: PinId + ValidPwmOutputPin<SliceId, pwm::B> + Send + 'static,
{
    let timing = VGA_TIMING;
    let mut peripherals = unsafe { pac::Peripherals::steal() };

    if configure_clocks {
        let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
        let _clocks = init_clocks::<XOSC_HZ>(
            peripherals.XOSC,
            peripherals.ROSC,
            peripherals.CLOCKS,
            peripherals.PLL_SYS,
            peripherals.PLL_USB,
            &mut peripherals.RESETS,
            &mut watchdog,
            timing.bit_clk,
        );
    }

    let (led_pin, data_pins, clock_pins, mut sio_fifo) = pins;

    let serializer = DviSerializer::new(
        peripherals.PIO0,
        &mut peripherals.RESETS,
        data_pins,
        clock_pins,
    );

    let dma = peripherals.DMA.split(&mut peripherals.RESETS);
    let dma_channels = DmaChannels::new(
        (dma.ch0, dma.ch1, dma.ch2, dma.ch3, dma.ch4, dma.ch5),
        serializer.tx(),
    );

    {
        let inst = unsafe { (*DVI_INST.0.get()).write(DviInst::new(timing, dma_channels)) };
        inst.setup_dma();
        inst.start();
    }

    let mut mc = Multicore::new(&mut peripherals.PSM, &mut peripherals.PPB, &mut sio_fifo);
    let cores = mc.cores();
    cores[1]
        .spawn(
            unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK.mem) },
            move || core1_main(serializer),
        )
        .unwrap();

    unsafe {
        FIFO = MaybeUninit::new(sio_fifo);
        NVIC::unmask(Interrupt::SIO_IRQ_PROC0);
    }

    init_display_swapcell(DISPLAY_WIDTH);
    init_4bpp_palette(
        unsafe { &mut *core::ptr::addr_of_mut!(GLOBAL_PALETTE) },
        PALETTE,
    );

    let _ = led_pin;
}

#[interrupt]
fn SIO_IRQ_PROC0() {
    let fifo = unsafe { (*core::ptr::addr_of_mut!(FIFO)).assume_init_mut() };
    while let Some(line_ix) = fifo.read() {
        unsafe { render_line(line_ix) };
    }
}