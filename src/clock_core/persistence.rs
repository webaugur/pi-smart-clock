use crate::clock_core::alarm::AlarmManager;
#[cfg(not(feature = "full"))]
use crate::prelude::*;
use crate::storage::logical;

#[cfg(feature = "full")]
use crate::platform::linux_audio::resolve_media_path;
#[cfg(feature = "full")]
use chrono::Local;

fn csv_escape(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        let escaped = field.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        field.to_string()
    }
}

fn normalize_media_path(field: &str, default_sounds_prefix: bool) -> String {
    let trimmed = field.trim().trim_matches('"');
    if trimmed.is_empty() {
        return String::new();
    }

    #[cfg(not(feature = "full"))]
    {
        let _ = default_sounds_prefix;
        return trimmed.to_string();
    }

    #[cfg(feature = "full")]
    {
        if resolve_media_path(trimmed).is_some() {
            return trimmed.to_string();
        }
        if default_sounds_prefix && !trimmed.contains('/') {
            let with_sounds = format!("sounds/{trimmed}");
            if resolve_media_path(&with_sounds).is_some() {
                return with_sounds;
            }
        }
        let with_videos = if trimmed.contains('/') {
            trimmed.to_string()
        } else {
            format!("videos/{trimmed}")
        };
        if resolve_media_path(&with_videos).is_some() {
            return with_videos;
        }
        trimmed.to_string()
    }
}

fn alarm_backup_stamp<P: crate::drivers::platform::Platform>(platform: &P) -> String {
    #[cfg(feature = "full")]
    {
        let _ = platform;
        return Local::now().format("%Y%m%d_%H%M%S").to_string();
    }
    #[cfg(not(feature = "full"))]
    {
        let t = platform.get_current_time();
        return format!(
            "boot{:02}{:02}{:02}",
            t.hour, t.minute, t.second
        );
    }
}

pub async fn save_alarms(platform: &mut impl crate::drivers::platform::Platform, alarms: &AlarmManager) {
    let mut csv = String::from("id,hour,minute,enabled,repeat,label,sound_file,video_file,snooze_minutes\n");

    for (i, alarm) in alarms.alarms.iter().enumerate() {
        if let Some(a) = alarm {
            let escaped_label = csv_escape(&a.label);
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                i,
                a.hour,
                a.minute,
                a.enabled,
                a.repeat,
                escaped_label,
                a.sound_file,
                a.video_file,
                a.snooze_minutes
            ));
        }
    }

    platform
        .write_file(logical::ALARMS_CSV, csv.as_bytes())
        .await;
    platform
        .copy_file(
            logical::ALARMS_CSV,
            &logical::alarms_backup(&alarm_backup_stamp(platform)),
        )
        .await;
}

pub async fn load_alarms(
    platform: &mut impl crate::drivers::platform::Platform,
    alarms: &mut AlarmManager,
) {
    let data = platform.read_file(logical::ALARMS_CSV).await;

    let Some(data) = data else {
        return;
    };

    let content = String::from_utf8_lossy(&data);
    for line in content.lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 5 {
            continue;
        }
        let id: usize = fields[0].parse().unwrap_or(0);
        if id >= 4 {
            continue;
        }
        let sound_raw = fields.get(6).unwrap_or(&"sounds/cuckoo.wav");
        let (video_raw, snooze_field): (&str, &str) = if fields.len() >= 9 {
            (fields[7], fields[8])
        } else if fields.len() == 8 && fields[7].parse::<u32>().is_ok() {
            ("", fields[7])
        } else {
            (fields.get(7).unwrap_or(&""), "9")
        };

        alarms.alarms[id] = Some(crate::clock_core::alarm::Alarm {
            id,
            hour: fields[1].parse().unwrap_or(7),
            minute: fields[2].parse().unwrap_or(0),
            enabled: fields[3].parse().unwrap_or(false),
            repeat: fields[4].parse().unwrap_or(true),
            label: fields
                .get(5)
                .unwrap_or(&"Alarm")
                .trim_matches('"')
                .replace("\"\"", "\""),
            sound_file: normalize_media_path(sound_raw, true),
            video_file: normalize_media_path(video_raw, false),
            snooze_minutes: snooze_field.parse().unwrap_or(9),
        });
    }
}