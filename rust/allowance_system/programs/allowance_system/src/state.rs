use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub nonce: u64,
    pub bump: u8,
}

#[account]
pub struct Allowance {
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub remaining_amount: u64,
    pub expires_at_slot: u64,
    pub nonce: u64,
    pub bump: u8
}