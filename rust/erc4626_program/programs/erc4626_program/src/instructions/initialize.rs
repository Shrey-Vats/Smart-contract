use anchor_lang::prelude::*;

use crate::{Position, Vault};

#[derive(Accounts)]
pub struct Initialize <'info>{

    #[account(
        init,
        space= 8 + 32 + 32 + 16 + 1,
        payer=owner,
        seeds=[b"position", owner.key().as_ref(), vault.key().as_ref()],
        bump
    )]

    pub position: Account<'info, Position>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault", vault.mint.as_ref()],
        bump=vault.bump
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>
}

#[event]
pub struct InitializeEvent {
    pub position: Pubkey,
    pub vault: Pubkey,
    pub owner: Pubkey,
}