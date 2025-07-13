#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod error;
pub mod events;

use instructions::*;
use events::*;

declare_id!("9E5nutqKTvWYDBWWNnH9gGyJLUQLjKaeABosszTPHhnZ");

#[program]
pub mod stake2wake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize_treasury(&ctx.bumps)?;

        emit!(InitializeEvent {
            authority: ctx.accounts.authority.key(),
            treasury: ctx.accounts.treasury.key(),
            bonk_mint: ctx.accounts.bonk_mint.key(),
            bonk_ata: ctx.accounts.bonk_ata.key(),
            total_collected: 0,
        });
        Ok(())
    }

    pub fn start_challenge(
        ctx: Context<StartChallenge>,
        wakeup_time: u64,
        stake_amount: u64,
        total_days: u64
    ) -> Result<()> {
        ctx.accounts.start_challenge(wakeup_time, stake_amount, total_days, &ctx.bumps)?;

        emit!(StartChallengeEvent {
            user: ctx.accounts.user.key(),
            user_challenge: ctx.accounts.user_challenge.key(),
            user_token_account: ctx.accounts.user_token_account.key(),
            bonk_mint: ctx.accounts.bonk_mint.key(),
            vault: ctx.accounts.vault.key(),
            wakeup_time,
            stake_amount,
            total_days,
        });
        Ok(())
    }
    pub fn check_status(ctx: Context<CheckStatus>) -> Result<()> {
        let did_complete = ctx.accounts.check_status()?; // getting the status of the challenge 
        let challenge = &ctx.accounts.user_challenge;

        // bool to find the wether user recieves the bonk or not
        let tokens_returned = did_complete && challenge.completed_days == challenge.total_days;
        let was_failed = !challenge.is_active && !tokens_returned;

        emit!(CheckStatusEvent {
            user: ctx.accounts.user.key(),
            user_challenge: challenge.key(),
            completed_days: challenge.completed_days,
            last_check_time: challenge.last_check_time,
            is_active: challenge.is_active,
            did_complete,
            tokens_returned,
            was_failed,
        });

        Ok(())
    }
}
