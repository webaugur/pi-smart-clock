#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiMode {
    Boot,
    Clock,
    Menu,
    TimeSet,
    Alarm,
    About,
}
