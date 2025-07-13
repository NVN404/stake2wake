use anchor_lang::prelude::*;

#[event]

pub struct InitializeEvent{
    pub authority: Pubkey, 
    pub treasury: Pubkey, 
    pub bonk_mint: Pubkey, 
    pub bonk_ata: Pubkey, 
    pub total_collected: u64, 
}

#[event]
pub struct StartChallengeEvent {
    pub user: Pubkey, 
    pub user_challenge: Pubkey, 
    pub user_token_account: Pubkey, 
    pub bonk_mint: Pubkey,
    pub vault: Pubkey, 
    pub wakeup_time: u64,
    pub stake_amount: u64, 
    pub total_days: u64, 
}

#[event]
pub struct CheckStatusEvent{
    pub user:Pubkey,
    pub user_challenge:Pubkey,
    pub completed_days: u64,
    pub last_check_time: u64,
    pub is_active: bool,
}