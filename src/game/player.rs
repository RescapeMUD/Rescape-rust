use rand::prelude::*; 

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub class: String,
    pub cha: i32,
    pub con: i32,
    pub dex: i32,
    pub int: i32,
    pub sth: i32,
    pub wis: i32,
    pub location: String,
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new(name: &str, class: &str) -> Self {
        let mut rng = rand::rng(); 

        // Roll four d6, drop the lowest
        fn roll_stat(rng: &mut impl Rng) -> i32 {
            let mut rolls: Vec<i32> = (0..4).map(|_| rng.random_range(1..=3)).collect();
            rolls.sort();
            rolls.iter().skip(1).sum() // Drop lowest roll
        }

        let mut player = Self {
            name: name.to_string(),
            class: class.to_string(),
            cha: roll_stat(&mut rng),
            con: roll_stat(&mut rng),
            dex: roll_stat(&mut rng),
            int: roll_stat(&mut rng),
            sth: roll_stat(&mut rng),
            wis: roll_stat(&mut rng),
            location: "village".to_string(), // Start in the village
            inventory: vec![],
        };

        // Apply class bonuses
        match class {
            "Mage" => { player.int += 4; player.cha += 2; player.con -= 2 }
            "Warrior" => { player.sth += 4; player.con += 2; player.int -= 2 }
            "Thief" => { player.dex += 4; player.cha += 2; player.wis -= 2 }
            "Necromancer" => { player.wis += 4; player.int += 2; player.cha -= 2 }
            _ => {}
        }



        player
    }

    /// Get player stats
    pub fn stats(&self) -> String {
        format!(
            "Name: {}\nClass: {}\nSTR: {}\nDEX: {}\nCON: {}\nINT: {}\nWIS: {}\nCHA: {}",
            self.name, self.class, self.sth, self.dex, self.con, self.int, self.wis, self.cha
        )
    }

    pub fn show_inventory(&self) -> String {
        format!("Inventory: {:?}", self.inventory)
    }

}
