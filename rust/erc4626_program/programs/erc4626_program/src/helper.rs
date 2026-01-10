use anchor_lang::prelude::*;

use crate::error::ErrorCode;

pub fn convert_to_shares(
    user_assets: u128,
    total_assets: u128,
    total_shares: u128,
) -> Result<u128> {
    let user_share = user_assets
        .checked_mul(total_shares)
        .ok_or(ErrorCode::MathOverflow)?
        / total_assets;

    Ok(user_share)
}

pub fn convert_to_assets(user_share: u128, total_assets: u128, total_shares: u128) -> Result<u128> {
    let user_assets = user_share
        .checked_mul(total_assets)
        .ok_or(ErrorCode::MathOverflow)?
        / total_shares;

    Ok(user_assets)
}
