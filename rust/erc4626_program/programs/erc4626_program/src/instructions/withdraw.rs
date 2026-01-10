use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{Position, Vault};


#[derive(Accounts)]
pub struct Withdraw <'info>{
    
    #[account(
        mut,
        seeds=[b"position", position.owner.as_ref(), vault.key().as_ref()],
        bump=position.bump,
        has_one=owner,
    )]

    pub position: Account<'info, Position>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        constraint = owner_ata.mint == vault.mint,
        constraint = owner_ata.owner == owner.key()
    )]
    pub owner_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"vault", vault.mint.as_ref()],
        bump=vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        constraint = vault_ata.mint == vault.mint,
        constraint = vault_ata.owner == vault.key()
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>
}

#[event]
pub struct WithdrawEvent {
    pub position: Pubkey,
    pub vault: Pubkey,
    pub owner: Pubkey,

    pub withdrew_assets: u128,
    pub withdrew_share: u128
}