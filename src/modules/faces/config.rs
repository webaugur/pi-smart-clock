use std::path::Path;

use super::FaceId;

#[cfg(feature = "full")]
use crate::storage::linux as xdg_storage;

pub fn load_face_id() -> FaceId {
    if let Some(path) = resolve_faces_config_path() {
        if let Ok(id) = parse_faces_config(&path) {
            eprintln!("[faces] loaded {} → {}", path.display(), id.as_str());
            return id;
        }
    }
    eprintln!("[faces] no config found, using default ({})", FaceId::default().as_str());
    FaceId::default()
}

fn resolve_faces_config_path() -> Option<std::path::PathBuf> {
    #[cfg(feature = "full")]
    {
        return xdg_storage::find_config("faces.conf", "faces.conf.example");
    }
    #[cfg(not(feature = "full"))]
    {
        let path = Path::new("config/faces.conf");
        if path.is_file() {
            return Some(path.to_path_buf());
        }
        let example = Path::new("config/faces.conf.example");
        if example.is_file() {
            return Some(example.to_path_buf());
        }
        None
    }
}

fn parse_faces_config(path: &Path) -> Result<FaceId, String> {
    if !path.is_file() {
        return Err(format!("{} not found", path.display()));
    }
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut face = None;
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim().eq_ignore_ascii_case("face") {
            face = FaceId::parse(value.trim());
        }
    }
    face.ok_or_else(|| format!("missing face= in {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_config() {
        let id = parse_faces_config(&resolve_faces_config_path().expect("faces example")).unwrap();
        assert_eq!(id, FaceId::RetroRoman);
    }
}