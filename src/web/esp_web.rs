use crate::drivers::platform::Platform;

pub struct WebServer;

impl WebServer {
    pub async fn start<P: Platform>(platform: &mut P) {
        // Start ESP8266 AP + HTTP server
        platform.esp8266_start_web_server().await;
    }

    pub async fn handle_request<P: Platform>(&self, platform: &mut P, path: &str) {
        if path == "/config" {
            // Serve configuration page
        }
    }
}