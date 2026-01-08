use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("User have insufficent balance to transfer")]
    InsufficentBalance,
    #[msg("Failed to increase value")]
    FailedToIncrease,
    #[msg("Failed to decrease value")]
    FailedToDecrease,
    #[msg("User is not authorized")]
    Unauthorized,
    #[msg("Entered amount is invaild")]
    InvaildAmount,
    #[msg("Mint are different")]
    InvaildMint,
    #[msg("Allowance expired")]
    AllowanceExpired,
    #[msg("Allowance invalid")]
    AllowanceInvalidated
}
