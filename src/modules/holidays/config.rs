use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[cfg(feature = "linux-full")]
use crate::storage::linux as xdg_storage;

/// Configuration for the holidays bottom panel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HolidaysConfig {
    /// ISO-like country/region code or name. e.g. "US", "GB", "CA".
    /// Determines the set of public holidays shown.
    pub country: String,
    /// How many holidays (from today onward) to consider when building the list.
    pub max_upcoming: usize,
}

impl Default for HolidaysConfig {
    fn default() -> Self {
        Self {
            country: "US".to_string(),
            max_upcoming: 6,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConfigMeta {
    pub path: Option<PathBuf>,
    pub modified: Option<SystemTime>,
}

#[derive(Clone, Debug)]
pub struct LoadedHolidaysConfig {
    pub config: HolidaysConfig,
    pub meta: ConfigMeta,
}

pub fn load_holidays_config_loaded() -> LoadedHolidaysConfig {
    load_holidays_config_from(resolve_holidays_config(), true)
}

pub fn reload_holidays_config_if_changed(meta: &ConfigMeta) -> Option<LoadedHolidaysConfig> {
    let path = meta.path.as_ref()?;
    let modified = fs::metadata(path).ok().and_then(|m| m.modified().ok())?;
    if meta.modified == Some(modified) {
        return None;
    }
    Some(load_holidays_config_from(Some(path.clone()), true))
}

fn resolve_holidays_config() -> Option<PathBuf> {
    #[cfg(feature = "linux-full")]
    {
        return xdg_storage::find_config("holidays.conf", "holidays.conf.example");
    }
    #[cfg(not(feature = "linux-full"))]
    {
        let path = Path::new("config/holidays.conf");
        if path.is_file() {
            return Some(path.to_path_buf());
        }
        let example = Path::new("config/holidays.conf.example");
        if example.is_file() {
            return Some(example.to_path_buf());
        }
        None
    }
}

fn load_holidays_config_from(path: Option<PathBuf>, log: bool) -> LoadedHolidaysConfig {
    let Some(path) = path else {
        if log {
            eprintln!("[holidays] no config found, using defaults (US)");
        }
        return LoadedHolidaysConfig {
            config: HolidaysConfig::default(),
            meta: ConfigMeta {
                path: None,
                modified: None,
            },
        };
    };

    let modified = fs::metadata(&path).ok().and_then(|m| m.modified().ok());
    match parse_holidays_config(&path) {
        Ok(cfg) => {
            if log {
                eprintln!(
                    "[holidays] loaded {} → country={}, max_upcoming={}",
                    path.display(),
                    cfg.country,
                    cfg.max_upcoming
                );
            }
            LoadedHolidaysConfig {
                config: cfg,
                meta: ConfigMeta {
                    path: Some(path),
                    modified,
                },
            }
        }
        Err(e) => {
            if log {
                eprintln!("[holidays] {e}, using defaults (US)");
            }
            LoadedHolidaysConfig {
                config: HolidaysConfig::default(),
                meta: ConfigMeta {
                    path: Some(path),
                    modified,
                },
            }
        }
    }
}

fn parse_holidays_config(path: &Path) -> Result<HolidaysConfig, String> {
    let mut cfg = HolidaysConfig::default();
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
            "country" | "region" | "locale" | "holidays" => {
                if !value.is_empty() {
                    cfg.country = normalize_country(value);
                }
            }
            "max_upcoming" | "max" | "count" => {
                if let Ok(n) = value.parse::<usize>() {
                    if n > 0 {
                        cfg.max_upcoming = n;
                    }
                }
            }
            _ => {}
        }
    }

    Ok(cfg)
}

fn normalize_country(s: &str) -> String {
    let s = s.trim().to_ascii_uppercase();
    match s.as_str() {
        "USA" | "UNITED STATES" | "UNITED_STATES" => "US".to_string(),
        "UK" | "BRITAIN" | "UNITED KINGDOM" | "UNITED_KINGDOM" => "GB".to_string(),
        "DEUTSCHLAND" => "DE".to_string(),
        "CANADA" => "CA".to_string(),
        "FRANCE" => "FR".to_string(),
        "AUSTRALIA" => "AU".to_string(),
        "JAPAN" => "JP".to_string(),
        "CHINA" | "PRC" | "CN" => "CN".to_string(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_config() {
        // The example may or may not exist at test time in all envs; fall back gracefully.
        if let Ok(cfg) = parse_holidays_config(Path::new("config/holidays.conf.example")) {
            assert_eq!(cfg.country, "US");
        }
    }

    #[test]
    fn normalizes_variants() {
        assert_eq!(normalize_country("uk"), "GB");
        assert_eq!(normalize_country("United States"), "US");
        assert_eq!(normalize_country("de"), "DE");
    }
}
