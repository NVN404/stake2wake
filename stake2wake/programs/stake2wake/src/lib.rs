use anchor_lang::prelude::*;

declare_id!("9E5nutqKTvWYDBWWNnH9gGyJLUQLjKaeABosszTPHhnZ");

#[program]
pub mod stake2wake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
