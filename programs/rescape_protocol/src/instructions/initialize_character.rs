use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeCharacter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Character::MAX_SIZE,
        seeds = [b"character", authority.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub character: Account<'info, Character>,

    pub system_program: Program<'info, System>,
}
