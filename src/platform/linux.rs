use chrono::{DateTime, Local};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use std::io::Read;

use crate::drivers::esp8266::{load_esp8266_config, Esp8266Client};
use crate::storage::linux as xdg_storage;
use crate::drivers::platform::Platform;
use crate::layout::{l, Layout};
use crate::platform::linux_audio::{LinuxAudioEngine, resolve_media_path};

pub trait SdlPlatformExt {
    fn canvas_mut(&mut self) -> &mut Canvas<Window>;
    fn ingest_key(&mut self, key: Keycode, pressed: bool);
    fn set_font(&mut self, font: &'static Font<'static, 'static>);
    fn font(&self) -> Option<&'static Font<'static, 'static>>;
    fn audio_mut(&mut self) -> &mut LinuxAudioEngine;
    fn esp8266(&self) -> Option<&Esp8266Client>;
}

pub struct SdlPlatform {
    canvas: Canvas<Window>,
    font: Option<&'static Font<'static, 'static>>,
    font_pt: u16,
    audio: LinuxAudioEngine,
    esp8266: Option<Esp8266Client>,
    rotary_delta: i32,
    button_down: bool,
}

impl SdlPlatform {
    pub fn new(canvas: Canvas<Window>) -> Result<Self, String> {
        Ok(Self {
            canvas,
            font: None,
            font_pt: l().font_size,
            audio: LinuxAudioEngine::new()?,
            esp8266: None,
            rotary_delta: 0,
            button_down: false,
        })
    }

    /// Keep the window at layout aspect ratio and map logical coords uniformly.
    pub fn configure_display(&mut self) -> Result<(), String> {
        let layout = l();
        let (out_w, out_h) = self.canvas.output_size()?;
        let (win_w, win_h) = Layout::snap_window_size(out_w, out_h);

        if win_w != out_w || win_h != out_h {
            self.canvas
                .window_mut()
                .set_size(win_w, win_h)
                .map_err(|e| e.to_string())?;
            return Ok(());
        }

        self.canvas.set_viewport(None::<Rect>);
        self.canvas.set_scale(1.0, 1.0)?;
        self.canvas
            .set_logical_size(layout.screen_w as u32, layout.screen_h as u32)
            .map_err(|e| e.to_string())?;

        eprintln!(
            "[display] output {}x{}, logical {}x{} (4:3 vertical)",
            out_w, out_h, layout.screen_w, layout.screen_h
        );
        Ok(())
    }

    fn rgb(c: u32) -> Color {
        Color::RGB(((c >> 16) & 0xFF) as u8, ((c >> 8) & 0xFF) as u8, (c & 0xFF) as u8)
    }

    fn draw_circle_impl(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        self.canvas.set_draw_color(color);
        for r in (0..=radius).rev() {
            let d = (r * 2) as u32;
            let _ = self.canvas.fill_rect(Rect::new(cx - r, cy - r, d, d));
        }
    }
}

impl SdlPlatformExt for SdlPlatform {
    fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    fn set_font(&mut self, font: &'static Font<'static, 'static>) {
        self.font = Some(font);
        self.font_pt = l().font_size;
    }

    fn font(&self) -> Option<&'static Font<'static, 'static>> {
        self.font
    }

    fn audio_mut(&mut self) -> &mut LinuxAudioEngine {
        &mut self.audio
    }

    fn esp8266(&self) -> Option<&Esp8266Client> {
        self.esp8266.as_ref()
    }

    fn ingest_key(&mut self, key: Keycode, pressed: bool) {
        if !pressed {
            return;
        }
        match key {
            Keycode::Left | Keycode::Down => self.rotary_delta -= 1,
            Keycode::Right | Keycode::Up => self.rotary_delta += 1,
            Keycode::Space | Keycode::Return => self.button_down = true,
            _ => {}
        }
    }

}

impl Platform for SdlPlatform {
    async fn init(&mut self) -> Result<(), String> {
        eprintln!(
            "[storage] Linux XDG → config {}, data {}, state {}, cache {}",
            xdg_storage::xdg_config_dir().display(),
            xdg_storage::xdg_data_dir().display(),
            xdg_storage::xdg_state_dir().display(),
            xdg_storage::xdg_cache_dir().display(),
        );

        let cfg = load_esp8266_config();
        if !cfg.enabled {
            return Ok(());
        }
        match tokio::task::spawn_blocking(move || Esp8266Client::open(&cfg)).await {
            Ok(Ok(client)) => self.esp8266 = Some(client),
            Ok(Err(e)) => eprintln!("[esp8266] not available: {e}"),
            Err(e) => eprintln!("[esp8266] init task failed: {e}"),
        }
        Ok(())
    }

