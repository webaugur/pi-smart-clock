use std::path::Path;

use super::FaceId;

const CONFIG_PATHS: [&str; 2] = ["config/faces.conf", "config/faces.conf.example"];

pub fn load_face_id() -> FaceId {
    for path in CONFIG_PATHS {
        if let Ok(id) = parse_faces_config(Path::new(path)) {
            eprintln!("[faces] loaded {path} → {}", id.as_str());
            return id;
        }
    }
    eprintln!("[faces] no config found, using default ({})", FaceId::default().as_str());
    FaceId::default()
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
        let id = parse_faces_config(Path::new("config/faces.conf.example")).unwrap();
        assert_eq!(id, FaceId::RetroRoman);
    }
}