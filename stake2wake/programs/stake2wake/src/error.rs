use anchor_lang::prelude::*;

#[error_code]
pub enum Stake2WakeError {
    #[msg("Unauthorized only the authority can perform this action")]
    Unauthorized,
    #[msg("The amount of BONK tokens staked is less than the minimum required")]
    InvalidStakeAmount,
    #[msg("The days specified for the challenge must be greater than zero")]
    InvalidTotalDays,
    #[msg("The wakeup time must be in the future")]
    InvalidWakeupTime,
}
