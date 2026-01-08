use anchor_lang::prelude::*;

use crate::Vault;

#[derive(Accounts)]
pub struct Invalidate <'info>{

    #[account(
        mut,
        seeds=[b"vault", vault.owner.as_ref()],
        bump=vault.bump,
        has_one=owner
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub owner: Signer<'info>
}