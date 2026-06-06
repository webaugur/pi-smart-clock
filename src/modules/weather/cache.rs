use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::api::{fetch_weather_data, WeatherSnapshot};
use super::config::{ConfigMeta, WeatherConfig};
use super::icons::WeatherIcon;
use crate::storage::logical;

const CITY_CACHE: &str = logical::WEATHER_CITY_CACHE;
const DATA_CACHE: &str = logical::WEATHER_DATA_CACHE;
const CITY_MAX_AGE: Duration = Duration::from_secs(60 * 60);
const DEFAULT_UPDATE_MINUTES: u64 = 30;

#[derive(Serialize, Deserialize)]
struct CityCacheFile {
    city: String,
    latitude: f64,
    longitude: f64,
    config_mtime: u64,
    looked_up_at: u64,
}

#[derive(Serialize, Deserialize)]
struct WeatherCacheFile {
    latitude: f64,
    longitude: f64,
    units: String,
    config_mtime: u64,
    update_interval_minutes: u64,
    fetched_at: u64,
    city: String,
    temp: f32,
    humidity: u8,
    condition: String,
    weather_code: u16,
    aqi: Option<u32>,
    aqi_label: String,
}

pub fn resolve_city(config: &WeatherConfig, meta: &ConfigMeta) -> String {
    if let Some(ref city) = config.city {
        let city = city.clone();
        let _ = save_city_cache(config, meta, &city);
        return city;
    }

    let config_mtime = meta.modified.map(system_time_secs).unwrap_or(0);
    if let Some(cache) = read_city_cache() {
        if cache.latitude == config.latitude
            && cache.longitude == config.longitude
            && cache.config_mtime == config_mtime
            && city_cache_age(&cache) < CITY_MAX_AGE
        {
            eprintln!("[weather] city cache hit → {}", cache.city);
            return cache.city;
        }
    }

    eprintln!("[weather] looking up city name…");
    let city = super::api::fetch_city_name(config.latitude, config.longitude)
        .unwrap_or_else(|| "Nearby".to_string());
    let _ = save_city_cache(config, meta, &city);
    city
}

pub fn load_weather_snapshot(
    config: &WeatherConfig,
    meta: &ConfigMeta,
    city: &str,
) -> Option<WeatherSnapshot> {
    let cache = read_weather_cache()?;
    if !weather_cache_valid(&cache, config, meta) {
        return None;
    }
    eprintln!("[weather] data cache hit");
    Some(snapshot_from_cache(cache, city))
}

pub fn refresh_weather_snapshot(
    config: &WeatherConfig,
    meta: &ConfigMeta,
    city: &str,
    alerts_active: bool,
) -> Result<WeatherSnapshot, String> {
    let interval = update_interval(config, alerts_active);

    if let Some(cache) = read_weather_cache() {
        if weather_cache_valid(&cache, config, meta) && weather_cache_age(&cache) < interval {
            return Ok(snapshot_from_cache(cache, city));
        }
    }

    let snapshot = fetch_weather_data(config, city)?;
    save_weather_cache(config, meta, &snapshot)?;
    Ok(snapshot)
}

pub fn update_interval(config: &WeatherConfig, alerts_active: bool) -> Duration {
    if alerts_active {
        Duration::from_secs(5 * 60)
    } else {
        Duration::from_secs(config.update_interval_minutes * 60)
    }
}

pub fn default_update_minutes() -> u64 {
    DEFAULT_UPDATE_MINUTES
}

fn weather_cache_valid(cache: &WeatherCacheFile, config: &WeatherConfig, meta: &ConfigMeta) -> bool {
    let config_mtime = meta.modified.map(system_time_secs).unwrap_or(0);
    cache.latitude == config.latitude
        && cache.longitude == config.longitude
        && cache.units == config.units.open_meteo_param()
        && cache.config_mtime == config_mtime
        && cache.update_interval_minutes == config.update_interval_minutes
        && cache_file_mtime(DATA_CACHE)
            .map(|mtime| mtime >= config_mtime)
            .unwrap_or(false)
}

fn weather_cache_age(cache: &WeatherCacheFile) -> Duration {
    age_since(cache.fetched_at)
}

fn city_cache_age(cache: &CityCacheFile) -> Duration {
    age_since(cache.looked_up_at)
}

