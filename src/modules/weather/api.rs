use std::io::Read;

use serde::Deserialize;

use super::config::{TemperatureUnit, WeatherConfig};
use super::icons::{WeatherIcon, wmo_condition};

#[derive(Clone, Debug)]
pub struct WeatherSnapshot {
    pub temp: f32,
    pub humidity: u8,
    pub condition: String,
    pub weather_code: u16,
    pub icon: WeatherIcon,
    pub aqi: Option<u32>,
    pub aqi_label: String,
    pub city: String,
}

impl WeatherSnapshot {
    pub fn aqi_line(&self) -> String {
        match self.aqi {
            Some(v) => format!("AQI {v} {}", self.aqi_label),
            None => "AQI unavailable".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct ForecastResponse {
    current: ForecastCurrent,
}

#[derive(Deserialize)]
struct ForecastCurrent {
    temperature_2m: f32,
    relative_humidity_2m: u8,
    weather_code: u16,
}

#[derive(Deserialize)]
struct AirQualityResponse {
    current: AirQualityCurrent,
}

#[derive(Deserialize)]
struct AirQualityCurrent {
    us_aqi: Option<u32>,
}

pub fn fetch_weather(config: &WeatherConfig) -> Result<WeatherSnapshot, String> {
    let city = if let Some(ref name) = config.city {
        name.clone()
    } else {
        fetch_city_name(config.latitude, config.longitude).unwrap_or_else(|| "Nearby".to_string())
    };
    fetch_weather_data(config, &city)
}

pub fn fetch_weather_data(config: &WeatherConfig, city: &str) -> Result<WeatherSnapshot, String> {
    fetch_weather_data_with_http(config, city, http_get_ureq)
}

pub fn fetch_weather_data_with_http<F>(config: &WeatherConfig, city: &str, http_get: F) -> Result<WeatherSnapshot, String>
where
    F: Fn(&str) -> Result<Vec<u8>, String>,
{
    let forecast = fetch_forecast_with(config, &http_get)?;
    let aqi = fetch_air_quality_with(config, &http_get).ok();

    let condition = wmo_condition(forecast.weather_code);
    let icon = WeatherIcon::from_code(forecast.weather_code);

    Ok(WeatherSnapshot {
        temp: forecast.temperature_2m,
        humidity: forecast.relative_humidity_2m,
        condition: condition.to_string(),
        weather_code: forecast.weather_code,
        icon,
        aqi,
        aqi_label: aqi.map(aqi_label).unwrap_or_else(|| "Unknown".to_string()),
        city: city.to_string(),
    })
}

#[derive(Deserialize)]
struct NominatimResponse {
    name: Option<String>,
    address: Option<NominatimAddress>,
}

#[derive(Deserialize)]
struct NominatimAddress {
    city: Option<String>,
    town: Option<String>,
    village: Option<String>,
    municipality: Option<String>,
    county: Option<String>,
}

pub fn fetch_city_name(latitude: f64, longitude: f64) -> Option<String> {
    let url = format!(
        "https://nominatim.openstreetmap.org/reverse?lat={latitude}&lon={longitude}&format=json&zoom=10"
    );

    let response: NominatimResponse = ureq::get(&url)
        .set("User-Agent", "pi-smart-clock/0.1.0 (linux smart clock display)")
        .call()
        .ok()?
        .into_json()
        .ok()?;

    if let Some(name) = response.name.filter(|n| !n.is_empty()) {
        return Some(name);
    }

    let addr = response.address?;
    addr.city
        .or(addr.town)
        .or(addr.village)
        .or(addr.municipality)
        .or(addr.county)
}

fn http_get_ureq(url: &str) -> Result<Vec<u8>, String> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| format!("request failed: {e}"))?;
    let mut body = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut body)
        .map_err(|e| format!("read body failed: {e}"))?;
    Ok(body)
}

fn fetch_forecast_with<F>(config: &WeatherConfig, http_get: &F) -> Result<ForecastCurrent, String>
where
    F: Fn(&str) -> Result<Vec<u8>, String>,
{
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,weather_code&temperature_unit={}&timezone={}",
        config.latitude,
        config.longitude,
        config.units.open_meteo_param(),
        &config.timezone
    );

    let body = http_get(&url).map_err(|e| format!("forecast request failed: {e}"))?;
    let response: ForecastResponse =
        serde_json::from_slice(&body).map_err(|e| format!("forecast parse failed: {e}"))?;

    Ok(response.current)
}

fn fetch_air_quality_with<F>(config: &WeatherConfig, http_get: &F) -> Result<u32, String>
where
    F: Fn(&str) -> Result<Vec<u8>, String>,
{
    let url = format!(
        "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&current=us_aqi",
        config.latitude, config.longitude
    );

    let body = http_get(&url).map_err(|e| format!("air quality request failed: {e}"))?;
    let response: AirQualityResponse =
        serde_json::from_slice(&body).map_err(|e| format!("air quality parse failed: {e}"))?;

    response
        .current
        .us_aqi
        .ok_or_else(|| "no us_aqi in response".to_string())
}

pub fn aqi_label(aqi: u32) -> String {
    match aqi {
        0..=50 => "Good".to_string(),
        51..=100 => "Moderate".to_string(),
        101..=150 => "Unhealthy (sensitive)".to_string(),
        151..=200 => "Unhealthy".to_string(),
        201..=300 => "Very unhealthy".to_string(),
        _ => "Hazardous".to_string(),
    }
}

pub fn format_temp(temp: f32, units: TemperatureUnit) -> String {
    format!("{:.0}{}", temp.round(), units.symbol())
}