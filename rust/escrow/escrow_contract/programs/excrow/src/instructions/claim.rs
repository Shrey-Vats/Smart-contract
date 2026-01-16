use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{Escrow, EscrowStatus};

#[derive(Accounts)]
pub struct Claim <'info>{

    #[account(
        mut,
        seeds=[b"escrow", escrow_account.from.as_ref(), escrow_account.to.as_ref()],
        bump=escrow_account.bump,
        close = from,
        has_one = from,
        constraint = escrow_account.status == EscrowStatus::Funded,
        constraint = escrow_account.to == to.key() 
    )]
    pub escrow_account: Account<'info, Escrow>,
    
    #[account(mut)]
    pub from: Signer<'info>,

    #[account(mut)]
    pub to: SystemAccount<'info>,

    #[account(
        mut,
        token::mint = escrow_account.mint,
        token::authority = to.key()
    )]
    pub to_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}

#[event]
pub struct ReleasedEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub end_date: i64,
    pub escrow_pda: Pubkey
}