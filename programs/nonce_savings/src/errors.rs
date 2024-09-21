use anchor_lang::prelude::*;

#[error_code]
pub enum NonceError {
    #[msg("Savings account is inactive")]
    SavingsInactive,
    #[msg("Funds are still locked")]
    FundsStillLocked,
    #[msg("Unauthorized access to savings account")]
    Unauthorized,
    #[msg("Name Of Savings to Long")]
    NameTooLong,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
}
