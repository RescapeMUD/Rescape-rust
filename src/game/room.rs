use std::collections::HashMap;
use crate::game::flags::RoomFlag;

#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub description: String,
    pub exits: HashMap<String, String>,
    pub items: Vec<String>,
    pub flags: Vec<RoomFlag>,
}
