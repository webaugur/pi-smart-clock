use crate::icons::draw_symbolic_icon;
use chrono::Timelike;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeatherIcon {
    Clear,
    PartlyCloudy,
    Cloudy,
    Fog,
    Drizzle,
    Rain,
    Snow,
    Thunderstorm,
    Unknown,
}

impl WeatherIcon {
    pub fn from_code(code: u16) -> Self {
        match code {
            0 => Self::Clear,
            1 | 2 => Self::PartlyCloudy,
            3 => Self::Cloudy,
            45 | 48 => Self::Fog,
            51 | 53 | 55 | 56 | 57 => Self::Drizzle,
            61 | 63 | 65 | 66 | 67 | 80 | 81 | 82 => Self::Rain,
            71 | 73 | 75 | 77 | 85 | 86 => Self::Snow,
            95 | 96 | 99 => Self::Thunderstorm,
            _ => Self::Unknown,
        }
    }

    fn asset_path(self, night: bool) -> &'static str {
        match self {
            Self::Clear if night => "status/weather-clear-night-symbolic.svg",
            Self::Clear => "status/weather-clear-symbolic.svg",
            Self::PartlyCloudy => "status/weather-few-clouds-symbolic.svg",
            Self::Cloudy => "status/weather-overcast-symbolic.svg",
            Self::Fog => "status/weather-fog-symbolic.svg",
            Self::Drizzle => "status/weather-showers-scattered-symbolic.svg",
            Self::Rain => "status/weather-showers-symbolic.svg",
            Self::Snow => "status/weather-snow-symbolic.svg",
            Self::Thunderstorm => "status/weather-storm-symbolic.svg",
            Self::Unknown => "status/adw-tab-icon-missing-symbolic.svg",
        }
    }
}

pub fn wmo_condition(code: u16) -> &'static str {
    match code {
        0 => "Clear sky",
        1 => "Mainly clear",
        2 => "Partly cloudy",
        3 => "Overcast",
        45 => "Fog",
        48 => "Depositing rime fog",
        51 => "Light drizzle",
        53 => "Drizzle",
        55 => "Dense drizzle",
        56 => "Freezing drizzle",
        57 => "Dense freezing drizzle",
        61 => "Slight rain",
        63 => "Rain",
        65 => "Heavy rain",
        66 => "Freezing rain",
        67 => "Heavy freezing rain",
        71 => "Slight snow",
        73 => "Snow",
        75 => "Heavy snow",
        77 => "Snow grains",
        80 => "Rain showers",
        81 => "Heavy rain showers",
        82 => "Violent rain showers",
        85 => "Snow showers",
        86 => "Heavy snow showers",
        95 => "Thunderstorm",
        96 => "Thunderstorm with hail",
        99 => "Thunderstorm with heavy hail",
        _ => "Unknown",
    }
}

pub fn draw_weather_icon(
    canvas: &mut Canvas<Window>,
    icon: WeatherIcon,
    x: i32,
    y: i32,
    size: u32,
) {
    let hour = chrono::Local::now().hour();
    let night = hour >= 20 || hour < 6;
    let tint = weather_tint(icon);
    draw_symbolic_icon(canvas, icon.asset_path(night), x, y, size, tint);
}

fn weather_tint(icon: WeatherIcon) -> Color {
    match icon {
        WeatherIcon::Clear => Color::RGB(255, 210, 72),
        WeatherIcon::PartlyCloudy => Color::RGB(255, 210, 72),
        WeatherIcon::Cloudy => Color::RGB(175, 188, 205),
        WeatherIcon::Fog => Color::RGB(155, 165, 180),
        WeatherIcon::Drizzle => Color::RGB(110, 175, 255),
        WeatherIcon::Rain => Color::RGB(72, 145, 255),
        WeatherIcon::Snow => Color::RGB(220, 238, 255),
        WeatherIcon::Thunderstorm => Color::RGB(255, 220, 48),
        WeatherIcon::Unknown => Color::RGB(160, 170, 190),
    }
}