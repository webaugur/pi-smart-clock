#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelKind {
    Weather,
    Calendar,
    Holidays,
}

impl PanelKind {
    pub const ALL: [PanelKind; 3] = [
        PanelKind::Weather,
        PanelKind::Calendar,
        PanelKind::Holidays,
    ];

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