#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BottomSlot {
    Left,
    Mid,
    Right,
}

impl BottomSlot {
    pub const ALL: [BottomSlot; 3] = [Self::Left, Self::Mid, Self::Right];

    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "b_left" | "left" => Some(Self::Left),
            "b_mid" | "mid" | "middle" | "center" | "centre" => Some(Self::Mid),
            "b_right" | "right" => Some(Self::Right),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "b_left",
            Self::Mid => "b_mid",
            Self::Right => "b_right",
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::Left => 0,
            Self::Mid => 1,
            Self::Right => 2,
        }
    }
}