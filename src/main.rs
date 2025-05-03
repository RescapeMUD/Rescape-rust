mod game;
use game::{player::Player, command::Command, world::World};
use warp::Filter;
use warp::ws::{Message, WebSocket};
use futures_util::{StreamExt, SinkExt};

const CLASS_DESCRIPTIONS: &str = r#"
Choose from class:

Class           [Playstyle | Strengths | Weaknesses]
Mage 🧙‍♂️         [Arcane spellcaster  | High burst damage, ranged magic | Low endurance, relies on mana and mastery]
Warrior ⚔️      [Melee combat tank   | Strong attacks, high HP        | Poor magical resistance, relies on strength]
Thief 🏹        [Agile, stealthy      | High evasion, critical hits    | Vulnerable to sustained attacks, relies on speed and subterfuge]
Necromancer ☠️  [Dark magic/summoning | Raises undead, life drain  | Unsettling presence, relies on the undead]

Type the class name:
"#;

fn autocomplete_class(input: &str) -> Option<String> {
    let classes = vec!["Mage", "Warrior", "Thief", "Necromancer"];
    let input_lower = input.to_lowercase();

    let matching: Vec<&str> = classes
        .iter()
        .filter(|&class| class.to_lowercase().starts_with(&input_lower))
        .map(|&class| class)
        .collect();

    if matching.len() == 1 {
        Some(matching[0].to_string())
    } else {
        None
    }
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting Rescape MUD on http://localhost:4000");

    // Serve static files
    let files = warp::fs::dir("./frontend/build");

    // WebSocket route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_websocket)
        });

    // Combine routes
    let routes = ws_route.or(files);

    // Run the server on localhost:4000
    warp::serve(routes).run(([0, 0, 0, 0], 4000)).await;
}

pub async fn handle_websocket(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();

    let mut player_name = String::new();
    let mut player_class = String::new();
    let mut player: Option<Player> = None;
    let mut world = World::new();

    // Initial greeting
    let _ = tx.send(Message::text("Welcome to Rescape MUD.\nEnter your name:")).await;

    while let Some(Ok(msg)) = rx.next().await {
        if let Ok(received) = msg.to_str() {
            let received = received.trim().to_string();

            if received.eq_ignore_ascii_case("exit") {
                let _ = tx.send(Message::text("Goodbye! Disconnecting...")).await;
                break;
            }

            if player.is_none() {
                // Step 1: Get name
                if player_name.is_empty() {
                    player_name = received.clone();
                    let greeting = format!("Hello, {}!\n{}", player_name, CLASS_DESCRIPTIONS);
                    let _ = tx.send(Message::text(greeting)).await;
                    continue;
                }

                // Step 2: Get class with autocomplete
                if player_class.is_empty() {
                    match autocomplete_class(&received) {
                        Some(valid_class) => {
                            player_class = valid_class;
                            player = Some(Player::new(&player_name, &player_class));
                            let response = format!(
                                "Selected: {}\n\nCharacter Created!\n{}",
                                player_class,
                                player.as_ref().unwrap().stats()
                            );
                            let _ = tx.send(Message::text(response)).await;
                        }
                        None => {
                            let _ = tx.send(Message::text("Invalid class. Try typing more letters.")).await;
                        }
                    }
                    continue;
                }
            }

            // Step 3: Handle in-game commands
            if let Some(ref mut player) = player {
                let command = Command::from_str(&received);
                let response = command.execute(player, &mut world);
                let _ = tx.send(Message::text(response)).await;
            } else {
                let _ = tx.send(Message::text("You must create a character first.")).await;
            }
        }
    }
}
