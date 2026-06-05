use crate::core::alarm::AlarmManager;
use chrono::Local;

fn csv_escape(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        let escaped = field.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        field.to_string()
    }
}

pub async fn save_alarms(platform: &mut impl crate::drivers::platform::Platform, alarms: &AlarmManager) {
    let mut csv = String::from("id,hour,minute,enabled,repeat,label,sound_file,snooze_minutes\n");

    for (i, alarm) in alarms.alarms.iter().enumerate() {
        if let Some(a) = alarm {
            let escaped_label = csv_escape(&a.label);
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                i, a.hour, a.minute, a.enabled, a.repeat, escaped_label, a.sound_file, a.snooze_minutes
            ));
        }
    }

    platform.write_file("/sd/config/alarms.csv", csv.as_bytes()).await;
    platform.copy_file("/sd/config/alarms.csv", &format!("/sd/config/alarms_{}.csv.bak", Local::now().format("%Y%m%d_%H%M%S"))).await;
}

pub async fn load_alarms(platform: &mut impl crate::drivers::platform::Platform, alarms: &mut AlarmManager) {
    if let Some(data) = platform.read_file("/sd/config/alarms.csv").await {
        let content = String::from_utf8_lossy(&data);
        for line in content.lines().skip(1) {
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() >= 5 {
                let id: usize = fields[0].parse().unwrap_or(0);
                if id < 4 {
                    alarms.alarms[id] = Some(crate::core::alarm::Alarm {
                        id,
                        hour: fields[1].parse().unwrap_or(7),
                        minute: fields[2].parse().unwrap_or(0),
                        enabled: fields[3].parse().unwrap_or(false),
                        repeat: fields[4].parse().unwrap_or(true),
                        label: fields.get(5).unwrap_or(&"Alarm").trim_matches('"').replace("\"\"", "\"").to_string(),
                        sound_file: fields.get(6).unwrap_or(&"cuckoo.wav").to_string(),
                        snooze_minutes: fields.get(7).unwrap_or(&"9").parse().unwrap_or(9),
                    });
                }
            }
        }
    }
}