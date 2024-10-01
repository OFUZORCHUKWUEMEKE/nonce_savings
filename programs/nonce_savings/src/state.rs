use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SavingsAccount {
    #[max_len(32)]
    pub name: String, // Name of the savings account
    pub user: Pubkey,                 // User's public key
    pub sol_balance: u64,             // SOL balance in the savings account
    pub usdc_balance: u64,            // USDC balance in the savings account
    pub current_time: i64,            // Timestamp when the savings account was created
    pub lock_duration: u16,           // Lock duration (in days)
    pub usd_price: Option<f64>,       // Optional field to store USD price
    pub bump: u8,                     // Bump seed for PDA (program-derived account)
    pub type_of_savings: SavingsType, // Type of savings (enum)
}

#[account]
pub struct CounterAccount {
    pub counter: Pubkey,
    pub savings_count: u64,
    pub bump: u8,
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq)]
pub enum SavingsState {
    Active,
    InActive,
    Locked,
    Unlocked,
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone, InitSpace)]
pub enum SavingsType {
    TimeLockedSavings,
    ValueLockedSavings,
}