    async fn draw_text(&mut self, text: &str, x: i32, y: i32, size: u8, color: u32) {
        let Some(font) = self.font else {
            return;
        };
        let surface = match font.render(text).blended(Self::rgb(color)) {
            Ok(s) => s,
            Err(_) => return,
        };
        let creator = self.canvas.texture_creator();
        let texture = match creator.create_texture_from_surface(&surface) {
            Ok(t) => t,
            Err(_) => return,
        };
        let q = texture.query();
        let pt = if size == 0 { self.font_pt } else { size as u16 };
        let scale = pt as f32 / self.font_pt as f32;
        let w = ((q.width as f32) * scale).round().max(1.0) as u32;
        let h = ((q.height as f32) * scale).round().max(1.0) as u32;
        let _ = self.canvas.copy(&texture, None, Rect::new(x, y, w, h));
    }

    async fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32, _thickness: u8) {
        self.canvas.set_draw_color(Self::rgb(color));
        let _ = self.canvas.draw_line(
            sdl2::rect::Point::new(x1, y1),
            sdl2::rect::Point::new(x2, y2),
        );
    }

    async fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: u32) {
        self.draw_circle_impl(cx, cy, radius, Self::rgb(color));
    }

    async fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        self.canvas.set_draw_color(Self::rgb(color));
        let _ = self
            .canvas
            .fill_rect(Rect::new(x, y, w.max(1) as u32, h.max(1) as u32));
    }

    async fn draw_clock_face(&mut self, cx: i32, cy: i32, diameter: u32) {
        crate::modules::faces::draw_face(&mut self.canvas, cx, cy, diameter);
    }

    async fn draw_clock_hour_hand(
        &mut self,
        cx: i32,
        cy: i32,
        length: i32,
        angle_deg: f32,
        night: bool,
    ) {
        crate::modules::faces::draw_hour_hand(
            &mut self.canvas,
            cx,
            cy,
            length,
            angle_deg,
            night,
        );
    }

    async fn draw_clock_minute_hand(
        &mut self,
        cx: i32,
        cy: i32,
        length: i32,
        angle_deg: f32,
        night: bool,
    ) {
        crate::modules::faces::draw_minute_hand(
            &mut self.canvas,
            cx,
            cy,
            length,
            angle_deg,
            night,
        );
    }

    async fn draw_clock_hub(&mut self, cx: i32, cy: i32, night: bool) {
        crate::modules::faces::draw_hub(&mut self.canvas, cx, cy, night);
    }

    async fn draw_clock_second_hand(
        &mut self,
        cx: i32,
        cy: i32,
        length: i32,
        angle_deg: f32,
        night: bool,
    ) {
        crate::modules::faces::draw_second_hand(
            &mut self.canvas,
            cx,
            cy,
            length,
            angle_deg,
            night,
        );
    }

    async fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let _ = self.canvas.clear();
    }

    async fn clear_center_area(&mut self) {
        let layout = l();
        self.draw_rect(
            layout.center_x,
            layout.center_y,
            layout.center_w as i32,
            layout.center_h as i32,
            0x000000,
        )
        .await;
    }

    async fn present(&mut self) {
        let _ = self.canvas.present();
    }

    async fn play_sound(&mut self, path: &str, _volume: f32) {
        self.audio.play_one_shot(path);
    }

    async fn play_raw_audio(&mut self, path: &str) {
        self.play_sound(path, 1.0).await;
    }

    async fn play_alarm_loop(&mut self, path: &str) {
        self.audio.play_alarm_loop(path);
    }

    async fn stop_alarm_sound(&mut self) {
        self.audio.stop_alarm();
    }

    fn get_current_time(&self) -> DateTime<Local> {
        Local::now()
    }

    fn delay_ms(&self, ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }

    async fn delay(&self, ms: u64) {
        self.delay_ms(ms);
    }

    async fn fetch_weather(&self) -> Result<(i32, String), String> {
        let loaded = crate::modules::weather::load_weather_config_loaded();
        let city = crate::modules::weather::cache::resolve_city(&loaded.config, &loaded.meta);
        let snapshot = if let Some(client) = self.esp8266.clone() {
            crate::modules::weather::fetch_weather_data_with_http(&loaded.config, &city, move |url| {
                client.http_get(url)
            })?
        } else {
            crate::modules::weather::fetch_weather_data(&loaded.config, &city)?
        };
        Ok((snapshot.temp.round() as i32, snapshot.condition))
    }

    async fn write_file(&mut self, path: &str, data: &[u8]) {
        if let Err(e) = xdg_storage::write_file(path, data) {
            eprintln!("[storage] write failed ({path}): {e}");
        }
    }

    async fn read_file(&mut self, path: &str) -> Option<Vec<u8>> {
        xdg_storage::read_file(path).or_else(|| {
            resolve_media_path(path)
                .and_then(|p| std::fs::read(p).ok())
        })
    }

    async fn copy_file(&mut self, from: &str, to: &str) {
        if let Err(e) = xdg_storage::copy_file(from, to) {
            eprintln!("[storage] copy failed ({from} → {to}): {e}");
        }
    }

    fn read_rotary_delta(&mut self) -> i32 {
        let d = self.rotary_delta;
        self.rotary_delta = 0;
        d
    }

    fn read_pushbutton(&mut self) -> bool {
        let b = self.button_down;
        self.button_down = false;
        b
    }

    async fn speak(&mut self, message: &str) {
        crate::clock_core::voice_feedback::VoiceFeedback::speak(self, message).await;
    }

    async fn show_weather(&mut self) {}

    async fn esp8266_mqtt_connect(
        &mut self,
        broker: &str,
        port: u16,
        user: Option<&str>,
        pass: Option<&str>,
    ) {
        let Some(client) = self.esp8266.clone() else {
            return;
        };
        let broker = broker.to_string();
        let user = user.map(str::to_string);
        let pass = pass.map(str::to_string);
        let result = tokio::task::spawn_blocking(move || {
            client.mqtt_connect(&broker, port, user.as_deref(), pass.as_deref())
        })
        .await;
        if let Ok(Err(e)) = result {
            eprintln!("[esp8266] mqtt connect failed: {e}");
        }
    }

    async fn esp8266_mqtt_publish(&mut self, topic: &str, payload: &str, retain: bool) {
        let Some(client) = self.esp8266.clone() else {
            return;
        };
        let topic = topic.to_string();
        let payload = payload.to_string();
        let result =
            tokio::task::spawn_blocking(move || client.mqtt_publish(&topic, &payload, retain)).await;
        if let Ok(Err(e)) = result {
            eprintln!("[esp8266] mqtt publish failed: {e}");
        }
    }

    async fn esp8266_mqtt_subscribe(&mut self, topic: &str) {
        let Some(client) = self.esp8266.clone() else {
            return;
        };
        let topic = topic.to_string();
        let result = tokio::task::spawn_blocking(move || client.mqtt_subscribe(&topic)).await;
        if let Ok(Err(e)) = result {
            eprintln!("[esp8266] mqtt subscribe failed: {e}");
        }
    }

    async fn esp8266_get_ntp(&mut self, server: &str) -> Option<String> {
        let client = self.esp8266.clone()?;
        let server = server.to_string();
        tokio::task::spawn_blocking(move || client.ntp(&server).ok())
            .await
            .ok()?
    }

    async fn http_download_binary(&mut self, url: &str) -> Option<Vec<u8>> {
        if let Some(client) = self.esp8266.clone() {
            let url = url.to_string();
            if let Ok(Ok(data)) = tokio::task::spawn_blocking(move || client.http_get(&url)).await {
                return Some(data);
            }
        }
        tokio::task::spawn_blocking({
            let url = url.to_string();
            move || {
                let response = ureq::get(&url).call().ok()?;
                let mut body = Vec::new();
                response.into_reader().read_to_end(&mut body).ok()?;
                Some(body)
            }
        })
        .await
        .ok()?
    }

    fn is_linux(&self) -> bool {
        true
    }

    fn is_tock(&self) -> bool {
        false
    }

    async fn run_forever(&mut self) {
        loop {
            self.delay(1000).await;
        }
    }

    fn reboot(&mut self) {}
}