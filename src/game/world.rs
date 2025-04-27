use std::collections::HashMap;
use crate::game::player::Player;

pub struct Room {
    pub description: String,
    pub exits: HashMap<String, String>, // e.g., "north" -> "forest"
    pub items: Vec<String>,
}

pub struct World {
    pub rooms: HashMap<String, Room>,
}

impl World {
    pub fn new() -> Self {
        let mut rooms = HashMap::new();

        rooms.insert("village".to_string(), Room {
            description: "You are in a peaceful village. Paths lead north to a forest.".to_string(),
            exits: HashMap::from([("north".to_string(), "forest".to_string())]),
            items: vec!["sword".to_string()],
        });

        rooms.insert("forest".to_string(), Room {
            description: "You are in a dense forest. The village is to the south.".to_string(),
            exits: HashMap::from([("south".to_string(), "village".to_string())]),
            items: vec![],
        });

        Self { rooms }
    }

    pub fn take_item(&mut self, player: &mut Player, item: &str) -> String {
        if let Some(room) = self.rooms.get_mut(&player.location) {
            if let Some(index) = room.items.iter().position(|i| i == item) {
                room.items.remove(index);
                player.inventory.push(item.to_string());
                return format!("You picked up {}.", item);
            }
        }
        "Item not found.".to_string()
    }

    pub fn drop_item(&mut self, player: &mut Player, item: &str) -> String {
        if let Some(index) = player.inventory.iter().position(|i| i == item) {
            player.inventory.remove(index);
            if let Some(room) = self.rooms.get_mut(&player.location) {
                room.items.push(item.to_string());
            }
            return format!("You dropped {}.", item);
        }
        "You don't have that item.".to_string()
    }

    pub fn describe_room(&self, location: &String) -> String {
        match self.rooms.get(location) {
            Some(room) => format!("{}\nItems: {:?}\n", room.description, room.items),
            None => "Unknown location.".to_string(),
        }
    }

    pub fn move_player(&mut self, player: &mut Player, direction: &str) -> String {
        if let Some(room) = self.rooms.get(&player.location) {
            if let Some(new_location) = room.exits.get(direction) {
                player.location = new_location.clone();
                return format!("You move {}.\n{}", direction, self.describe_room(&player.location));
            }
        }
        "You can't go that way.".to_string()
    }
}