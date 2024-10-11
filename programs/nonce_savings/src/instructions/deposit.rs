use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::{self, Token}, token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked}
};
use crate::{
    constants::DESCRIMINATOR,
    state::{CounterAccount, SavingsAccount, SavingsState, SavingsType},
};

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"savings",
            user.key().as_ref(),
            &savings_account.random_seed.as_ref()
        ],
        bump = savings_account.bump,
        constraint = savings_account.user == user.key() // Verify ownership
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositUSDC<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"savings",
            user.key().as_ref(),
            &savings_account.random_seed.as_ref()
        ],
        bump = savings_account.bump,
        constraint = savings_account.user == user.key() // Verify ownership
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint=usdc_mint,
        associated_token::token_program=token_program,
        associated_token::authority=user
        
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint=usdc_mint,
        associated_token::authority=savings_account,
        associated_token::token_program=token_program
    )]
    pub vault_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> DepositSol<'info> {
    pub fn deposit_sol(&mut self, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: self.user.to_account_info(),
                to: self.savings_account.to_account_info(),
            },
        );

        anchor_lang::system_program::transfer(cpi_context, amount)?;

        // self.counter_account.savings_count += 1;
        self.savings_account.sol_balance += amount;
        msg!("Saving deposited");
      
        Ok(())
    }
}

impl<'info> DepositUSDC<'info> {
    pub fn deposit_usdc(&mut self, amount: u64) -> Result<()> {
   

        let cpi_accounts = TransferChecked {
                from: self.user_ata.to_account_info(),
                to: self.vault_account.to_account_info(),
                authority: self.user.to_account_info(),
                mint:self.usdc_mint.to_account_info()
        };

        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            cpi_accounts,
        );

        anchor_spl::token_interface::transfer_checked(cpi_ctx, amount,6)?;

        self.savings_account.usdc_balance += amount;

        msg!("Deposited USDC Successfully");

        Ok(())
    }
}
