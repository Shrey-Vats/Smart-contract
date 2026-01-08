use anchor_lang::prelude::*;

use crate::Vault;

#[derive(Accounts)]
pub struct Initialize <'info>{

    #[account(
        init,
        space=8 + 32 + 1,
        payer=owner,
        seeds=[b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}


#[event]
pub struct InitializeEvent {
    pub owner: Pubkey,
    pub vault: Pubkey
}