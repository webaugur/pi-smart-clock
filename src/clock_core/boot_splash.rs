//! Boot splash: MP4 video on capable Linux hosts, PNG/JPEG elsewhere.

pub const SPLASH_MP4: &str = "assets/splash/boot.mp4";
/// Static splash candidates (first match wins): PNG preferred, then JPEG.
pub const SPLASH_IMAGES: &[&str] = &[
    "assets/splash/boot.png",
    "assets/splash/boot.jpg",
    "assets/splash/boot.jpeg",
];
pub const SPLASH_MIN_MS: u64 = 2500;
pub const SPLASH_FRAME_MS: u64 = 33;

#[cfg(feature = "full")]
mod linux {
    use sdl2::pixels::PixelFormatEnum;
    use sdl2::rect::Rect;
    use sdl2::render::Canvas;
    use sdl2::video::Window;
    use std::io::Read;
    use std::process::{Child, ChildStdout, Command, Stdio};

    use std::path::PathBuf;

    use crate::clock_core::boot_splash::{SPLASH_IMAGES, SPLASH_MP4};
    use crate::layout::l;
    use crate::platform::linux_audio::resolve_media_path;

    fn resolve_splash_image() -> Option<PathBuf> {
        SPLASH_IMAGES
            .iter()
            .find_map(|path| resolve_media_path(path))
    }

    enum Mode {
        None,
        Video,
        Image,
    }

    pub struct BootSplash {
        mode: Mode,
        child: Option<Child>,
        stdout: Option<ChildStdout>,
        frame_buf: Vec<u8>,
        frame_w: u32,
        frame_h: u32,
    }

    impl BootSplash {
        pub fn new() -> Self {
            Self {
                mode: Mode::None,
                child: None,
                stdout: None,
                frame_buf: Vec::new(),
                frame_w: 0,
                frame_h: 0,
            }
        }

        pub fn try_start_video(&mut self) -> bool {
            let Some(resolved) = resolve_media_path(SPLASH_MP4) else {
                return false;
            };
            if Command::new("ffmpeg").arg("-version").output().is_err() {
                eprintln!("[boot_splash] ffmpeg not installed — using image fallback");
                return false;
            }

            let layout = l();
            self.frame_w = layout.screen_w as u32;
            self.frame_h = layout.screen_h as u32;
            self.frame_buf.resize((self.frame_w * self.frame_h * 3) as usize, 0);

            // Cover: scale up preserving aspect ratio, then center-crop (no stretch, no letterbox).
            let vf = format!(
                "scale={}:{}:force_original_aspect_ratio=increase,crop={}:{}:(iw-ow)/2:(ih-oh)/2",
                self.frame_w, self.frame_h, self.frame_w, self.frame_h
            );

            // Decode on demand; the boot loop paces frames. `-re` would throttle ffmpeg to
            // wall-clock time while tick/render also sleep, so playback falls behind (~0.5×).
            let mut child = match Command::new("ffmpeg")
                .args([
                    "-nostdin",
                    "-threads",
                    "0",
                    "-i",
                    resolved.to_string_lossy().as_ref(),
                    "-an",
                    "-vf",
                    &vf,
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
                    eprintln!("[boot_splash] ffmpeg spawn failed: {e}");
                    return false;
                }
            };

            let stdout = match child.stdout.take() {
                Some(s) => s,
                None => {
                    let _ = child.kill();
                    return false;
                }
            };

            self.child = Some(child);
            self.stdout = Some(stdout);
            self.mode = Mode::Video;
            eprintln!("[boot_splash] playing {}", resolved.display());
            true
        }

        pub fn try_load_image(&mut self) -> bool {
            let Some(resolved) = resolve_splash_image() else {
                return false;
            };
            let img = match image::open(&resolved) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("[boot_splash] image load failed ({}): {e}", resolved.display());
                    return false;
                }
            };

