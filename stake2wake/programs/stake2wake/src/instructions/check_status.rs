use anchor_lang::prelude::*;

use crate::state::ChallengeAccount;
use crate::error::Stake2WakeError;

#[derive(Accounts)]
pub struct CheckStatus<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // user who started the challenge

    #[account(
        mut,
        seeds = [b"challenge", user.key().as_ref(), clock.unix_timestamp.to_le_bytes().as_ref()], // using clock here to allow multiple challenges
        bump = user_challenge.bump,
        has_one = user @ Stake2WakeError::Unauthorized // checks who is checking the challenge
    )]
    pub user_challenge: Account<'info, ChallengeAccount>, // user challenge account

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> CheckStatus<'info> {
    pub fn check_status(&mut self) -> Result<()> {
        let now = self.clock.unix_timestamp as u64; // it get's the current time
        let challenge = &mut self.user_challenge;

        require!(challenge.is_active, Stake2WakeError::InactiveChallenge); // checks if the challenge is active or not

        let current_day = (now - challenge.start_time) / 86400; // gets the current day since the challenge started
        let last_checked_day = (challenge.last_check_time - challenge.start_time) / 86400; // it gets when the user checked the challenge last time

        require!(current_day > last_checked_day, Stake2WakeError::AlreadyCheckedInToday); // checks if the user already checked

        let seconds_from_midnight = now % 86400; // days starts from midnight so it gets the time from midnight in seconds
        let extra_time = 15 * 60; // extra time given to the user to check in

        let earliest_time = seconds_from_midnight.saturating_sub(extra_time); // extra time before given wakeup time
        let latest = challenge.wakeup_time + extra_time;// extra time after the wakeup time

        require!(
            seconds_from_midnight >= earliest_time && seconds_from_midnight <= latest,
            Stake2WakeError::MissedWakeupTime
        ); // checks if the user checked in within the allowed time

        challenge.completed_days += 1; // add one day to completed days as the user checked in
        challenge.last_check_time = now; // change the checked in time with latest one 

        if challenge.completed_days >= challenge.total_days {
            challenge.is_active = false;
        } // if the given completed time is done the the is done so is active will go false

        Ok(())
    }
}
