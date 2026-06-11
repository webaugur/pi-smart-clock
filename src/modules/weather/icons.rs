use chrono::Timelike;
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
        // Playful cartoony icons (adapted from Meteocons Fill / Tabler MIT sets + project styling
        // for high-sat, chunky, room-visible look). Files under assets/icons/playful/status/.
        // Hi-res variants (e.g. sun.hires.svg) are auto-selected by the atlas for large sizes.
        match self {
            Self::Clear if night => "status/moon.svg",
            Self::Clear => "status/sun.svg",
            Self::PartlyCloudy => "status/cloud-sun.svg",
            Self::Cloudy => "status/cloud.svg",
            Self::Fog => "status/fog.svg",
            Self::Drizzle => "status/cloud-rain.svg",
            Self::Rain => "status/cloud-rain.svg",
            Self::Snow => "status/cloud-snow.svg",
            Self::Thunderstorm => "status/cloud-storm.svg",
            Self::Unknown => "status/help.svg",
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
    // Use playful cartoony icons (colors baked in; hi/lo variants handled by atlas)
    crate::icons::draw_icon(canvas, icon.asset_path(night), x, y, size);
}