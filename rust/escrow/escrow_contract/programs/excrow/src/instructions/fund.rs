use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{Escrow, EscrowStatus};

#[derive(Accounts)]
pub struct Funds <'info>{

    #[account(
        mut,
        seeds=[b"escrow", escrow_account.from.as_ref(), escrow_account.to.as_ref()],
        bump=escrow_account.bump,
        has_one= from,
        constraint = escrow_account.status == EscrowStatus::Initialized
    )]
    pub escrow_account: Account<'info, Escrow>,

    #[account(
        mut,
        token::mint = escrow_account.mint,
        token::authority = escrow_account.key()
    )]
    pub escrow_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub from: Signer<'info>,


    #[account(
        mut,
        token::mint = escrow_account.mint,
        token::authority = from.key()
    )]
    pub from_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}

#[event]
pub struct FundEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub end_date: i64,
    pub escrow_pda: Pubkey
}