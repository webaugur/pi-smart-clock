use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::io::Read;
use std::process::{Child, ChildStdout, Command, Stdio};

use crate::platform::linux_audio::resolve_media_path;

pub const CENTER_X: i32 = 267;
pub const CENTER_Y: i32 = 140;
pub const CENTER_W: u32 = 266;
pub const CENTER_H: u32 = 160;
const FRAME_BYTES: usize = (CENTER_W as usize) * (CENTER_H as usize) * 3;

pub struct AlarmVideoPlayer {
    child: Option<Child>,
    stdout: Option<ChildStdout>,
    frame_buf: Vec<u8>,
    playing_path: Option<String>,
}

impl AlarmVideoPlayer {
    pub fn new() -> Self {
        Self {
            child: None,
            stdout: None,
            frame_buf: vec![0; FRAME_BYTES],
            playing_path: None,
        }
    }

    pub fn is_playing(&self) -> bool {
        self.child.is_some()
    }

    pub fn start(&mut self, path: &str) {
        let resolved = match resolve_media_path(path) {
            Some(p) => p,
            None => {
                eprintln!("[alarm_video] not found: {path}");
                return;
            }
        };
        if self.playing_path.as_deref() == Some(path) && self.child.is_some() {
            return;
        }
        self.stop();

        if Command::new("ffmpeg").arg("-version").output().is_err() {
            eprintln!("[alarm_video] ffmpeg not installed");
            return;
        }

        let mut child = match Command::new("ffmpeg")
            .args([
                "-nostdin",
                "-re",
                "-i",
                resolved.to_string_lossy().as_ref(),
                "-an",
                "-vf",
                &format!("scale={CENTER_W}:{CENTER_H}"),
                "-f",
                "rawvideo",
                "-pix_fmt",
                "rgb24",
                "-",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[alarm_video] spawn failed: {e}");
                return;
            }
        };

        let stdout = match child.stdout.take() {
            Some(s) => s,
            None => {
                eprintln!("[alarm_video] no stdout from ffmpeg");
                let _ = child.kill();
                return;
            }
        };
        self.child = Some(child);
        self.stdout = Some(stdout);
        self.playing_path = Some(path.to_string());
        eprintln!("[alarm_video] playing {}", resolved.display());
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.stdout = None;
        self.playing_path = None;
    }

    pub fn poll_frame(&mut self, canvas: &mut Canvas<Window>) {
        let Some(stdout) = self.stdout.as_mut() else {
            return;
        };

        match stdout.read_exact(&mut self.frame_buf) {
            Ok(()) => {}
            Err(_) => {
                self.stop();
                return;
            }
        }

        let creator = canvas.texture_creator();
        let mut texture = match creator.create_texture(
            PixelFormatEnum::RGB24,
            sdl2::render::TextureAccess::Streaming,
            CENTER_W,
            CENTER_H,
        ) {
            Ok(t) => t,
            Err(_) => return,
        };

        if texture
            .update(None, &self.frame_buf, (CENTER_W * 3) as usize)
            .is_err()
        {
            return;
        }

        let _ = canvas.copy(
            &texture,
            None,
            Rect::new(CENTER_X, CENTER_Y, CENTER_W, CENTER_H),
        );
    }
}

impl Drop for AlarmVideoPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}