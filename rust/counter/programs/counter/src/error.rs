use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Counter overflow")]
    Overflow,
    #[msg("Counter Underflow")]
    Underflow,
    #[msg("Unauthorized caller")]
    Unauthorized
}
