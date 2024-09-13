use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("8JP6GACTPuFPoQzr3Y6Yx9aKtwTUkdZPzAmLjd19vSrS");

#[program]
pub mod nonce_savings {
    use super::*;

    pub fn initialize_savings(
        ctx: Context<InitializeSavings>,
        name: String,
        lock_duration: i64,
        amount: u64,
    ) -> Result<()> {
        require!(name.len() <= 50, NonceError::NameTooLong);

        let savings_account = &mut ctx.accounts.savings_account;
        savings_account.name = name;
        savings_account.owner = ctx.accounts.user.key();
        savings_account.sol_balance = 0;
        savings_account.usdc_balance = 0;
        savings_account.unlock_time = 0;
        savings_account.lock_duration = 0;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeSavings<'info> {
    #[account(
        init,
        payer=user,
        space= DESCRIMAINATOR + SavingsAccount::INIT_SPACE,
        seeds=[b"savings",user.key().as_ref(),name.as_bytes()],
        bump
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DepositSol<'info> {
    #[account(
        mut,
        seeds=[b"sol_savings",user.key().as_ref(),name.as_bytes()],
        bump,
        has_one= owner @ NonceError::Unauthorized,
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DepositUSDC<'info> {
    #[account(
        mut,
        seeds = [b"usdc_savings",user.key().as_ref(),name.as_bytes()],
        bump,
        has_one = owner @ NonceError::Unauthorized
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct WthdrawSol<'info> {
    #[account(
        mut,
        seeds = [b"sol_savings", user.key().as_ref(), savings_account.name.as_bytes()],
        bump,
        has_one = owner @ NonceError::Unauthorized
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct SavingsAccount {
    #[max_len(32)]
    pub name: String,
    pub owner: Pubkey,
    pub sol_balance: u64,
    pub usdc_balance: u64,
    pub unlock_time: i64,
    pub lock_duration: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SavingsStatus {
    Active,
    Unlocked,
}

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

const DESCRIMAINATOR: usize = 8;
