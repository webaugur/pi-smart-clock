mod config;
mod serial_link;

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub use config::{load_esp8266_config, Esp8266Config};

#[derive(Debug)]
enum Command {
    Ping,
    WifiConnect { ssid: String, password: String },
    MqttConnect {
        broker: String,
        port: u16,
        user: Option<String>,
        pass: Option<String>,
    },
    MqttPublish {
        topic: String,
        payload: String,
        retain: bool,
    },
    MqttSubscribe { topic: String },
    Ntp { server: String },
    HttpGet { url: String },
    Shutdown,
}

#[derive(Clone)]
pub struct Esp8266Client {
    tx: Sender<Envelope>,
    ready: Arc<Mutex<bool>>,
    _worker: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl Esp8266Client {
    pub fn open(cfg: &Esp8266Config) -> Result<Self, String> {
        if !cfg.enabled {
            return Err("esp8266 disabled in config".to_string());
        }
        let port = serial_link::resolve_port(&cfg.port)
            .ok_or_else(|| "no serial port found (set port= in config/esp8266.conf)".to_string())?;
        let mut serial = serial_link::open_port(&port, cfg.baud)?;
        eprintln!("[esp8266] opened {port} @ {} baud", cfg.baud);

        let (cmd_tx, cmd_rx) = mpsc::channel::<Envelope>();
        let ready = Arc::new(Mutex::new(false));
        let ready_worker = Arc::clone(&ready);

        let worker = thread::Builder::new()
            .name("esp8266-serial".into())
            .spawn(move || worker_loop(&mut *serial, cmd_rx, ready_worker))
            .map_err(|e| e.to_string())?;

        let client = Self {
            tx: cmd_tx,
            ready: Arc::clone(&ready),
            _worker: Arc::new(Mutex::new(Some(worker))),
        };

        client.ping()?;
        *client.ready.lock().expect("ready lock") = true;
        eprintln!("[esp8266] bridge online");

        if !cfg.wifi_ssid.is_empty() {
            match client.wifi_connect(&cfg.wifi_ssid, &cfg.wifi_password) {
                Ok(ip) => eprintln!("[esp8266] WiFi connected ({ip})"),
                Err(e) => eprintln!("[esp8266] WiFi connect failed: {e}"),
            }
        }

        Ok(client)
    }

    pub fn is_ready(&self) -> bool {
        *self.ready.lock().expect("ready lock")
    }

    pub fn ping(&self) -> Result<(), String> {
        self.send(Command::Ping, Duration::from_secs(2))
    }

    pub fn wifi_connect(&self, ssid: &str, password: &str) -> Result<String, String> {
        let resp = self.send_with_response(
            Command::WifiConnect {
                ssid: ssid.to_string(),
                password: password.to_string(),
            },
            Duration::from_secs(30),
        )?;
        let line = response_as_line(resp)?;
        if let Some(ip) = line.strip_prefix("WIFI OK ") {
            return Ok(ip.to_string());
        }
        if line == "OK" {
            return Ok(String::new());
        }
        Err(line)
    }

    pub fn mqtt_connect(
        &self,
        broker: &str,
        port: u16,
        user: Option<&str>,
        pass: Option<&str>,
    ) -> Result<(), String> {
        self.send_ok(
            Command::MqttConnect {
                broker: broker.to_string(),
                port,
                user: user.map(str::to_string),
                pass: pass.map(str::to_string),
            },
            Duration::from_secs(15),
        )
    }

    pub fn mqtt_publish(&self, topic: &str, payload: &str, retain: bool) -> Result<(), String> {
        self.send_ok(
            Command::MqttPublish {
                topic: topic.to_string(),
                payload: payload.to_string(),
                retain,
            },
            Duration::from_secs(5),
        )
    }

    pub fn mqtt_subscribe(&self, topic: &str) -> Result<(), String> {
        self.send_ok(
            Command::MqttSubscribe {
                topic: topic.to_string(),
            },
            Duration::from_secs(5),
        )
    }

    pub fn ntp(&self, server: &str) -> Result<String, String> {
        let resp = self.send_with_response(
            Command::Ntp {
                server: server.to_string(),
            },
            Duration::from_secs(15),
        )?;
        let line = response_as_line(resp)?;
        if let Some(ts) = line.strip_prefix("NTP OK ") {
            let ts = ts.trim();
            if let Ok(epoch) = ts.parse::<i64>() {
                if let Some(dt) = chrono::DateTime::from_timestamp(epoch, 0) {
                    return Ok(dt.to_rfc3339());
                }
            }
            return Ok(ts.to_string());
        }
        Err(line)
    }

    pub fn http_get(&self, url: &str) -> Result<Vec<u8>, String> {
        match self.send_with_response(
            Command::HttpGet {
                url: url.to_string(),
            },
            Duration::from_secs(60),
        )? {
            Response::Line(line) => {
                if let Some(rest) = line.strip_prefix("HTTP OK ") {
                    let mut parts = rest.splitn(2, ' ');
                    let status = parts.next().unwrap_or("0");
                    let len = parts
                        .next()
                        .unwrap_or("0")
                        .parse::<usize>()
                        .map_err(|_| format!("invalid HTTP length in {line}"))?;
                    if status != "200" {
                        return Err(format!("HTTP status {status}"));
                    }
                    return Err(format!("expected binary HTTP payload ({len} bytes)"));
                }
                Err(line)
            }
            Response::Binary(data) => Ok(data),
        }
    }

    fn send_ok(&self, cmd: Command, timeout: Duration) -> Result<(), String> {
        let resp = self.send_with_response(cmd, timeout)?;
        match resp {
            Response::Line(line) if line == "OK" || line.starts_with("MQTT OK") => Ok(()),
            Response::Line(line) => Err(line),
            Response::Binary(_) => Err("unexpected binary response".to_string()),
        }
    }

    fn send(&self, cmd: Command, timeout: Duration) -> Result<(), String> {
        match self.send_with_response(cmd, timeout)? {
            Response::Line(line) if line == "PONG" || line == "OK" => Ok(()),
            Response::Line(line) if line.starts_with("ERR ") => Err(line),
            Response::Line(line) => Err(line),
            Response::Binary(_) => Err("unexpected binary response".to_string()),
        }
    }

    fn send_with_response(&self, cmd: Command, timeout: Duration) -> Result<Response, String> {
        let (reply_tx, reply_rx) = mpsc::channel();
        self.tx
            .send(Envelope { cmd, reply: reply_tx })
            .map_err(|_| "esp8266 worker stopped".to_string())?;
        match reply_rx.recv_timeout(timeout) {
            Ok(Ok(resp)) => Ok(resp),
            Ok(Err(e)) => Err(e),
            Err(mpsc::RecvTimeoutError::Timeout) => Err("esp8266 command timeout".to_string()),
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                Err("esp8266 worker disconnected".to_string())
            }
        }
    }
}

