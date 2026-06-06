use std::path::Path;

#[cfg(feature = "linux-full")]
use crate::storage::linux as xdg_storage;

#[derive(Clone, Debug)]
pub struct Esp8266Config {
    pub enabled: bool,
    pub port: String,
    pub baud: u32,
    pub wifi_ssid: String,
    pub wifi_password: String,
}

impl Default for Esp8266Config {
    fn default() -> Self {
        Self {
            enabled: false,
            port: "auto".to_string(),
            baud: 115_200,
            wifi_ssid: String::new(),
            wifi_password: String::new(),
        }
    }
}

pub fn load_esp8266_config() -> Esp8266Config {
    if let Some(path) = resolve_esp8266_config_path() {
        if let Ok(cfg) = parse_config(&path) {
            eprintln!("[esp8266] loaded {}", path.display());
            return cfg;
        }
    }
    Esp8266Config::default()
}

fn resolve_esp8266_config_path() -> Option<std::path::PathBuf> {
    #[cfg(feature = "linux-full")]
    {
        return xdg_storage::find_config("esp8266.conf", "esp8266.conf.example");
    }
    #[cfg(not(feature = "linux-full"))]
    {
        let path = Path::new("config/esp8266.conf");
        if path.is_file() {
            return Some(path.to_path_buf());
        }
        let example = Path::new("config/esp8266.conf.example");
        if example.is_file() {
            return Some(example.to_path_buf());
        }
        None
    }
}

fn parse_config(path: &Path) -> Result<Esp8266Config, String> {
    if !path.is_file() {
        return Err(format!("{} not found", path.display()));
    }
    let mut cfg = Esp8266Config::default();
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        match key.as_str() {
            "enabled" => cfg.enabled = parse_bool(value),
            "port" => cfg.port = value.to_string(),
            "baud" => cfg.baud = value.parse().unwrap_or(cfg.baud),
            "wifi_ssid" | "ssid" => cfg.wifi_ssid = value.to_string(),
            "wifi_password" | "wifi_pass" | "password" => cfg.wifi_password = value.to_string(),
            _ => {}
        }
    }
    Ok(cfg)
}

fn parse_bool(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_config() {
        let cfg = parse_config(
            &resolve_esp8266_config_path().expect("esp8266 example config"),
        )
        .unwrap();
        assert!(!cfg.enabled);
        assert_eq!(cfg.baud, 115_200);
    }
}