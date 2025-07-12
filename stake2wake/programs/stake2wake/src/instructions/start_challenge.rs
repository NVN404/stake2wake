use anchor_lang::prelude::*;

use crate::state::UserChallenge;

#[derive(Accounts)]
pub struct StartChallenge<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + UserChallenge::INIT_SPACE, // anchor automatically allocates space 
        seeds = [b"user_challenge", user.key().as_ref()],
        bump
    )]
    pub user_challenge: Account<'info, UserChallenge>,// we are creating account for userchallenge first 
    

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn start_challenge(
    ctx: Context<StartChallenge>,
    wakeup_time: u64,
    stake_amount: u64,
    total_days: u64,
    mint: Pubkey,
    vault: Pubkey,
) -> Result<()> {
    let clock = Clock::get()?;
    let start_time = clock.unix_timestamp as u64;
    let end_time = start_time + total_days * 86400; // 86400 seconds = 1 day

    let user_challenge = &mut ctx.accounts.user_challenge;
    user_challenge.user = ctx.accounts.user.key();
    user_challenge.wakeup_time = wakeup_time;
    user_challenge.stake_amount = stake_amount;
    user_challenge.is_active = true;
    user_challenge.mint = mint;
    user_challenge.vault = vault;
    user_challenge.start_time = start_time;
    user_challenge.end_time = end_time;
    user_challenge.last_check_time = 0;
    user_challenge.completed_days = 0;
    user_challenge.total_days = total_days;

    // Get bump from context
    let (_pda, bump) = Pubkey::find_program_address(
        &[b"user_challenge", ctx.accounts.user.key.as_ref()],
        ctx.program_id,
    );
    user_challenge.bump = bump;

    Ok(())
}
