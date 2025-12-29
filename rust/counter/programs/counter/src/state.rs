use anchor_lang::prelude::*;
// declare the state

#[account]
pub struct Counter {
    pub count: u64,
    pub user: Pubkey,
}
