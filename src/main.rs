use pi_smart_clock::drivers::platform::Platform;
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
        "[display] {}x{} -> {:?} logical {}x{} (4:3 vertical), window {}x{}",
        display_w,
        display_h,
        layout.orientation,
        layout.screen_w,
        layout.screen_h,
        win_w,
        win_h
    );

    // Font loading for all UI text (bottom panels, menus, status, alarms, etc.).
    // DejaVu is the baseline (excellent Latin/Greek/Cyrillic coverage).
    // For Japanese, Chinese, Korean, and other scripts we try common CJK-capable
    // system fonts first (Noto Sans CJK, IPA fonts, etc.). These are optional but
    // strongly recommended if you use non-Latin holiday regions (e.g. country=JP).
    //
    // Install examples:
    //   Debian Trixie: sudo apt install fonts-noto-cjk fonts-ipafont-gothic fonts-dejavu-core
    //   OpenIndiana 2025 (pkgsrc): pkgin install noto-ttf ipafont dejavu-ttf (or equivalent); adjust paths below if under /opt/local or /usr/pkg
    let font_candidates: [&str; 10] = [
        // CJK-capable (preferred)
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/opentype/ipafont-gothic/ipag.ttf",
        "/usr/share/fonts/truetype/ipafont-gothic/ipag.ttf",
        "/usr/share/fonts/truetype/fonts-japanese-gothic.ttf",
        // OI / pkgsrc common locations
        "/opt/local/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/pkg/share/fonts/TTF/DejaVuSans.ttf",
        // Baseline DejaVu
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        // Bundled fallback
        "assets/fonts/DejaVuSans.ttf",
    ];

    let mut loaded_font = None;
    for path in font_candidates {
        if let Ok(f) = ttf.load_font(path, layout.font_size) {
            eprintln!("[font] loaded UI font: {}", path);
            loaded_font = Some(f);
            break;
        }
    }

    let font = loaded_font.ok_or_else(|| {
        "No UI font found. On Debian Trixie: apt install fonts-dejavu-core (or fonts-noto-cjk). On OpenIndiana 2025: use pkgsrc for dejavu-ttf / noto-ttf. See docs/LINUX.md.".to_string()
    })?;
    let font: &'static sdl2::ttf::Font<'static, 'static> = Box::leak(Box::new(font));
    let _ttf = ttf;

    let mut window = video
        .window("Smart Clock", win_w, win_h)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    let (min_w, min_h) = Layout::minimum_window_size();
    window
        .set_minimum_size(min_w, min_h)
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

    platform.init().await?;

    let mut state = SmartClockState::new();
    state.init(&mut platform).await?;

    let mut pump = sdl.event_pump()?;
    loop {
        for event in pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(()),
                Event::Window { win_event, .. } => {
                    if let sdl2::event::WindowEvent::Resized(_, _) = win_event {
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
        // Boot tick already sleeps per frame; an extra 16 ms here doubled splash pacing.
        if state.boot_done {
            tokio::time::sleep(std::time::Duration::from_millis(16)).await;
        }
    }
}