use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::{ Token}, token_interface::{self, Mint, TokenAccount, TokenInterface, Transfer,transfer_checked,TransferChecked}
   
};
use crate::{
    constants::DESCRIMINATOR,
    errors::NonceError,
    state::{CounterAccount, SavingsAccount, SavingsState, SavingsType},
};
#[derive(Accounts)]
pub struct WithDrawSol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds=[b"counter",user.key().as_ref()],
        bump=counter_account.bump
    )]
    counter_account: Account<'info, CounterAccount>,
    #[account(
        mut,
        seeds=[b"savings",user.key().as_ref(),&counter_account.savings_count.to_le_bytes()],
        bump=savings_account.bump
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawUSDC<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds=[b"counter",user.key().as_ref()],
        bump = counter_account.bump
    )]
    counter_account: Account<'info, CounterAccount>,
    #[account(
        mut,
        seeds=[b"savings",user.key().as_ref(),&counter_account.savings_count.to_le_bytes()],
        bump = savings_account.bump
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    // Mint of token
    #[account(
        // address = usdc_mint.key(),
        mint::token_program = token_program,
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

impl<'info> WithDrawSol<'info> {
    pub fn withdrawsol(&mut self) -> Result<()> {
        // Get the current balance of the savings account
        let balance = self.savings_account.get_lamports();

        let cpi_program = self.system_program.to_account_info();

        let ctx_accounts = anchor_lang::system_program::Transfer {
            from: self.savings_account.to_account_info(),
            to: self.user.to_account_info(),
        };
        let binding = self.user.to_account_info().key();
        let signers_seeds: [&[&[u8]]; 1] = [&[
            b"savings".as_ref(),
            binding.as_ref(),
            &self.counter_account.savings_count.to_le_bytes(),
            &[self.savings_account.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, ctx_accounts, &signers_seeds);

        anchor_lang::system_program::transfer(cpi_ctx, balance)?;

        self.savings_account.sol_balance = 0;

        Ok(())
    }
}

impl<'info> WithdrawUSDC<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        let balance = self.savings_account.usdc_balance;
        if balance == 0 {
            return Err(NonceError::InsufficientFunds.into());
        }
        // Get the current timestamp
        let current_timestamp = Clock::get()?.unix_timestamp;

        // Check if the lock period has elapsed
        if current_timestamp
            < self.savings_account.current_time
                + (self.savings_account.lock_duration as i64) * 86400
        {
            return Err(NonceError::FundsStillLocked.into());
        }

        let cpi_accounts = TransferChecked {
            from: self.vault_account.to_account_info(),
            to: self.user_ata.to_account_info(),
            authority: self.savings_account.to_account_info(),
            mint:self.usdc_mint.to_account_info()
        };
        let binding = self.user.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"savings",
            &binding.as_ref(),
            &self.counter_account.savings_count.to_le_bytes(),
            &[self.savings_account.bump],
        ]];
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
       transfer_checked(ctx,balance,self.usdc_mint.decimals);

        // Set the USDC balance in the savings account to zero after withdrawal
        self.savings_account.usdc_balance = 0;
        Ok(())
    }
}
