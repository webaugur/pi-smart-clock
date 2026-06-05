use pi_smart_clock::platform::linux::{SdlPlatform, SdlPlatformExt};
use pi_smart_clock::runtime::SmartClockState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[tokio::main]
async fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let ttf: &'static sdl2::ttf::Sdl2TtfContext =
        Box::leak(Box::new(sdl2::ttf::init().map_err(|e| e.to_string())?));
    let _audio = sdl.audio()?;
    let _mixer = sdl2::mixer::init(sdl2::mixer::InitFlag::empty())?;
    sdl2::mixer::open_audio(44100, sdl2::mixer::AUDIO_S16LSB, 2, 2048)?;

    let font = ttf
        .load_font(
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            20,
        )
        .or_else(|_| ttf.load_font("/usr/share/fonts/TTF/DejaVuSans.ttf", 20))
        .map_err(|e| format!("need DejaVu font: {e}"))?;
    let font: &'static sdl2::ttf::Font<'static, 'static> = Box::leak(Box::new(font));
    let _ttf = ttf;

    let window = video
        .window("Smart Clock", 800, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut platform = SdlPlatform::new(canvas);
    platform.set_font(&font);

    let mut state = SmartClockState::new();
    state.init(&mut platform).await?;

    let mut pump = sdl.event_pump()?;
    loop {
        for event in pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(()),
                Event::KeyDown {
                    keycode: Some(k), ..
                } => {
                    if k == Keycode::Escape {
                        return Ok(());
                    }
                    if k == Keycode::M {
                        state.ui_mode = pi_smart_clock::runtime::UiMode::Menu;
                    }
                    platform.ingest_key(k, true);
                }
                _ => {}
            }
        }

        state.tick(&mut platform).await;
        state.render_linux(&mut platform).await;
        tokio::time::sleep(std::time::Duration::from_millis(16)).await;
    }
}
