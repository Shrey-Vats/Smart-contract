use anchor_lang::prelude::*;

use crate::Escrow;

#[derive(Accounts)]
pub struct Deposit <'info>{

    #[account(
        mut,
        seeds=[b"escrow", escrow_account.from.as_ref(), escrow_account.to.as_ref()],
        bump=escrow_account.bump,
        has_one= from
    )]
    pub escrow_account: Account<'info, Escrow>,

    #[account(mut)]
    pub from: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[event]
pub struct DepositEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub end_date: i64,
    pub escrow_pda: Pubkey
}