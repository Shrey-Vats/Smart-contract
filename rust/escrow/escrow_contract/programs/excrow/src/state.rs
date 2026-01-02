use anchor_lang::prelude::*;


#[account]
pub struct Escrow {
    pub to: Pubkey,
    pub from: Pubkey,
    pub amount: u64,
    pub end_date: i64,
    pub bump: u8
}

impl Escrow {
    pub const INIT_SPACE: usize = 32 + 32 + 8 + 8 + 1;
}