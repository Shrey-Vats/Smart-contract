use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub mint: Pubkey,
    pub total_shares: u128,
    pub total_assets: u128,
    pub bump: u8
}
#[account]
pub struct Position {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub shares_owned: u128,
    pub bump:u8 
}
