use anchor_lang::prelude::*;

use crate::Wallet;

#[derive(Accounts)]
pub struct Initializefn <'info>{
    #[account(
        init,
        payer=user,
        space= 8 + 8 + 32 + 1,
        seeds= [b"vault", user.key().as_ref()],
        bump 
    )]

    pub vault: Account<'info, Wallet>,
    
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[event]
pub struct InitializefnEvent {
    pub user: Pubkey,
    pub amount: u64
}