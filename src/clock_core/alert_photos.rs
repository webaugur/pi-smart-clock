use crate::drivers::platform::Platform;
#[cfg(not(feature = "full"))]
use crate::prelude::*;
use crate::storage::logical;

pub struct AlertPhotoManager {
    pub current_photo_path: Option<String>,
}

impl AlertPhotoManager {
    pub fn new() -> Self {
        Self { current_photo_path: None }
    }

    pub async fn fetch_photo<P: Platform>(&mut self, platform: &mut P, alert_id: &str, url: Option<&str>) -> Option<String> {
        if let Some(u) = url {
            if let Some(data) = platform.http_download_binary(u).await {
                let path = logical::alert_photo(alert_id);
                platform.save_photo_as_bmp(&path, &data).await;
                self.current_photo_path = Some(path.clone());
                return Some(path);
            }
        }
        // Fallback to placeholder
        let path = logical::alert_photo(alert_id);
        platform.create_official_placeholder(&path).await;
        self.current_photo_path = Some(path.clone());
        Some(path)
    }
}