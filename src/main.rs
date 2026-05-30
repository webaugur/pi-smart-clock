use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

mod panel;
mod clock;
mod modules;

use panel::Panel;
use modules::calendar::CalendarPanel;
use modules::holidays::HolidaysPanel;
use modules::weather::WeatherPanel;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem.window("Smart Clock", 800, 480)
        .position_centered()
        .fullscreen()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut font = ttf_context.load_font("DejaVuSans.ttf", 28)?;

    let mut clock = clock::Clock::new(&mut font, &texture_creator)?;
    let mut calendar = CalendarPanel::new();
    let mut holidays = HolidaysPanel::new();
    let mut weather = WeatherPanel::new();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit {..} = event {
                break 'running;
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        clock.draw(&mut canvas)?;

        let bottom_y = 320;
        let panel_h = 160;
        let panel_w = 267;

        calendar.draw(&mut canvas, 0, bottom_y, panel_w, panel_h);
        holidays.draw(&mut canvas, panel_w, bottom_y, panel_w, panel_h);
        weather.draw(&mut canvas, panel_w * 2, bottom_y, panel_w, panel_h);

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}