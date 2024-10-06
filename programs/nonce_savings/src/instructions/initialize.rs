use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::Token, token_interface::{Mint,TokenAccount, TokenInterface}
};
use crate::{
    constants::DESCRIMINATOR,
    state::{CounterAccount, SavingsAccount, SavingsState, SavingsType},
    errors::NonceError
};

#[derive(Accounts)]
pub struct InitializeSolSavings<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // Creates Counter for the number of savings the user has
    #[account(
        mut,
        seeds=[b"counter", user.key().as_ref()],
        bump
    )]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(
        init,
        payer=user,
        seeds=[b"savings",user.key().as_ref(),&counter_account.savings_count.to_le_bytes()],
        bump,
        space= DESCRIMINATOR + SavingsAccount::INIT_SPACE
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUSDCSavings<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds=[b"counter", user.key().as_ref()],
        bump=counter_account.bump
    )]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(
        init,
        seeds=[b"savings",user.key().as_ref(),&counter_account.savings_count.to_le_bytes()],
        bump,
        payer = user,
        space= DESCRIMINATOR + SavingsAccount::INIT_SPACE
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    // Mint of token
    #[account(
        mint::token_program = token_program,
        mint::authority = user
    )]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint=usdc_mint,
        associated_token::token_program=token_program,
        associated_token::authority=user
        
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer=user,
        associated_token::mint=usdc_mint,
        associated_token::authority=savings_account,
        associated_token::token_program=token_program
    )]
    pub vault_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializeSolSavings<'info> {
    pub fn initialize(
        &mut self,
        name: String,
        amount: u64,
        duration: i64,
        type_Of_savings: SavingsType,
        usd_price: Option<f64>,
        bump:u8
    ) -> Result<()> {
        self.savings_account.set_inner(SavingsAccount {
            name: name,
            user: self.user.key(),
            sol_balance: 0,
            usdc_balance: 0,
            type_of_savings: type_Of_savings,
            current_time: Clock::get()?.unix_timestamp,
            bump:bump,
            usd_price: usd_price,
            lock_duration: duration,
        });
        self.counter_account.savings_count += 1;
        msg!("Initialized Savings Account");
        Ok(())
    }
}

impl<'info> InitializeUSDCSavings<'info> {
    pub fn initializeusdcsavings(
        &mut self,
        amount: u64,
        name: String,
        duration: i64,
        type_Of_savings: SavingsType,
        usd_price: Option<f64>,
        bump:u8
    ) -> Result<()> {
        self.savings_account.set_inner(SavingsAccount {
            name: name,
            user: self.user.key(),
            sol_balance: 0,
            usdc_balance: 0,
            type_of_savings: type_Of_savings,
            current_time: Clock::get()?.unix_timestamp,
            bump,
            lock_duration: duration,
            usd_price: usd_price,
        });
        self.counter_account.savings_count += 1;
        msg!("Initialized Savings Account");
        Ok(())
    }
}