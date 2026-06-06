use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub const APP_SLUG: &str = "pi-smart-clock";

pub fn home_dir() -> PathBuf {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

pub fn xdg_config_dir() -> PathBuf {
    env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".config"))
        .join(APP_SLUG)
}

pub fn xdg_data_dir() -> PathBuf {
    env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".local/share"))
        .join(APP_SLUG)
}

pub fn xdg_state_dir() -> PathBuf {
    env::var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".local/state"))
        .join(APP_SLUG)
}

pub fn xdg_cache_dir() -> PathBuf {
    env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".cache"))
        .join(APP_SLUG)
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn resolve_repo_path(name: &str) -> Option<PathBuf> {
    let rel = Path::new(name);
    if rel.is_absolute() && rel.is_file() {
        return Some(rel.to_path_buf());
    }
    let from_manifest = repo_root().join(name);
    if from_manifest.is_file() {
        return Some(from_manifest);
    }
    if rel.is_file() {
        return Some(rel.to_path_buf());
    }
    None
}

/// Map virtual `/sd/...` and `cache/...` paths to XDG locations on Linux.
pub fn resolve_logical_path(logical: &str) -> PathBuf {
    if let Some(relative) = logical.strip_prefix("/sd/") {
        if let Some(name) = relative.strip_prefix("config/") {
            if name.starts_with("alarms_") && name.ends_with(".csv.bak") {
                return xdg_state_dir().join("backups").join(name);
            }
            if name == "alarms.csv" {
                return xdg_data_dir().join("config").join(name);
            }
            return xdg_config_dir().join(name);
        }
        if relative.starts_with("alerts/") {
            return xdg_data_dir().join(relative);
        }
        return xdg_data_dir().join(relative);
    }

    if let Some(name) = logical.strip_prefix("cache/") {
        return xdg_cache_dir().join(name);
    }

    PathBuf::from(logical)
}

pub fn read_path_candidates(logical: &str) -> Vec<PathBuf> {
    let mut paths = vec![resolve_logical_path(logical)];
    match logical {
        crate::storage::logical::ALARMS_CSV => {
            push_repo_fallback(&mut paths, "config/alarms.csv");
            push_repo_fallback(&mut paths, "config/alarms.csv.example");
        }
        "/sd/config/weather.conf" => {
            push_repo_fallback(&mut paths, "config/weather.conf");
            push_repo_fallback(&mut paths, "config/weather.conf.example");
        }
        "/sd/config/alarms.csv.example" => {
            push_repo_fallback(&mut paths, "config/alarms.csv.example");
        }
        _ => {}
    }
    paths
}

fn push_repo_fallback(paths: &mut Vec<PathBuf>, repo_rel: &str) {
    if let Some(path) = resolve_repo_path(repo_rel) {
        if !paths.contains(&path) {
            paths.push(path);
        }
    }
}

pub fn read_file(logical: &str) -> Option<Vec<u8>> {
    for path in read_path_candidates(logical) {
        if path.is_file() {
            if let Ok(data) = fs::read(&path) {
                return Some(data);
            }
        }
    }
    None
}

pub fn write_file(logical: &str, data: &[u8]) -> Result<(), String> {
    let path = resolve_logical_path(logical);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
    }
    fs::write(&path, data).map_err(|e| format!("write {}: {e}", path.display()))?;
    eprintln!("[storage] wrote {} ({} bytes)", path.display(), data.len());
    Ok(())
}

pub fn copy_file(from: &str, to: &str) -> Result<(), String> {
    let data = read_file(from).ok_or_else(|| format!("copy source missing: {from}"))?;
    write_file(to, &data)
}

/// User config: `~/.config/pi-smart-clock/<name>`, then repo `config/`.
pub fn find_config(name: &str, example_name: &str) -> Option<PathBuf> {
    let xdg = xdg_config_dir().join(name);
    if xdg.is_file() {
        return Some(xdg);
    }
    resolve_repo_path(&format!("config/{name}"))
        .or_else(|| resolve_repo_path(&format!("config/{example_name}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_sd_alarms_to_local_share() {
        let path = resolve_logical_path(crate::storage::logical::ALARMS_CSV);
        assert!(path.to_string_lossy().contains(".local/share/pi-smart-clock/config/alarms.csv"));
    }

    #[test]
    fn maps_cache_to_xdg_cache() {
        let path = resolve_logical_path(crate::storage::logical::WEATHER_DATA_CACHE);
        assert!(path.to_string_lossy().contains(".cache/pi-smart-clock/weather_data.json"));
    }

    #[test]
    fn maps_alarm_backup_to_state() {
        let path = resolve_logical_path("/sd/config/alarms_20260101_120000.csv.bak");
        assert!(path.to_string_lossy().contains(".local/state/pi-smart-clock/backups/"));
    }
}