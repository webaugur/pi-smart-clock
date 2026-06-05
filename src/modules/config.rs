use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::modules::module_id::ModuleId;
use crate::modules::slot::BottomSlot;
use crate::platform::linux_audio::resolve_media_path;

const CONFIG_PATHS: [&str; 2] = ["config/panels.conf", "config/panels.conf.example"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BottomSlotConfig {
    pub left: ModuleId,
    pub mid: ModuleId,
    pub right: ModuleId,
}

impl BottomSlotConfig {
    pub fn default() -> Self {
        Self {
            left: ModuleId::Weather,
            mid: ModuleId::Calendar,
            right: ModuleId::Holidays,
        }
    }

    pub fn module_for(&self, slot: BottomSlot) -> ModuleId {
        match slot {
            BottomSlot::Left => self.left,
            BottomSlot::Mid => self.mid,
            BottomSlot::Right => self.right,
        }
    }
}

pub fn load_bottom_slots() -> BottomSlotConfig {
    let Some(path) = resolve_panel_config() else {
        eprintln!("[panels] no config found, using defaults");
        return BottomSlotConfig::default();
    };

    match parse_panel_config(&path) {
        Ok(slots) => {
            eprintln!(
                "[panels] loaded {} → b_left={}, b_mid={}, b_right={}",
                path.display(),
                slots.left.as_str(),
                slots.mid.as_str(),
                slots.right.as_str(),
            );
            slots
        }
        Err(e) => {
            eprintln!("[panels] {e}, using defaults");
            BottomSlotConfig::default()
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

fn parse_panel_config(path: &Path) -> Result<BottomSlotConfig, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut map: HashMap<BottomSlot, ModuleId> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let slot = BottomSlot::from_str(key.trim()).ok_or_else(|| {
                format!("unknown bottom slot '{}' in {}", key.trim(), path.display())
            })?;
            let module = ModuleId::from_str(value.trim()).ok_or_else(|| {
                format!(
                    "unknown module '{}' for slot {} in {}",
                    value.trim(),
                    slot.as_str(),
                    path.display()
                )
            })?;
            map.insert(slot, module);
            continue;
        }

        return Err(format!(
            "expected slot=module pairs (e.g. b_left=weather) in {}",
            path.display()
        ));
    }

    let mut cfg = BottomSlotConfig::default();
    let mut missing = Vec::new();
    for slot in BottomSlot::ALL {
        if let Some(module) = map.get(&slot) {
            match slot {
                BottomSlot::Left => cfg.left = *module,
                BottomSlot::Mid => cfg.mid = *module,
                BottomSlot::Right => cfg.right = *module,
            }
        } else {
            missing.push(slot.as_str());
        }
    }

    if !missing.is_empty() {
        return Err(format!(
            "missing slot assignment(s): {} in {}",
            missing.join(", "),
            path.display()
        ));
    }

    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_config() {
        let slots = parse_panel_config(Path::new("config/panels.conf.example")).unwrap();
        assert_eq!(slots.left, ModuleId::Weather);
        assert_eq!(slots.mid, ModuleId::Calendar);
        assert_eq!(slots.right, ModuleId::Holidays);
    }

    #[test]
    fn accepts_permuted_modules() {
        let dir = std::env::temp_dir().join("pi-smart-clock-panels-test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("panels.conf");
        std::fs::write(
            &path,
            "b_left=holidays\nb_mid=weather\nb_right=calendar\n",
        )
        .unwrap();
        let slots = parse_panel_config(&path).unwrap();
        assert_eq!(slots.left, ModuleId::Holidays);
        assert_eq!(slots.mid, ModuleId::Weather);
        assert_eq!(slots.right, ModuleId::Calendar);
    }
}