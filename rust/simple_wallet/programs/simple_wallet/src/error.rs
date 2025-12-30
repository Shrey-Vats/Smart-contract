use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("User not found")]
    Unauthorized,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Funds exceed the limit")]
    AmountOverflow,
    #[msg("Invalid amount")]
    InvalidAmount
}
