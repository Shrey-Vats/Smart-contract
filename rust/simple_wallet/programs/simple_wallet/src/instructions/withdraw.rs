use anchor_lang::prelude::*;

use crate::Wallet;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds=[b"vault", vault.authority.as_ref()],
        bump
    )]
    pub vault: Account<'info, Wallet>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct WithdrawEvent {
    pub user: Pubkey,
    pub withdraw_amount: u64,
}