            let layout = l();
            self.frame_w = layout.screen_w as u32;
            self.frame_h = layout.screen_h as u32;
            let rgb = image::imageops::resize(
                &img.to_rgb8(),
                self.frame_w,
                self.frame_h,
                image::imageops::FilterType::Triangle,
            );
            self.frame_buf = rgb.into_raw();
            self.mode = Mode::Image;
            eprintln!("[boot_splash] showing {}", resolved.display());
            true
        }

        pub fn is_active(&self) -> bool {
            !matches!(self.mode, Mode::None)
        }

        pub fn has_frame(&self) -> bool {
            !self.frame_buf.is_empty() && self.frame_w > 0 && self.frame_h > 0
        }

        pub fn blit(&mut self, canvas: &mut Canvas<Window>) {
            match self.mode {
                Mode::Video => self.blit_video_frame(canvas),
                Mode::Image => self.blit_image_frame(canvas),
                Mode::None => {}
            }
        }

        fn blit_image_frame(&self, canvas: &mut Canvas<Window>) {
            if self.frame_buf.is_empty() {
                return;
            }
            self.blit_buffer(canvas, &self.frame_buf);
        }

        fn blit_video_frame(&mut self, canvas: &mut Canvas<Window>) {
            let Some(stdout) = self.stdout.as_mut() else {
                return;
            };
            match stdout.read_exact(&mut self.frame_buf) {
                Ok(()) => self.blit_buffer(canvas, &self.frame_buf),
                Err(_) => self.freeze_video(),
            }
        }

        /// Keep the last decoded frame visible after the clip ends.
        pub fn freeze_if_video(&mut self) {
            if matches!(self.mode, Mode::Video) {
                self.freeze_video();
            }
        }

        fn freeze_video(&mut self) {
            if let Some(mut child) = self.child.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
            self.stdout = None;
            if self.frame_buf.is_empty() {
                self.mode = Mode::None;
            } else {
                self.mode = Mode::Image;
            }
        }

        fn blit_buffer(&self, canvas: &mut Canvas<Window>, buf: &[u8]) {
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
                .update(None, buf, (self.frame_w * 3) as usize)
                .is_err()
            {
                return;
            }
            let _ = canvas.copy(
                &texture,
                None,
                Rect::new(0, 0, self.frame_w, self.frame_h),
            );
        }

        pub fn stop(&mut self) {
            if let Some(mut child) = self.child.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
            self.stdout = None;
            self.mode = Mode::None;
        }

        /// Checkerboard dissolve — opaque tiles over the clock scene (`progress` 0 = full, 1 = gone).
        pub fn blit_reveal_checkerboard(&self, canvas: &mut Canvas<Window>, progress: f32) {
            use crate::clock_core::boot::reveal::{cell_shows_splash, CHECKER_BLOCK};

            let progress = progress.clamp(0.0, 1.0);
            if progress >= 1.0 || !self.has_frame() {
                return;
            }

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

            let fw = self.frame_w as i32;
            let fh = self.frame_h as i32;
            let cols = (fw + CHECKER_BLOCK - 1) / CHECKER_BLOCK;
            let rows = (fh + CHECKER_BLOCK - 1) / CHECKER_BLOCK;

            for cy in 0..rows {
                for cx in 0..cols {
                    if !cell_shows_splash(cx, cy, progress) {
                        continue;
                    }
                    let x = cx * CHECKER_BLOCK;
                    let y = cy * CHECKER_BLOCK;
                    let w = CHECKER_BLOCK.min(fw - x) as u32;
                    let h = CHECKER_BLOCK.min(fh - y) as u32;
                    let _ = canvas.copy(
                        &texture,
                        Some(Rect::new(x, y, w, h)),
                        Rect::new(x, y, w, h),
                    );
                }
            }
        }
    }

    impl Drop for BootSplash {
        fn drop(&mut self) {
            self.stop();
        }
    }

}

#[cfg(feature = "full")]
pub use linux::BootSplash;