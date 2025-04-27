use crate::game::player::Player;
use crate::game::world::World;

pub enum Command {
    Look,
    Stats,
    CreateCharacter,
    North,
    South,
    East,
    West,
    Up,
    Down,
    Inventory,
    Take(String),
    Drop(String),
    Attack(String),
    Cast(String),
    Unknown(String),
}

impl Command {
    pub fn from_str(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            "look" => Command::Look,
            "north" => Command::North,
            "south" => Command::South,
            "east" => Command::East,
            "west" => Command::West,
            "up" => Command::Up,
            "down" => Command::Down,
            "stats" => Command::Stats,
            "create" => Command::CreateCharacter,
            "inventory" => Command::Inventory,
            _ if input.starts_with("take ") => Command::Take(input[5..].to_string()),
            _ if input.starts_with("drop ") => Command::Drop(input[5..].to_string()),
            _ if input.starts_with("attack ") => Command::Attack(input[7..].to_string()),
            _ if input.starts_with("cast ") => Command::Cast(input[5..].to_string()),
            _ => Command::Unknown(input.to_string()),
        }
    }

    pub fn execute(&self, player: &mut Player, world: &mut World) -> String {
        match self {
            Command::Look => "You see a dimly lit room.".to_string(),
            Command::North => world.move_player(player, "north"),
            Command::South => world.move_player(player, "south"),
            Command::East => world.move_player(player, "east"),
            Command::West => world.move_player(player, "west"),
            Command::Up => world.move_player(player, "up"),
            Command::Down => world.move_player(player, "down"),
            Command::Inventory => player.show_inventory(),
            Command::Take(item) => world.take_item(player, item),
            Command::Drop(item) => world.drop_item(player, item),
            Command::Attack(target) => format!("You attack {}!", target),
            Command::Cast(spell) => format!("You cast {}!", spell),
            Command::Stats => format!(
                "Name: {}\nClass: {}\nSTR: {}\nDEX: {}\nCON: {}\nINT: {}\nWIS: {}\nCHA: {}",
                player.name, player.class, player.sth, player.dex, player.con, player.int, player.wis, player.cha
            ),
            Command::CreateCharacter => "Character creation in progress...".to_string(),
            Command::Unknown(cmd) => format!("Unknown command: {}", cmd),
        }
    }
}