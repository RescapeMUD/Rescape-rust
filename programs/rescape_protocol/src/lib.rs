use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

declare_id!("D22c9XRYbzh7XGPGciRVrbgrtQKBFTMK3SMPBqLmdmZv");

#[program]
pub mod rescape_protocol {
    use super::*;

    pub fn initialize_character(
        ctx: Context<InitializeCharacter>,
        name: String,
        class: String,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.owner = ctx.accounts.authority.key();
        character.name = name;
        character.class = class;
        character.level = 1;
        character.quest_flags = vec![];
        Ok(())
    }
}