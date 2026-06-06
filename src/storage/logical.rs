//! Virtual paths used across Linux and embedded builds.
//! Platform code maps these to real locations.

pub const ALARMS_CSV: &str = "/sd/config/alarms.csv";

pub fn alarms_backup(timestamp: &str) -> String {
    format!("/sd/config/alarms_{timestamp}.csv.bak")
}

pub const WEATHER_CITY_CACHE: &str = "cache/weather_city.json";
pub const WEATHER_DATA_CACHE: &str = "cache/weather_data.json";

pub fn alert_photo(alert_id: &str) -> String {
    format!("/sd/alerts/photo_{alert_id}.bmp")
}