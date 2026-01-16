use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::Escrow;

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct Initialize <'info>{

    #[account(
        init,
        space = 8 + Escrow::INIT_SPACE,
        payer = from,
        seeds= [b"escrow", from.key().as_ref(), to.key().as_ref(), escrow_id.to_be_bytes().as_ref()],
        bump,
    )]

    pub escrow_account: Account<'info, Escrow>,

    #[account(mut)]
    pub from: Signer<'info>,

    pub to: UncheckedAccount<'info>,

    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>
}

#[event]
pub struct InitializeEvent {
    pub escrow_id: u64,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub escrow_pda: Pubkey,
    pub mint: Pubkey
}