impl Drop for Esp8266Client {
    fn drop(&mut self) {
        let _ = self.tx.send(Envelope {
            cmd: Command::Shutdown,
            reply: mpsc::channel().0,
        });
        if let Some(handle) = self._worker.lock().expect("worker lock").take() {
            let _ = handle.join();
        }
    }
}

#[derive(Debug)]
enum Response {
    Line(String),
    Binary(Vec<u8>),
}

struct Envelope {
    cmd: Command,
    reply: Sender<Result<Response, String>>,
}

fn worker_loop(port: &mut dyn serialport::SerialPort, rx: Receiver<Envelope>, ready: Arc<Mutex<bool>>) {
    let mut line_buf = Vec::with_capacity(256);
    while let Ok(env) = rx.recv() {
        if matches!(env.cmd, Command::Shutdown) {
            let _ = env.reply.send(Ok(Response::Line("OK".to_string())));
            *ready.lock().expect("ready lock") = false;
            break;
        }
        let result = dispatch_command(port, &mut line_buf, env.cmd);
        let _ = env.reply.send(result);
    }
}

fn response_as_line(resp: Response) -> Result<String, String> {
    match resp {
        Response::Line(line) => Ok(line),
        Response::Binary(_) => Err("unexpected binary response".to_string()),
    }
}

