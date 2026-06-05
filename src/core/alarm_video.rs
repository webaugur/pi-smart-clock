use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::io::Read;
use std::process::{Child, ChildStdout, Command, Stdio};

use crate::layout::l;
use crate::platform::linux_audio::resolve_media_path;

pub struct AlarmVideoPlayer {
    child: Option<Child>,
    stdout: Option<ChildStdout>,
    frame_buf: Vec<u8>,
    frame_w: u32,
    frame_h: u32,
    playing_path: Option<String>,
}

impl AlarmVideoPlayer {
    pub fn new() -> Self {
        Self {
            child: None,
            stdout: None,
            frame_buf: Vec::new(),
            frame_w: 0,
            frame_h: 0,
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

        let layout = l();
        self.frame_w = layout.center_w;
        self.frame_h = layout.center_h;
        self.frame_buf.resize((self.frame_w * self.frame_h * 3) as usize, 0);

        let mut child = match Command::new("ffmpeg")
            .args([
                "-nostdin",
                "-re",
                "-i",
                resolved.to_string_lossy().as_ref(),
                "-an",
                "-vf",
                &format!("scale={}:{}", self.frame_w, self.frame_h),
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
        if self.frame_buf.is_empty() {
            return;
        }

        match stdout.read_exact(&mut self.frame_buf) {
            Ok(()) => {}
            Err(_) => {
                self.stop();
                return;
            }
        }

        let layout = l();
        let creator = canvas.texture_creator();
        let mut texture = match creator.create_texture(
            PixelFormatEnum::RGB24,
            sdl2::render::TextureAccess::Streaming,
            self.frame_w,
            self.frame_h,
        ) {
            Ok(t) => t,
            Err(_) => return,
        };

        if texture
            .update(None, &self.frame_buf, (self.frame_w * 3) as usize)
            .is_err()
        {
            return;
        }

        let _ = canvas.copy(
            &texture,
            None,
            Rect::new(
                layout.center_x,
                layout.center_y,
                self.frame_w,
                self.frame_h,
            ),
        );
    }
}

impl Drop for AlarmVideoPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}