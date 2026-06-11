mod config;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::modules::bottom_module::{BottomModule, PanelLine};
use crate::modules::module_id::ModuleId;
use crate::panel::Panel;

pub use config::{load_holidays_config_loaded, ConfigMeta, HolidaysConfig, LoadedHolidaysConfig};

#[cfg(feature = "full")]
use chrono::{Datelike, Local};

/// A bottom panel showing upcoming public holidays for a configured country/region.
/// Supports multiple countries for global use (see config/holidays.conf).
pub struct HolidaysPanel {
    config: HolidaysConfig,
    config_meta: ConfigMeta,
    holidays: Vec<String>,
    last_computed_date: Option<String>, // simple key like "2026-06-10" to avoid recompute every tick
}

impl HolidaysPanel {
    pub fn new() -> Self {
        let loaded = load_holidays_config_loaded();
        let config = loaded.config.clone();
        let holidays = Self::compute_upcoming(&config);

        Self {
            config,
            config_meta: loaded.meta,
            holidays,
            last_computed_date: None,
        }
    }

    fn reload_config_if_changed(&mut self) {
        let Some(loaded) = config::reload_holidays_config_if_changed(&self.config_meta) else {
            return;
        };
        eprintln!("[holidays] config changed, recomputing holidays");
        self.config = loaded.config;
        self.config_meta = loaded.meta;
        self.holidays = Self::compute_upcoming(&self.config);
        self.last_computed_date = None;
    }

    /// Recompute the list of holiday strings for the current local date.
    fn refresh_if_needed(&mut self) {
        self.reload_config_if_changed();

        #[cfg(feature = "full")]
        {
            let today = Local::now().date_naive();
            let key = format!("{}", today);
            if self.last_computed_date.as_ref() == Some(&key) {
                return;
            }
            self.holidays = Self::compute_upcoming_for_date(&self.config, today);
            self.last_computed_date = Some(key);
        }

        #[cfg(not(feature = "full"))]
        {
            if self.last_computed_date.is_none() {
                // Embedded: date not yet reliable (see PICO-004). Keep samples or minimal list.
                self.holidays = Self::fallback_samples();
                self.last_computed_date = Some("embedded".to_string());
            }
        }
    }

    fn compute_upcoming(cfg: &HolidaysConfig) -> Vec<String> {
        #[cfg(feature = "full")]
        {
            Self::compute_upcoming_for_date(cfg, Local::now().date_naive())
        }
        #[cfg(not(feature = "full"))]
        {
            Self::fallback_samples()
        }
    }

    #[cfg(feature = "full")]
    fn compute_upcoming_for_date(cfg: &HolidaysConfig, today: chrono::NaiveDate) -> Vec<String> {
        let year = today.year();
        let all = holidays_for_year(year, &cfg.country);
        let mut upcoming: Vec<_> = all
            .into_iter()
            .filter(|(d, _)| *d >= today)
            .collect();
        upcoming.sort_by_key(|(d, _)| *d);
        upcoming
            .into_iter()
            .take(cfg.max_upcoming)
            .map(|(d, name)| format_holiday(d, name))
            .collect()
    }

    #[cfg_attr(feature = "full", allow(dead_code))]
    fn fallback_samples() -> Vec<String> {
        vec![
            "Jun 19 - Juneteenth".to_string(),
            "Jul 4 - Independence Day".to_string(),
            "Sep 1 - Labor Day".to_string(),
        ]
    }
}

fn format_holiday(date: chrono::NaiveDate, name: &str) -> String {
    // e.g. "Dec 25 - Christmas Day"
    let mon = match date.month() {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    };
    format!("{} {} - {}", mon, date.day(), name)
}