fn dispatch_command(
    port: &mut dyn serialport::SerialPort,
    line_buf: &mut Vec<u8>,
    cmd: Command,
) -> Result<Response, String> {
    match cmd {
        Command::Ping => {
            serial_link::write_line(port, "PING")?;
            read_expected_line(port, line_buf, "PONG")
        }
        Command::WifiConnect { ssid, password } => {
            serial_link::write_line(port, &format!("WIFI {ssid}\t{password}"))?;
            read_ok_line(port, line_buf)
        }
        Command::MqttConnect {
            broker,
            port: mqtt_port,
            user,
            pass,
        } => {
            let creds = match (user, pass) {
                (Some(u), Some(p)) => format!(" {u}\t{p}"),
                _ => String::new(),
            };
            serial_link::write_line(port, &format!("MQTT_CONN {broker} {mqtt_port}{creds}"))?;
            read_ok_line(port, line_buf)
        }
        Command::MqttPublish {
            topic,
            payload,
            retain,
        } => {
            let retain_flag = if retain { 1 } else { 0 };
            let bytes = payload.as_bytes();
            serial_link::write_line(port, &format!("MQTT_PUB {topic} {retain_flag} {}", bytes.len()))?;
            port.write_all(bytes).map_err(|e| e.to_string())?;
            port.flush().map_err(|e| e.to_string())?;
            read_ok_line(port, line_buf)
        }
        Command::MqttSubscribe { topic } => {
            serial_link::write_line(port, &format!("MQTT_SUB {topic}"))?;
            read_ok_line(port, line_buf)
        }
        Command::Ntp { server } => {
            serial_link::write_line(port, &format!("NTP {server}"))?;
            read_ok_line(port, line_buf)
        }
        Command::HttpGet { url } => {
            serial_link::write_line(port, &format!("HTTP_GET {url}"))?;
            let line = read_response_line(port, line_buf)?;
            if let Some(rest) = line.strip_prefix("HTTP OK ") {
                let mut parts = rest.splitn(2, ' ');
                let _status = parts.next().unwrap_or("0");
                let len = parts
                    .next()
                    .unwrap_or("0")
                    .parse::<usize>()
                    .map_err(|_| format!("invalid HTTP header: {line}"))?;
                let data = serial_link::read_exact(port, len)?;
                return Ok(Response::Binary(data));
            }
            Ok(Response::Line(line))
        }
        Command::Shutdown => Err("internal command routing error".to_string()),
    }
}

fn read_ok_line(port: &mut dyn serialport::SerialPort, line_buf: &mut Vec<u8>) -> Result<Response, String> {
    let line = read_response_line(port, line_buf)?;
    if line.starts_with("ERR ") {
        return Err(line);
    }
    Ok(Response::Line(line))
}

fn read_expected_line(
    port: &mut dyn serialport::SerialPort,
    line_buf: &mut Vec<u8>,
    expected: &str,
) -> Result<Response, String> {
    let line = read_response_line(port, line_buf)?;
    if line == expected {
        Ok(Response::Line(line))
    } else if line.starts_with("ERR ") {
        Err(line)
    } else {
        Err(format!("expected {expected}, got {line}"))
    }
}

fn read_response_line(
    port: &mut dyn serialport::SerialPort,
    line_buf: &mut Vec<u8>,
) -> Result<String, String> {
    let deadline = std::time::Instant::now() + Duration::from_secs(60);
    while std::time::Instant::now() < deadline {
        if let Some(line) = serial_link::read_line(port, line_buf)? {
            if line.starts_with("LOG ") {
                eprintln!("[esp8266] {}", &line[4..]);
                continue;
            }
            if line.starts_with("MQTT_MSG ") {
                eprintln!("[esp8266] {}", line);
                continue;
            }
            return Ok(line);
        }
        thread::sleep(Duration::from_millis(5));
    }
    Err("esp8266 response timeout".to_string())
}