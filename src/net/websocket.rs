use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
//use std::collections::HashMap;
use crate::game::command::Command;
use crate::game::player::Player;
use crate::game::world::World;

// Class identity breakdown for display in character creation
const CLASS_DESCRIPTIONS: &str = r#"
Choose from class:

Class           [Playstyle | Strengths | Weaknesses]
Mage 🧙‍♂️         [Arcane spellcaster  | High burst damage, ranged magic | Low endurance, relies on mana and mastery]
Warrior ⚔️      [Melee combat tank   | Strong attacks, high HP        | Poor magical resistance, relies on strength]
Thief 🏹        [Agile, stealthy      | High evasion, critical hits    | Vulnerable to sustained attacks, relies on speed and subterfuge]
Necromancer ☠️  [Dark magic/summoning | Raises undead, life drain  | Unsettling presence, relies on the undead]

Type the class name:
"#;

// Function to auto-complete class names
fn autocomplete_class(input: &str) -> Option<String> {
    let classes = vec!["Mage", "Warrior", "Thief", "Necromancer"];
    let input_lower = input.to_lowercase();
    
    let matching_classes: Vec<&str> = classes
        .iter()
        .filter(|&class| class.to_lowercase().starts_with(&input_lower))
        .map(|&class| class)
        .collect();

    if matching_classes.len() == 1 {

        Some(matching_classes[0].to_string())
    } else {
        None
    }
}

pub async fn start_websocket(addr: &str) {
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind WebSocket listener");

    println!("WebSocket server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_websocket(stream));
    }
}

async fn handle_websocket(stream: tokio::net::TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket connection failed: {}", e);
            return;
        }
    };

    println!("New WebSocket connection established!");
    let (mut write, mut read) = ws_stream.split();

    let mut player_name = String::new();
    let mut player_class = String::new();
    let mut player: Option<Player> = None;
    let mut world = World::new();

    while let Some(Ok(msg)) = read.next().await {
        if msg.is_text() {
            let received = msg.to_text().unwrap().trim().to_string();

            if player.is_none() {
                // Character Creation: Step 1 - Get Player Name
                if player_name.is_empty() {
                    player_name = received.clone();
                    let response = format!("Hello, {}!\n\n{}", player_name, CLASS_DESCRIPTIONS);
                    write.send(response.into()).await.unwrap();
                    continue;
                }

                // Character Creation: Step 2 - Get Player Class
                if player_class.is_empty() {
                    match autocomplete_class(&received) {
                        Some(valid_class) => {
                            player_class = valid_class;
                            player = Some(Player::new(&player_name, &player_class));
                            let response = format!("Selected: {}\n\nCharacter Created!\n{}", player.as_ref().unwrap().class, player.as_ref().unwrap().stats());
                            write.send(response.into()).await.unwrap();
                        }
                        None => {
                            write.send("Invalid class. Please type the full class name or a valid unique prefix.".into()).await.unwrap();
                        }
                    }
                    // let classes = vec!["Mage", "Warrior", "Thief", "Necromancer"];
                    // if classes.contains(&received.as_str()) {
                    //     player_class = received.clone();
                    //     player = Some(Player::new(&player_name, &player_class));
                    //     let response = format!("Character Created!\n{}", player.as_ref().unwrap().stats());
                    //     write.send(response.into()).await.unwrap();
                    // } else {
                    //     write.send("Invalid class. Choose Mage, Warrior, Thief, or Necromancer.".into()).await.unwrap();
                    // }
                    continue;
                }
            } else {
                // Process Commands After Character Creation
                let command = Command::from_str(&received);

                if let Some(ref mut player) = player {
                    let response = command.execute(player, &mut world); // ✅ Now Player is correctly passed
                    write.send(response.into()).await.unwrap();
                } else {
                    write.send("You must create a character first.".into()).await.unwrap();
                }
            }
        }
    }
}