use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::{Allowance, Vault};

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(
        init,
        payer=owner,
        space= 8 + 32 + 32 + 32 + 32 + 8 + 8 + 8 + 1,
        seeds=[b"allowance", vault.key().as_ref(), spender.key().as_ref(), mint_account.key().as_ref()],
        bump
    )]
    pub allowance_account: Account<'info, Allowance>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub spender: SystemAccount<'info>,

    #[account(
        seeds=[b"vault", vault.owner.as_ref()],
        bump= vault.bump,
        has_one=owner
    )]
    pub vault: Account<'info, Vault>,
    pub vault_ata: Account<'info, TokenAccount>,
    pub mint_account: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct ApproveEvent {
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub allowance_account: Pubkey,
    pub mint_account: Pubkey,
    pub approve_amount: u64,
}
