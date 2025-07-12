use anchor_lang::prelude::*;

#[error_code]
pub enum Stake2WakeError {
    #[msg("Unauthorized only the authority can perform this action")]
    Unauthorized,
}
