use anchor_lang::prelude::*;

use crate::Wallet;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds=[b"vault", vault.authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Wallet>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[event]
pub struct DepositEvent {
    pub user: Pubkey,
    pub deposit_amount: u64,
}
