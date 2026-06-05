#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ModuleId {
    Weather,
    Calendar,
    Holidays,
}

impl ModuleId {
    pub const ALL: [ModuleId; 3] = [Self::Weather, Self::Calendar, Self::Holidays];

    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "weather" => Some(Self::Weather),
            "calendar" => Some(Self::Calendar),
            "holidays" => Some(Self::Holidays),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Weather => "weather",
            Self::Calendar => "calendar",
            Self::Holidays => "holidays",
        }
    }
}