#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomFlag {
    Safe,
    Hidden,
    Private,
    Trap,
    Locked,
    Objective,
    Shop,
    Home,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZoneFlag {
    Hostile,
    Restricted,
    NoReturn,
    Puzzle,
    Dark,
    Blessed,
    Instance,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
    TurnedIn,
    Failed,
}
