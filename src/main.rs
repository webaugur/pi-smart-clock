use pi_smart_clock::layout::Layout;
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

    let display = video.display_bounds(0).map_err(|e| e.to_string())?;
    let display_w = display.width();
    let display_h = display.height();
    let layout = Layout::init(display_w, display_h);
    let (win_w, win_h) = Layout::window_size(display_w, display_h);

    eprintln!(
        "[display] {}x{} -> {:?} logical {}x{}, window {}x{}",
        display_w,
        display_h,
        layout.orientation,
        layout.screen_w,
        layout.screen_h,
        win_w,
        win_h
    );

    let font = ttf
        .load_font(
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            layout.font_size,
        )
        .or_else(|_| {
            ttf.load_font(
                "/usr/share/fonts/TTF/DejaVuSans.ttf",
                layout.font_size,
            )
        })
        .map_err(|e| format!("need DejaVu font: {e}"))?;
    let font: &'static sdl2::ttf::Font<'static, 'static> = Box::leak(Box::new(font));
    let _ttf = ttf;

    let window = video
        .window("Smart Clock", win_w, win_h)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut platform = SdlPlatform::new(canvas).map_err(|e| e.to_string())?;
    platform.configure_display()?;
    platform.set_font(&font);

    let mut state = SmartClockState::new();
    state.init(&mut platform).await?;

    let mut pump = sdl.event_pump()?;
    loop {
        for event in pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(()),
                Event::Window { win_event, .. } => {
                    if let sdl2::event::WindowEvent::Resized(w, h) = win_event {
                        let _ = (w, h);
                        platform.configure_display()?;
                    }
                }
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