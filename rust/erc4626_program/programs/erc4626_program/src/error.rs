use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid vault account")]
    InvalidVault,

    #[msg("Insufficint user balance")]
    InsufficientBalance,

    #[msg("Math overflow or underflow")]
    MathOverflow,

    #[msg("Insufficient shares")]
    InsufficientShares,

    #[msg("Invalid vault state")]
    InvalidVaultState,

    #[msg("Vault already initialized")]
    VaultAlreadyInitialized
}
