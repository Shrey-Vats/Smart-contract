use anchor_lang::prelude::*;


#[account]
pub struct Escrow {
    pub escrow_id: u64,

    pub to: Pubkey,
    pub from: Pubkey,
    pub mint: Pubkey,
    pub arbiter: Pubkey,

    pub amount: u64,
    pub end_date: i64,

    pub status: EscrowStatus,


    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EscrowStatus {
    Initialized,
    Funded,
    Claimed,
    Refunded,
    Cancelled,
    Disputed
}

impl Escrow {
    pub const INIT_SPACE: usize = 32 + 32 + 8 + 8 + 1;
}