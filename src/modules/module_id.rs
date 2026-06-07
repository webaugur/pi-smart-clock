#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ModuleId {
    Weather,
    Calendar,
    Holidays,
    // New upper-row modules (same format, Linux primary)
    Lunar,
    Zodiac,
    Pentagram, // Venus-Moon pentagram cycle visualizer
}

impl ModuleId {
    pub const ALL: [ModuleId; 6] = [
        Self::Weather,
        Self::Calendar,
        Self::Holidays,
        Self::Lunar,
        Self::Zodiac,
        Self::Pentagram,
    ];

    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "weather" => Some(Self::Weather),
            "calendar" => Some(Self::Calendar),
            "holidays" => Some(Self::Holidays),
            "lunar" => Some(Self::Lunar),
            "zodiac" => Some(Self::Zodiac),
            "pentagram" | "venus" | "venus_pentagram" => Some(Self::Pentagram),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Weather => "weather",
            Self::Calendar => "calendar",
            Self::Holidays => "holidays",
            Self::Lunar => "lunar",
            Self::Zodiac => "zodiac",
            Self::Pentagram => "pentagram",
        }
    }
}