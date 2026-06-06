#![no_std]
#![no_main]

mod alloc;

use ::core::panic::PanicInfo;

use embassy_executor::Spawner;
use pi_smart_clock::drivers::platform::Platform;
use pi_smart_clock::layout::Layout;
use pi_smart_clock::platform::rp2040::PicoDviPlatform;
use pi_smart_clock::runtime::SmartClockState;

fn init_heap() {
    static mut HEAP_MEM: [u8; alloc::HEAP_SIZE] = [0; alloc::HEAP_SIZE];
    unsafe {
        alloc::init_heap(&mut *::core::ptr::addr_of_mut!(HEAP_MEM));
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    init_heap();
    let _layout = Layout::init(800, 480);

    let mut platform = PicoDviPlatform::new();
    platform.init().await.ok();
    let mut state = SmartClockState::new();
    state.init(&mut platform).await.ok();
    loop {
        state.tick(&mut platform).await;
        platform.delay(16).await;
    }
}