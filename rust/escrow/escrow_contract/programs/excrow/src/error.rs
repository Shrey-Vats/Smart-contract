use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Enter the invalid amount")]
    InvalidAmount,
    #[msg("Amount already deposit")]
    AlreadyDeposit,
    #[msg("Enter the invalid time")]
    InvalidTime,
    #[msg("Time has been exceed")]
    TimeExceed,
    #[msg("Only recever can call this function")]
    MisMatchSeller,
    #[msg("Refund time exceed")]
    RefundTimeExceed,
    #[msg("Too early to be extract, wait till given time")]
    TooEarlyToExtract,
    #[msg("Deposite requried")]
    DepositRequried,
    #[msg("Same user don't allowed")]
    InvalidParties,
    #[msg("Mis match signer")]
    MisMatchSigner,
}