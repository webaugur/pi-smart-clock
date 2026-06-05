mod api;
pub mod cache;
mod config;
mod icons;

use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::modules::bottom_module::{BottomModule, PanelLine};
use crate::modules::module_id::ModuleId;
use crate::panel::Panel;

pub use api::{fetch_weather, fetch_weather_data, format_temp, WeatherSnapshot};
pub use config::{
    load_weather_config, load_weather_config_loaded, ConfigMeta, TemperatureUnit, WeatherConfig,
};

pub struct WeatherPanel {
    config: WeatherConfig,
    config_meta: ConfigMeta,
    city_name: String,
    snapshot: Option<WeatherSnapshot>,
    last_refresh: Option<Instant>,
    last_error: Option<String>,
    fetching: bool,
}

impl WeatherPanel {
    pub fn new() -> Self {
        let loaded = config::load_weather_config_loaded();
        let config = loaded.config.clone();
        let city_name = cache::resolve_city(&config, &loaded.meta);
        eprintln!("[weather] location → {city_name}");

        let snapshot = cache::load_weather_snapshot(&config, &loaded.meta, &city_name);
        let last_refresh = snapshot.as_ref().map(|_| Instant::now());

        Self {
            config,
            config_meta: loaded.meta,
            city_name,
            snapshot,
            last_refresh,
            last_error: None,
            fetching: false,
        }
    }

    pub fn panel_title(&self) -> String {
        let city = self
            .snapshot
            .as_ref()
            .map(|s| s.city.as_str())
            .unwrap_or(self.city_name.as_str());
        truncate(city, 18)
    }

    pub fn refresh_if_due(&mut self, alerts_active: bool) {
        if self.fetching {
            return;
        }

        self.reload_config_if_changed();

        let interval = cache::update_interval(&self.config, alerts_active);
        let due = self
            .last_refresh
            .map(|t| t.elapsed() >= interval)
            .unwrap_or(true);

        if !due {
            return;
        }

        self.fetching = true;
        match cache::refresh_weather_snapshot(
            &self.config,
            &self.config_meta,
            &self.city_name,
            alerts_active,
        ) {
            Ok(snapshot) => {
                eprintln!(
                    "[weather] {} • {:.0}{} {} • {}% RH • {}",
                    snapshot.city,
                    snapshot.temp.round(),
                    self.config.units.symbol(),
                    snapshot.condition,
                    snapshot.humidity,
                    snapshot.aqi_line()
                );
                self.city_name = snapshot.city.clone();
                self.snapshot = Some(snapshot);
                self.last_error = None;
            }
            Err(e) => {
                eprintln!("[weather] fetch failed: {e}");
                self.last_error = Some(e);
            }
        }
        self.last_refresh = Some(Instant::now());
        self.fetching = false;
    }

    fn reload_config_if_changed(&mut self) {
        let Some(loaded) = config::reload_weather_config_if_changed(&self.config_meta) else {
            return;
        };

        eprintln!("[weather] config changed, refreshing location and cache");
        self.config = loaded.config;
        self.config_meta = loaded.meta;
        self.city_name = cache::resolve_city(&self.config, &self.config_meta);
        if let Some(snapshot) =
            cache::load_weather_snapshot(&self.config, &self.config_meta, &self.city_name)
        {
            self.snapshot = Some(snapshot);
            self.last_refresh = Some(Instant::now());
        } else {
            self.snapshot = None;
            self.last_refresh = None;
        }
    }

    pub fn has_data(&self) -> bool {
        self.snapshot.is_some()
    }

    pub fn temp_display(&self) -> String {
        match &self.snapshot {
            Some(s) => api::format_temp(s.temp, self.config.units),
            None => "--".to_string(),
        }
    }

    pub fn condition(&self) -> &str {
        self.snapshot
            .as_ref()
            .map(|s| s.condition.as_str())
            .unwrap_or("Loading...")
    }

    pub fn humidity_line(&self) -> String {
        match &self.snapshot {
            Some(s) => format!("Humidity {}%", s.humidity),
            None => "Humidity --".to_string(),
        }
    }

    pub fn aqi_line(&self) -> String {
        self.snapshot
            .as_ref()
            .map(|s| s.aqi_line())
            .unwrap_or_else(|| "AQI --".to_string())
    }

    pub fn status_line(&self) -> String {
        if let Some(err) = &self.last_error {
            return truncate(err, 28);
        }
        if self.fetching {
            return "Updating...".to_string();
        }
        if self.snapshot.is_none() {
            return "Waiting for data...".to_string();
        }
        String::new()
    }

    pub fn icon(&self) -> icons::WeatherIcon {
        self.snapshot
            .as_ref()
            .map(|s| s.icon)
            .unwrap_or(icons::WeatherIcon::Unknown)
    }

    /// Legacy hook used by the scheduler stub path.
    pub fn set_weather(&mut self, temp: i32, condition: String) {
        self.snapshot = Some(WeatherSnapshot {
            temp: temp as f32,
            humidity: 0,
            condition,
            weather_code: 0,
            icon: icons::WeatherIcon::Unknown,
            aqi: None,
            aqi_label: "Unknown".to_string(),
            city: self.city_name.clone(),
        });
    }

    pub fn temp(&self) -> i32 {
        self.snapshot
            .as_ref()
            .map(|s| s.temp.round() as i32)
            .unwrap_or(0)
    }
}

impl BottomModule for WeatherPanel {
    fn id(&self) -> ModuleId {
        ModuleId::Weather
    }

    fn draw_background(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        self.draw(canvas, x, y, w, h);
    }

    fn title(&self) -> (String, u32) {
        (self.panel_title(), 0x00FFAA)
    }

    fn lines(&self) -> Vec<PanelLine> {
        let mut lines = vec![
            PanelLine {
                text: self.temp_display(),
                size_pt: 36,
            },
            PanelLine {
                text: self.condition().to_string(),
                size_pt: 0,
            },
            PanelLine {
                text: self.humidity_line(),
                size_pt: 0,
            },
            PanelLine {
                text: self.aqi_line(),
                size_pt: 0,
            },
        ];
        let status = self.status_line();
        if !status.is_empty() {
            lines.push(PanelLine {
                text: status,
                size_pt: 0,
            });
        }
        lines
    }

    fn tick(&mut self, alerts_active: bool) {
        self.refresh_if_due(alerts_active);
    }
}

impl Panel for WeatherPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(17, 17, 17));
        let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
        canvas.set_draw_color(Color::RGB(0, 255, 170));
        let _ = canvas.fill_rect(Rect::new(x + 4, y + 4, (w - 8) as u32, 4));

        let icon_size = ((h - 20).max(80) as u32).min(112);
        let icon_x = x + w - icon_size as i32 - 6;
        let icon_y = y + (h - icon_size as i32) / 2;
        icons::draw_weather_icon(canvas, self.icon(), icon_x, icon_y, icon_size);
    }

    fn update(&mut self) {}
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    s.chars().take(max.saturating_sub(1)).collect::<String>() + "…"
}