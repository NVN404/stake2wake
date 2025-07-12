use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::state::UserChallenge;

#[derive(Accounts)]
pub struct StartChallenge<'info> {
    // Creates a new PDA account to store user challenge data
    #[account(
        init,
        payer = user,
        space = 8 + UserChallenge::INIT_SPACE, // 8 bytes for discriminator + struct size
        seeds = [b"user_challenge", user.key().as_ref()],
        bump
    )]
    pub user_challenge: Account<'info, UserChallenge>,

    // The user who is starting the challenge pays rent and signs tx
    #[account(mut)]
    pub user: Signer<'info>,

    // The user's token account (ata) from which stake will be deducted
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    // The vault where the stake will be stored (
    #[account(
        mut,
        constraint = vault.mint == user_token_account.mint,    //must have same mint as user_token_account

    )]
    pub vault: Account<'info, TokenAccount>,

    // SPL Token program for performing token transfer
    pub token_program: Program<'info, Token>,

    // Solana system program used for creating accounts
    pub system_program: Program<'info, System>,

    // Clock sysvar to get current onchain timestamp
    pub clock: Sysvar<'info, Clock>,
}

pub fn start_challenge(
    ctx: Context<StartChallenge>,
    wakeup_time: u64,  // When the user wants to wake up
    stake_amount: u64, // Amount user is staking
    total_days: u64,   // Number of days challenge lasts
) -> Result<()> {
    let clock = Clock::get()?; // Get current Unix timestamp
    let start_time = clock.unix_timestamp as u64;
    let end_time = start_time + total_days * 86400; // End time = start + N days

    // Initialize the challenge account with all details
    let challenge = &mut ctx.accounts.user_challenge;
    challenge.user = ctx.accounts.user.key();
    challenge.stake_amount = stake_amount;
    challenge.is_active = true;
    challenge.mint = ctx.accounts.user_token_account.mint;
    challenge.vault = ctx.accounts.vault.key();
    challenge.start_time = start_time;
    challenge.end_time = end_time;
    challenge.last_check_time = 0;
    challenge.completed_days = 0;
    challenge.total_days = total_days;

    let (_pda, bump) = Pubkey::find_program_address(
        &[b"user_challenge", ctx.accounts.user.key.as_ref()],
        ctx.program_id,
    );
    challenge.bump = bump;

    // Transfer full stake to vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    transfer(cpi_ctx, stake_amount)?;

    Ok(())
}
