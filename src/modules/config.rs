use std::fs;
use std::path::Path;

use crate::modules::kind::PanelKind;
use crate::platform::linux_audio::resolve_media_path;

const DEFAULT_SLOTS: [PanelKind; 3] = PanelKind::ALL;
const CONFIG_PATHS: [&str; 2] = ["config/panels.conf", "config/panels.conf.example"];

pub fn load_panel_slots() -> [PanelKind; 3] {
    let Some(path) = resolve_panel_config() else {
        eprintln!("[panels] no config found, using default order");
        return DEFAULT_SLOTS;
    };

    match parse_panel_config(&path) {
        Ok(slots) => {
            eprintln!(
                "[panels] loaded {} → [{}, {}, {}]",
                path.display(),
                slots[0].as_str(),
                slots[1].as_str(),
                slots[2].as_str(),
            );
            slots
        }
        Err(e) => {
            eprintln!("[panels] {e}, using default order");
            DEFAULT_SLOTS
        }
    }
}

fn resolve_panel_config() -> Option<std::path::PathBuf> {
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

fn parse_panel_config(path: &Path) -> Result<[PanelKind; 3], String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut slots = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let id = line.split(['#', ',', ';']).next().unwrap_or("").trim();
        if id.is_empty() {
            continue;
        }
        let kind = PanelKind::from_str(id)
            .ok_or_else(|| format!("unknown panel module '{id}' in {}", path.display()))?;
        slots.push(kind);
    }

    if slots.len() < 3 {
        return Err(format!(
            "expected 3 panel slots, found {} in {}",
            slots.len(),
            path.display()
        ));
    }

    Ok([slots[0], slots[1], slots[2]])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_config() {
        let slots = parse_panel_config(Path::new("config/panels.conf.example")).unwrap();
        assert_eq!(slots[0], PanelKind::Weather);
        assert_eq!(slots[1], PanelKind::Calendar);
        assert_eq!(slots[2], PanelKind::Holidays);
    }
}