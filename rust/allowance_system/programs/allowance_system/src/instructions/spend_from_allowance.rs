use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{Allowance, Vault};

#[derive(Accounts)]
pub struct SpendFromAllowance <'info>{

    #[account(
        seeds=[b"allowance", allowance_account.vault.as_ref(), allowance_account.spender.as_ref()],
        bump=allowance_account.bump
    )]
    pub allowance_account: Account<'info, Allowance>,

    #[account(
        mut,
        seeds=[b"vault", vault.owner.as_ref()], 
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub reciver_ata: Account<'info, TokenAccount>,

    pub sender: Signer<'info>,

    pub token_program: Program<'info, Token>
}

#[event]
pub struct SpendFromAllowanceEvent {
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub vault_ata: Pubkey,
    pub reciver_ata: Pubkey,
    pub amount: u64
}