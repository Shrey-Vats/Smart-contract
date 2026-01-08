use anchor_lang::prelude::*;

use crate::{Allowance};

#[derive(Accounts)]
pub struct RevokeAllowance <'info>{

    #[account(
        seeds=[b"allowance", allowance_account.vault.as_ref(), allowance_account.spender.as_ref()],
        bump= allowance_account.bump,
        has_one=owner,
    )]
    pub allowance_account: Account<'info, Allowance>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[event]
pub struct RevokeAllowanceEvent {
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub revoke_amount: u64
}