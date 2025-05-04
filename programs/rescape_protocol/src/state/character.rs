use anchor_lang::prelude::*;

#[account]
pub struct Character {
    pub owner: Pubkey,
    pub name: String,
    pub class: String,
    pub level: u8,
    pub quest_flags: Vec<u8>,
}

impl Character {
    pub const MAX_SIZE: usize = 32 + 4 + 32 + 1 + 4 + 64; // Rough upper bound
}