use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::Vault;


#[derive(Accounts)]
pub struct InitializeVault <'info>{
    #[account(
        init_if_needed,
        payer=sender,
        space= 8 +  32 + 16 + 16 + 1,
        seeds=[b"vault", mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub sender: Signer<'info>,

    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>
}

#[event]
pub struct InitializeVaultEvent {
    pub vault: Pubkey,
    pub sender: Pubkey,
    pub mint: Pubkey,
}