use anchor_lang::prelude::*;


#[account]
pub struct Wallet {
    pub authority: Pubkey,
    pub amount: u64, 
    pub bump: u8
}