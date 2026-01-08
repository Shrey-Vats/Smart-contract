use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";
pub const SLOTS_PER_DAY: u64 = 216_000;
pub const ALLOWANCE_VALIDITY_SLOTS: u64 = 7 * SLOTS_PER_DAY;