/// Returns (date, name) pairs for public holidays in the given year for the country.
/// This is the core that makes the module "work globally".
#[cfg(feature = "full")]
fn holidays_for_year(year: i32, country: &str) -> Vec<(chrono::NaiveDate, &'static str)> {
    use chrono::{NaiveDate, Weekday};

    let mut days = Vec::new();

    let c = country.to_ascii_uppercase();

    // Common / widely observed
    if let Some(d) = NaiveDate::from_ymd_opt(year, 1, 1) {
        days.push((d, "New Year's Day"));
    }
    if let Some(d) = NaiveDate::from_ymd_opt(year, 12, 25) {
        days.push((d, "Christmas Day"));
    }

    match c.as_str() {
        "US" => {
            // Martin Luther King Jr. Day - 3rd Monday of January
            if let Some(d) = nth_weekday(year, 1, 3, Weekday::Mon) {
                days.push((d, "MLK Day"));
            }
            // Presidents' Day - 3rd Monday of February
            if let Some(d) = nth_weekday(year, 2, 3, Weekday::Mon) {
                days.push((d, "Presidents' Day"));
            }
            // Memorial Day - last Monday of May
            if let Some(d) = last_weekday(year, 5, Weekday::Mon) {
                days.push((d, "Memorial Day"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 6, 19) {
                days.push((d, "Juneteenth"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 7, 4) {
                days.push((d, "Independence Day"));
            }
            // Labor Day - 1st Monday of September
            if let Some(d) = nth_weekday(year, 9, 1, Weekday::Mon) {
                days.push((d, "Labor Day"));
            }
            // Columbus Day / Indigenous Peoples' Day - 2nd Monday of October
            if let Some(d) = nth_weekday(year, 10, 2, Weekday::Mon) {
                days.push((d, "Columbus / Indigenous Day"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 11, 11) {
                days.push((d, "Veterans Day"));
            }
            // Thanksgiving - 4th Thursday of November
            if let Some(d) = nth_weekday(year, 11, 4, Weekday::Thu) {
                days.push((d, "Thanksgiving"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 12, 31) {
                days.push((d, "New Year's Eve"));
            }
        }
        "GB" | "UK" => {
            // Good Friday and Easter Monday
            let easter = easter_sunday(year);
            if let Some(gf) = easter.checked_sub_days(chrono::Days::new(2)) {
                days.push((gf, "Good Friday"));
            }
            if let Some(em) = easter.checked_add_days(chrono::Days::new(1)) {
                days.push((em, "Easter Monday"));
            }
            // Early May Bank Holiday - first Monday of May
            if let Some(d) = nth_weekday(year, 5, 1, Weekday::Mon) {
                days.push((d, "Early May Bank Holiday"));
            }
            // Spring Bank Holiday - last Monday of May
            if let Some(d) = last_weekday(year, 5, Weekday::Mon) {
                days.push((d, "Spring Bank Holiday"));
            }
            // Summer Bank Holiday - last Monday of August
            if let Some(d) = last_weekday(year, 8, Weekday::Mon) {
                days.push((d, "Summer Bank Holiday"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 12, 26) {
                days.push((d, "Boxing Day"));
            }
        }
        "CA" => {
            if let Some(d) = NaiveDate::from_ymd_opt(year, 7, 1) {
                days.push((d, "Canada Day"));
            }
            // Civic Holiday / Terry Fox day varies by province; use 1st Mon Aug as common
            if let Some(d) = nth_weekday(year, 8, 1, Weekday::Mon) {
                days.push((d, "Civic Holiday"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 9, 30) {
                days.push((d, "National Day for Truth and Reconciliation"));
            }
            if let Some(d) = nth_weekday(year, 10, 2, Weekday::Mon) {
                days.push((d, "Thanksgiving (CA)"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 11, 11) {
                days.push((d, "Remembrance Day"));
            }
            // Victoria Day - Monday before May 25
            if let Some(d) = victoria_day(year) {
                days.push((d, "Victoria Day"));
            }
        }
        "DE" => {
            if let Some(d) = NaiveDate::from_ymd_opt(year, 10, 3) {
                days.push((d, "German Unity Day"));
            }
            // Good Friday + Easter Monday
            let easter = easter_sunday(year);
            if let Some(gf) = easter.checked_sub_days(chrono::Days::new(2)) {
                days.push((gf, "Karfreitag (Good Friday)"));
            }
            if let Some(em) = easter.checked_add_days(chrono::Days::new(1)) {
                days.push((em, "Ostermontag (Easter Monday)"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 5, 1) {
                days.push((d, "Tag der Arbeit (Labour Day)"));
            }
            // Christmas Eve is widely observed
            if let Some(d) = NaiveDate::from_ymd_opt(year, 12, 24) {
                days.push((d, "Heiligabend (Christmas Eve)"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 12, 26) {
                days.push((d, "2. Weihnachtstag"));
            }
        }
        "FR" => {
            if let Some(d) = NaiveDate::from_ymd_opt(year, 5, 1) {
                days.push((d, "Fête du Travail"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 5, 8) {
                days.push((d, "Victoire 1945"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 7, 14) {
                days.push((d, "Fête Nationale (Bastille Day)"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 11, 11) {
                days.push((d, "Armistice 1918"));
            }
            let easter = easter_sunday(year);
            if let Some(em) = easter.checked_add_days(chrono::Days::new(1)) {
                days.push((em, "Lundi de Pâques"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 8, 15) {
                days.push((d, "Assomption"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 11, 1) {
                days.push((d, "Toussaint"));
            }
        }
        "AU" => {
            if let Some(d) = NaiveDate::from_ymd_opt(year, 1, 26) {
                days.push((d, "Australia Day"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 4, 25) {
                days.push((d, "Anzac Day"));
            }
            // Queen's Birthday / King's Birthday varies by state; use 2nd Mon Jun as common
            if let Some(d) = nth_weekday(year, 6, 2, Weekday::Mon) {
                days.push((d, "King's Birthday"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 12, 26) {
                days.push((d, "Boxing Day"));
            }
        }
        "JP" => {
            if let Some(d) = NaiveDate::from_ymd_opt(year, 2, 11) {
                days.push((d, "建国記念の日 (National Foundation Day)"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 4, 29) {
                days.push((d, "昭和の日 (Shōwa Day)"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 5, 3) {
                days.push((d, "憲法記念日 (Constitution Memorial Day)"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 5, 5) {
                days.push((d, "こどもの日 (Children's Day)"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 11, 3) {
                days.push((d, "文化の日 (Culture Day)"));
            }
            if let Some(d) = NaiveDate::from_ymd_opt(year, 11, 23) {
                days.push((d, "勤労感謝の日 (Labour Thanksgiving)"));
            }
            // Emperor's Birthday (Feb 23 since 2020)
            if let Some(d) = NaiveDate::from_ymd_opt(year, 2, 23) {
                days.push((d, "天皇誕生日 (Emperor's Birthday)"));
            }
        }
        "CN" | "CHINA" => {
            // Chinese cultural holidays (country + culture). Table primary (pre-derived) + lunar backup.
            // Using the small approx helper from the lunar module for demo + a few fixed cultural anchors.
            // Real version will use the full lunar engine + committed mapping table.
            let (m, d) = crate::modules::lunar::approx_chinese_new_year_gregorian(year);
            if let Some(dt) = NaiveDate::from_ymd_opt(year, m, d) {
                days.push((dt, "春节 (Spring Festival / Chinese New Year)"));
            }
            // Lantern Festival (cultural, ~15th of 1st lunar month) – approximate +2 weeks after CNY for demo
            let (m, d) = crate::modules::lunar::approx_chinese_new_year_gregorian(year);
            let mut dd = d + 14;
            let mut mm = m;
            if dd > 28 {
                dd -= 28;
                mm += 1;
            }
            if let Some(dt) = NaiveDate::from_ymd_opt(year, mm, dd) {
                days.push((dt, "元宵 (Lantern Festival)"));
            }
            // Mid-Autumn (cultural) – fixed-ish September anchor for demo (real = 15th 8th lunar)
            if let Some(dt) = NaiveDate::from_ymd_opt(year, 9, 17) {
                days.push((dt, "中秋 (Mid-Autumn Festival)"));
            }
            // Dragon Boat (cultural) – June anchor
            if let Some(dt) = NaiveDate::from_ymd_opt(year, 6, 10) {
                days.push((dt, "端午 (Dragon Boat Festival)"));
            }
        }
        _ => {
            // Unknown country: at least give a few universal ones + Christmas/New Year already added
            if let Some(d) = NaiveDate::from_ymd_opt(year, 5, 1) {
                days.push((d, "International Workers' Day"));
            }
        }
    }

    // Dedup just in case (e.g. Christmas)
    days.sort_by_key(|(d, _)| *d);
    days.dedup_by_key(|(d, _)| *d);
    days
}

/// nth occurrence of a weekday in a month (1-based n).
#[cfg(feature = "full")]
fn nth_weekday(year: i32, month: u32, n: i32, wd: chrono::Weekday) -> Option<chrono::NaiveDate> {
    use chrono::NaiveDate;
    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let first_wd = first.weekday();
    let diff = (wd.num_days_from_monday() as i32 - first_wd.num_days_from_monday() as i32 + 7) % 7;
    let day = 1 + diff + (n - 1) * 7;
    NaiveDate::from_ymd_opt(year, month, day as u32)
}

/// Last occurrence of a weekday in the month.
#[cfg(feature = "full")]
fn last_weekday(year: i32, month: u32, wd: chrono::Weekday) -> Option<chrono::NaiveDate> {
    use chrono::NaiveDate;
    let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    let first_next = NaiveDate::from_ymd_opt(y, m, 1)?;
    let last_of_month = first_next.pred_opt()?;
    let last_wd = last_of_month.weekday();
    let diff = (last_wd.num_days_from_monday() as i32 - wd.num_days_from_monday() as i32 + 7) % 7;
    last_of_month.checked_sub_days(chrono::Days::new(diff as u64))
}

/// Victoria Day (Canada): Monday preceding May 25.
#[cfg(feature = "full")]
fn victoria_day(year: i32) -> Option<chrono::NaiveDate> {
    use chrono::NaiveDate;
    let may25 = NaiveDate::from_ymd_opt(year, 5, 25)?;
    let wd = may25.weekday();
    let days_back = (wd.num_days_from_monday() as i32 - chrono::Weekday::Mon.num_days_from_monday() as i32 + 7) % 7;
    may25.checked_sub_days(chrono::Days::new(days_back as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holidays_panel_computes_real_data() {
        // On full this exercises date-aware computation + formatting.
        let panel = HolidaysPanel::new();
        let (title, _) = panel.title();
        assert!(title.to_lowercase().contains("holiday"), "title should mention holidays: {}", title);

        // Lines are either empty or follow "Mon DD - Name" style from our formatter.
        for line in panel.lines() {
            if !line.text.is_empty() {
                assert!(
                    line.text.contains(" - "),
                    "holiday line should contain ' - ': got '{}'",
                    line.text
                );
            }
        }
    }

    #[cfg(feature = "full")]
    #[test]
    fn us_holidays_include_key_dates() {
        // Spot check a known year.
        let list = holidays_for_year(2026, "US");
        let names: Vec<_> = list.iter().map(|(_, n)| *n).collect();
        assert!(names.iter().any(|n| n.contains("Juneteenth")));
        assert!(names.iter().any(|n| n.contains("Thanksgiving")));
        assert!(names.iter().any(|n| n.contains("Christmas")));
        // MLK is 3rd Mon Jan
        assert!(names.iter().any(|n| n.contains("MLK")));
    }
}

/// Compute Easter Sunday (Gregorian) for the year.
#[cfg(feature = "full")]
fn easter_sunday(year: i32) -> chrono::NaiveDate {
    // Anonymous Gregorian algorithm (widely used, good for 1900-2200+)
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = ((h + l - 7 * m + 114) % 31) + 1;
    chrono::NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap()
}

impl BottomModule for HolidaysPanel {
    fn id(&self) -> ModuleId {
        ModuleId::Holidays
    }

    fn draw_background(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        self.draw(canvas, x, y, w, h);
    }

    fn title(&self) -> (String, u32) {
        let label = match self.config.country.as_str() {
            "US" => "Holidays (US)".to_string(),
            "GB" => "Holidays (UK)".to_string(),
            "CA" => "Holidays (CA)".to_string(),
            "DE" => "Holidays (DE)".to_string(),
            "FR" => "Holidays (FR)".to_string(),
            "AU" => "Holidays (AU)".to_string(),
            "JP" => "Holidays (JP)".to_string(),
            "CN" | "CHINA" => "Holidays (CN)".to_string(),
            other => format!("Holidays ({})", other),
        };
        (label, 0xFFAA88)
    }

    fn lines(&self) -> Vec<PanelLine> {
        self.holidays
            .iter()
            .take(3)
            .map(|h| PanelLine {
                text: h.clone(),
                size_pt: 0,
            })
            .collect()
    }

    fn tick(&mut self, _alerts_active: bool) {
        self.refresh_if_needed();
    }
}

impl Panel for HolidaysPanel {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) {
        canvas.set_draw_color(Color::RGB(17, 17, 17));
        let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
        canvas.set_draw_color(Color::RGB(200, 120, 80));
        let _ = canvas.fill_rect(Rect::new(x + 4, y + 4, (w - 8) as u32, 3));

        let icon_size = ((h - 20).max(80) as u32).min(112);
        let icon_x = x + w - icon_size as i32 - 6;
        let icon_y = y + (h - icon_size as i32) / 2;
        crate::icons::draw_icon(
            canvas,
            "status/starred-symbolic.svg",
            icon_x,
            icon_y,
            icon_size,
        );
    }

    fn update(&mut self) {}
}
