use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub bump: u8
}

#[account]
pub struct Allowance {
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub remaining_amount: u64,
    pub bump: u8
}

impl Vault {
    pub fn space() -> u64 {
        32 + 1
    }   
}