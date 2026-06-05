use sdl2::mixer::{Channel, Chunk};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const CHIME_CHANNEL: i32 = 0;
pub const ALARM_CHANNEL: i32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChimeKind {
    Tick,
    Tock,
    Quarter,
    Half,
    Hour,
}

impl ChimeKind {
    pub fn default_path(self) -> &'static str {
        match self {
            ChimeKind::Tick => "sounds/tick.wav",
            ChimeKind::Tock => "sounds/tock.wav",
            ChimeKind::Quarter => "sounds/quarter.wav",
            ChimeKind::Half => "sounds/half.wav",
            ChimeKind::Hour => "sounds/bell.wav",
        }
    }
}

/// Resolve media paths for repo-root or cwd execution.
pub fn resolve_media_path(name: &str) -> Option<PathBuf> {
    if name.is_empty() {
        return None;
    }
    let p = Path::new(name);
    if p.is_absolute() && p.exists() {
        return Some(p.to_path_buf());
    }

    let mut candidates = vec![
        p.to_path_buf(),
        PathBuf::from("sounds").join(p.file_name().unwrap_or(p.as_os_str())),
        PathBuf::from("videos").join(p.file_name().unwrap_or(p.as_os_str())),
    ];
    if !name.contains('/') && !name.contains('\\') {
        candidates.insert(0, PathBuf::from("sounds").join(name));
        candidates.insert(1, PathBuf::from("videos").join(name));
    }

    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for c in candidates {
        if c.exists() {
            return Some(c);
        }
        let m = manifest.join(&c);
        if m.exists() {
            return Some(m);
        }
    }
    None
}

pub struct LinuxAudioEngine {
    chimes: HashMap<ChimeKind, Chunk>,
    missing_logged: HashMap<ChimeKind, bool>,
}

impl LinuxAudioEngine {
    /// Call after `sdl2::mixer::open_audio` in main.
    pub fn new() -> Result<Self, String> {
        let _ = sdl2::mixer::allocate_channels(8);

        let mut chimes = HashMap::new();
        let mut missing_logged = HashMap::new();
        for kind in [
            ChimeKind::Tick,
            ChimeKind::Tock,
            ChimeKind::Quarter,
            ChimeKind::Half,
            ChimeKind::Hour,
        ] {
            if let Some(path) = resolve_media_path(kind.default_path()) {
                match Chunk::from_file(path.to_string_lossy().as_ref()) {
                    Ok(chunk) => {
                        chimes.insert(kind, chunk);
                    }
                    Err(e) => {
                        eprintln!("[audio] failed to load {}: {e}", kind.default_path());
                        missing_logged.insert(kind, true);
                    }
                }
            } else {
                missing_logged.insert(kind, true);
            }
        }

        Ok(Self {
            chimes,
            missing_logged,
        })
    }

    pub fn play_chime(&mut self, kind: ChimeKind) {
        let Some(chunk) = self.chimes.get(&kind) else {
            if !self.missing_logged.get(&kind).copied().unwrap_or(false) {
                eprintln!("[audio] missing chime: {}", kind.default_path());
                self.missing_logged.insert(kind, true);
            }
            return;
        };
        let channel = Channel(CHIME_CHANNEL);
        let _ = channel.play(chunk, 0);
    }

    pub fn play_path_on_channel(&mut self, path: &str, channel_id: i32, loops: i32) -> bool {
        let Some(resolved) = resolve_media_path(path) else {
            eprintln!("[audio] file not found: {path}");
            return false;
        };
        match Chunk::from_file(resolved.to_string_lossy().as_ref()) {
            Ok(chunk) => {
                let channel = Channel(channel_id);
                let _ = channel.play(&chunk, loops);
                true
            }
            Err(e) => {
                eprintln!("[audio] load failed {}: {e}", resolved.display());
                false
            }
        }
    }

    pub fn play_alarm_loop(&mut self, path: &str) {
        self.stop_alarm();
        self.play_path_on_channel(path, ALARM_CHANNEL, -1);
    }

    pub fn stop_alarm(&mut self) {
        Channel(ALARM_CHANNEL).halt();
    }

    pub fn play_one_shot(&mut self, path: &str) {
        self.play_path_on_channel(path, CHIME_CHANNEL, 0);
    }
}