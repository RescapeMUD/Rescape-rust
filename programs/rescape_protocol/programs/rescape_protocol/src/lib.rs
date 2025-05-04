use anchor_lang::prelude::*;

declare_id!("GT5uuiNgJetZZ6ZuQKiYhX7CQR8NymCSeg8Pmzfq1bys");

#[program]
pub mod rescape_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
