use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::platform::linux_audio::resolve_media_path;

const CONFIG_PATHS: [&str; 2] = ["config/weather.conf", "config/weather.conf.example"];
const DEFAULT_UPDATE_MINUTES: u64 = 30;

#[derive(Clone, Debug, PartialEq)]
pub struct WeatherConfig {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub units: TemperatureUnit,
    pub update_interval_minutes: u64,
    /// Optional override; otherwise reverse-geocoded from lat/lon.
    pub city: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemperatureUnit {
    Fahrenheit,
    Celsius,
}

impl TemperatureUnit {
    pub fn open_meteo_param(self) -> &'static str {
        match self {
            Self::Fahrenheit => "fahrenheit",
            Self::Celsius => "celsius",
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Fahrenheit => "°F",
            Self::Celsius => "°C",
        }
    }
}

impl Default for WeatherConfig {
    fn default() -> Self {
        Self {
            latitude: 39.7684,
            longitude: -86.1581,
            timezone: "auto".to_string(),
            units: TemperatureUnit::Fahrenheit,
            update_interval_minutes: DEFAULT_UPDATE_MINUTES,
            city: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConfigMeta {
    pub path: Option<PathBuf>,
    pub modified: Option<SystemTime>,
}

#[derive(Clone, Debug)]
pub struct LoadedWeatherConfig {
    pub config: WeatherConfig,
    pub meta: ConfigMeta,
}

pub fn load_weather_config() -> WeatherConfig {
    load_weather_config_loaded().config
}

pub fn load_weather_config_loaded() -> LoadedWeatherConfig {
    load_weather_config_from(resolve_weather_config(), true)
}

pub fn reload_weather_config_if_changed(meta: &ConfigMeta) -> Option<LoadedWeatherConfig> {
    let path = meta.path.as_ref()?;
    let modified = fs::metadata(path).ok().and_then(|m| m.modified().ok())?;
    if meta.modified == Some(modified) {
        return None;
    }
    Some(load_weather_config_from(Some(path.clone()), true))
}

fn load_weather_config_from(path: Option<PathBuf>, log: bool) -> LoadedWeatherConfig {
    let Some(path) = path else {
        if log {
            eprintln!("[weather] no config found, using defaults");
        }
        return LoadedWeatherConfig {
            config: WeatherConfig::default(),
            meta: ConfigMeta {
                path: None,
                modified: None,
            },
        };
    };

    let modified = fs::metadata(&path).ok().and_then(|m| m.modified().ok());
    match parse_weather_config(&path) {
        Ok(cfg) => {
            if log {
                eprintln!(
                    "[weather] loaded {} → {:.4}, {:.4} ({}), refresh every {} min",
                    path.display(),
                    cfg.latitude,
                    cfg.longitude,
                    cfg.units.open_meteo_param(),
                    cfg.update_interval_minutes
                );
            }
            LoadedWeatherConfig {
                config: cfg,
                meta: ConfigMeta {
                    path: Some(path),
                    modified,
                },
            }
        }
        Err(e) => {
            if log {
                eprintln!("[weather] {e}, using defaults");
            }
            LoadedWeatherConfig {
                config: WeatherConfig::default(),
                meta: ConfigMeta {
                    path: Some(path),
                    modified,
                },
            }
        }
    }
}

fn resolve_weather_config() -> Option<std::path::PathBuf> {
    for path in CONFIG_PATHS {
        if let Some(resolved) = resolve_media_path(path) {
            return Some(resolved);
        }
        let p = Path::new(path);
        if p.exists() {
            return Some(p.to_path_buf());
        }
    }
    None
}

fn parse_weather_config(path: &Path) -> Result<WeatherConfig, String> {
    let mut cfg = WeatherConfig::default();
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();

        match key.as_str() {
            "latitude" | "lat" => {
                cfg.latitude = value.parse().map_err(|_| format!("bad latitude: {value}"))?;
            }
            "longitude" | "lon" | "lng" => {
                cfg.longitude = value.parse().map_err(|_| format!("bad longitude: {value}"))?;
            }
            "timezone" | "tz" => cfg.timezone = value.to_string(),
            "units" | "temperature_unit" => {
                cfg.units = match value.to_ascii_lowercase().as_str() {
                    "fahrenheit" | "f" | "farenheit" => TemperatureUnit::Fahrenheit,
                    "celsius" | "c" | "centigrade" => TemperatureUnit::Celsius,
                    other => return Err(format!("unknown units: {other}")),
                };
            }
            "update_interval_minutes" | "update_minutes" | "interval" => {
                cfg.update_interval_minutes = value
                    .parse()
                    .map_err(|_| format!("bad update_interval_minutes: {value}"))?;
            }
            "city" | "location" => {
                if !value.is_empty() {
                    cfg.city = Some(value.to_string());
                }
            }
            _ => {}
        }
    }

    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_config() {
        let cfg = parse_weather_config(Path::new("config/weather.conf.example")).unwrap();
        assert!((cfg.latitude - 39.7684).abs() < 0.0001);
        assert!((cfg.longitude - (-86.1581)).abs() < 0.0001);
        assert_eq!(cfg.units, TemperatureUnit::Fahrenheit);
        assert_eq!(cfg.update_interval_minutes, 15);
    }

    #[test]
    fn default_update_interval_is_30_minutes() {
        assert_eq!(WeatherConfig::default().update_interval_minutes, 30);
    }
}