fn age_since(unix_secs: u64) -> Duration {
    let now = now_secs();
    Duration::from_secs(now.saturating_sub(unix_secs))
}

fn snapshot_from_cache(cache: WeatherCacheFile, city: &str) -> WeatherSnapshot {
    WeatherSnapshot {
        temp: cache.temp,
        humidity: cache.humidity,
        condition: cache.condition,
        weather_code: cache.weather_code,
        icon: WeatherIcon::from_code(cache.weather_code),
        aqi: cache.aqi,
        aqi_label: cache.aqi_label,
        city: if city.is_empty() { cache.city } else { city.to_string() },
    }
}

fn read_city_cache() -> Option<CityCacheFile> {
    read_json(CITY_CACHE)
}

fn read_weather_cache() -> Option<WeatherCacheFile> {
    read_json(DATA_CACHE)
}

fn save_city_cache(config: &WeatherConfig, meta: &ConfigMeta, city: &str) -> Result<(), String> {
    let file = CityCacheFile {
        city: city.to_string(),
        latitude: config.latitude,
        longitude: config.longitude,
        config_mtime: meta.modified.map(system_time_secs).unwrap_or(0),
        looked_up_at: now_secs(),
    };
    write_json(CITY_CACHE, &file)
}

fn save_weather_cache(
    config: &WeatherConfig,
    meta: &ConfigMeta,
    snapshot: &WeatherSnapshot,
) -> Result<(), String> {
    let file = WeatherCacheFile {
        latitude: config.latitude,
        longitude: config.longitude,
        units: config.units.open_meteo_param().to_string(),
        config_mtime: meta.modified.map(system_time_secs).unwrap_or(0),
        update_interval_minutes: config.update_interval_minutes,
        fetched_at: now_secs(),
        city: snapshot.city.clone(),
        temp: snapshot.temp,
        humidity: snapshot.humidity,
        condition: snapshot.condition.clone(),
        weather_code: snapshot.weather_code,
        aqi: snapshot.aqi,
        aqi_label: snapshot.aqi_label.clone(),
    };
    write_json(DATA_CACHE, &file)
}

fn read_json<T: for<'de> Deserialize<'de>>(name: &str) -> Option<T> {
    let path = resolve_cache_path(name)?;
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn write_json<T: Serialize>(name: &str, value: &T) -> Result<(), String> {
    let path = resolve_cache_path(name).unwrap_or_else(|| PathBuf::from(name));
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    fs::write(&path, data).map_err(|e| e.to_string())?;
    Ok(())
}

fn resolve_cache_path(name: &str) -> Option<PathBuf> {
    #[cfg(feature = "linux-full")]
    {
        let path = crate::storage::linux::resolve_logical_path(name);
        return Some(path);
    }
    #[cfg(not(feature = "linux-full"))]
    {
        let _ = name;
        None
    }
}

fn cache_file_mtime(name: &str) -> Option<u64> {
    let path = resolve_cache_path(name)?;
    let modified = fs::metadata(path).ok()?.modified().ok()?;
    Some(system_time_secs(modified))
}

fn system_time_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn now_secs() -> u64 {
    system_time_secs(SystemTime::now())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::weather::api;
    use crate::modules::weather::config::TemperatureUnit;

    #[test]
    fn weather_cache_invalid_when_config_newer() {
        let config = WeatherConfig {
            latitude: 40.0,
            longitude: -86.0,
            timezone: "auto".into(),
            units: TemperatureUnit::Fahrenheit,
            update_interval_minutes: 30,
            city: None,
        };
        let meta = ConfigMeta {
            path: None,
            modified: Some(UNIX_EPOCH + Duration::from_secs(2000)),
        };
        let cache = WeatherCacheFile {
            latitude: 40.0,
            longitude: -86.0,
            units: "fahrenheit".into(),
            config_mtime: 1000,
            update_interval_minutes: 30,
            fetched_at: now_secs(),
            city: "Test".into(),
            temp: 70.0,
            humidity: 50,
            condition: "Clear".into(),
            weather_code: 0,
            aqi: Some(42),
            aqi_label: api::aqi_label(42),
        };
        assert!(!weather_cache_valid(&cache, &config, &meta));
    }
}