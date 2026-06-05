#![no_std]
#![no_main]

use embassy_executor::Spawner;
use pi_smart_clock::platform::rp2040::PicoDviPlatform;
use pi_smart_clock::runtime::SmartClockState;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut platform = PicoDviPlatform::new();
    platform.init().await.ok();
    let mut state = SmartClockState::new();
    state.init(&mut platform).await.ok();
    loop {
        state.tick(&mut platform).await;
        platform.delay(16).await;
    }
}
