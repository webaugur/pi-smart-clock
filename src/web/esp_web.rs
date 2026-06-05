use crate::drivers::platform::Platform;

/// ESP8266 Web Server with mobile-friendly UI
pub struct WebServer {
    pub enabled: bool,
}

impl WebServer {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub async fn handle_request<P: Platform>(&self, _platform: &mut P, path: &str) -> String {
        match path {
            "/" => self.home_page(),
            "/alarms" => self.alarms_page(),
            "/config" => self.config_page(),
            _ => self.not_found(),
        }
    }

    fn home_page(&self) -> String {
        r#"
        <!DOCTYPE html>
        <html><head><title>Smart Clock</title>
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>body { background: #111; color: #eee; font-family: sans-serif; padding: 20px; }
        a { color: #0af; text-decoration: none; }
        .card { background: #222; padding: 15px; margin: 10px 0; border-radius: 8px; }
        </style></head>
        <body>
        <h1>🕒 Smart Clock</h1>
        <div class="card">
          <p>Status: Running on Pico DVI</p>
          <a href="/alarms">Manage Alarms</a><br>
          <a href="/config">Settings & SAME Codes</a>
        </div>
        </body></html>
        "#.to_string()
    }

    fn alarms_page(&self) -> String {
        "<h1>Alarms</h1><p>Alarm management coming soon...</p>".to_string()
    }

    fn config_page(&self) -> String {
        "<h1>Configuration</h1><p>Clock speed, brightness, SAME codes, night mode...</p>".to_string()
    }

    fn not_found(&self) -> String {
        "404 - Page not found".to_string()
    }
}