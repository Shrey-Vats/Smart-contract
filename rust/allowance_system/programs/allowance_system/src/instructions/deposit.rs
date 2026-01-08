use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::Vault;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account[
        mut,
        seeds=[b"vault", vault.owner.as_ref()],
        bump=vault.bump,
        has_one=owner
    ]]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner_ata: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[event]
pub struct DepositEvent {
    pub owner: Pubkey,
    pub vault_ata: Pubkey,
    pub amount: u64
}