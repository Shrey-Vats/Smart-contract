use anchor_lang::prelude::*;

use crate::Escrow;

#[derive(Accounts)]
pub struct Initialize <'info>{

    #[account(
        init,
        space = 8 + Escrow::INIT_SPACE,
        payer = from,
        seeds= [b"escrow", from.key().as_ref(), to.key().as_ref()],
        bump,
    )]

    pub escrow_account: Account<'info, Escrow>,

    #[account(mut)]
    pub from: Signer<'info>,

    /// CHECK: Used only for its public key as a PDA seed.
    pub to: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>
}

#[event]
pub struct InitializeEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub escrow_pda: Pubkey
}