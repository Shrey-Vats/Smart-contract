use anchor_lang::prelude::*;

use crate::Escrow;

#[derive(Accounts)]
pub struct Extract <'info>{
    #[account(
        mut, 
        seeds=[b"escrow", escrow_account.from.as_ref(), escrow_account.to.as_ref()],
        bump=escrow_account.bump,
        close = receiver
    )]
    pub escrow_account: Account<'info, Escrow>,
    pub receiver: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[event]
pub struct ExtractEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub end_date: i64,
    pub escrow_pda: Pubkey
}