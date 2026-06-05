use std::path::Path;
use std::time::Duration;

use serialport::{SerialPort, SerialPortType};

pub fn open_port(path: &str, baud: u32) -> Result<Box<dyn SerialPort>, String> {
    let port = serialport::new(path, baud)
        .timeout(Duration::from_millis(200))
        .open()
        .map_err(|e| format!("open {path}: {e}"))?;
    Ok(port)
}

pub fn resolve_port(requested: &str) -> Option<String> {
    if !requested.is_empty() && requested != "auto" {
        if Path::new(requested).exists() {
            return Some(requested.to_string());
        }
        eprintln!("[esp8266] configured port {requested} not found, trying auto-detect");
    }
    detect_port()
}

fn port_type_score(port_type: &SerialPortType) -> u8 {
    match port_type {
        SerialPortType::UsbPort(_) => 0,
        SerialPortType::Unknown => 1,
        SerialPortType::PciPort => 2,
        SerialPortType::BluetoothPort => 3,
    }
}

pub fn detect_port() -> Option<String> {
    let ports = serialport::available_ports().ok()?;
    let mut candidates: Vec<(u8, String)> = ports
        .into_iter()
        .map(|info| {
            let name = info.port_name.clone();
            let mut score = port_type_score(&info.port_type);
            if name.contains("ttyUSB") || name.contains("ttyACM") {
                score = score.saturating_sub(1);
            } else if name.contains("ttyAMA") || name.contains("ttyS") {
                score = score.saturating_add(1);
            }
            (score, name)
        })
        .collect();
    candidates.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    candidates.into_iter().map(|(_, name)| name).next()
}

pub fn write_line(port: &mut dyn SerialPort, line: &str) -> Result<(), String> {
    let mut cmd = line.as_bytes().to_vec();
    cmd.push(b'\n');
    port.write_all(&cmd).map_err(|e| e.to_string())?;
    port.flush().map_err(|e| e.to_string())
}

pub fn read_line(port: &mut dyn SerialPort, buf: &mut Vec<u8>) -> Result<Option<String>, String> {
    buf.clear();
    let mut byte = [0u8; 1];
    loop {
        match port.read(&mut byte) {
            Ok(0) => return Ok(None),
            Ok(_) => {
                if byte[0] == b'\n' {
                    while buf.last() == Some(&b'\r') {
                        buf.pop();
                    }
                    return Ok(Some(String::from_utf8_lossy(buf).to_string()));
                }
                if byte[0] == b'\0' {
                    continue;
                }
                buf.push(byte[0]);
                if buf.len() > 8192 {
                    return Err("serial line too long".to_string());
                }
            }
            Err(e)
                if e.kind() == std::io::ErrorKind::TimedOut
                    || e.kind() == std::io::ErrorKind::WouldBlock =>
            {
                if buf.is_empty() {
                    return Ok(None);
                }
                return Ok(Some(String::from_utf8_lossy(buf).to_string()));
            }
            Err(e) => return Err(e.to_string()),
        }
    }
}

pub fn read_exact(port: &mut dyn SerialPort, len: usize) -> Result<Vec<u8>, String> {
    let mut out = vec![0u8; len];
    let mut read = 0;
    while read < len {
        match port.read(&mut out[read..]) {
            Ok(0) => return Err("serial EOF while reading payload".to_string()),
            Ok(n) => read += n,
            Err(e)
                if e.kind() == std::io::ErrorKind::TimedOut
                    || e.kind() == std::io::ErrorKind::WouldBlock =>
            {
                continue;
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(out)
}