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
        ctx.accounts.check_status()?;

        emit!(CheckStatusEvent {
            user: ctx.accounts.user.key(),
            user_challenge: ctx.accounts.user_challenge.key(),
            completed_days: ctx.accounts.user_challenge.completed_days,
            last_check_time: ctx.accounts.user_challenge.last_check_time,
            is_active: ctx.accounts.user_challenge.is_active,
        });
        Ok(())
    }
}
