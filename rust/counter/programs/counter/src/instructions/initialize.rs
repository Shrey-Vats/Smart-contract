use anchor_lang::prelude::*;

use crate::state::Counter;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer=user,
        space=8+32+8+8,
        seeds=[
            b"counter",
            user.key().as_ref()
        ],
        bump
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
