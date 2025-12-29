use anchor_lang::prelude::*;

use crate::state::Counter;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        seeds=[
            b"counter",
            user.key().as_ref()
        ],
        bump
    )]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}
