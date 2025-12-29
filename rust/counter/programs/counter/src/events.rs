use anchor_lang::prelude::*;

#[event]
pub struct CounterUpdated {
    pub user: Pubkey,
    pub new_value: u64,
}

#[event]
pub struct CounterInitialize {
    pub user: Pubkey,
    pub value: u64,
}
