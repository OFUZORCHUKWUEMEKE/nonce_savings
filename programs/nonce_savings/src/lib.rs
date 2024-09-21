pub mod errors;
use anchor_lang::prelude::*;
// use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};


declare_id!("8JP6GACTPuFPoQzr3Y6Yx9aKtwTUkdZPzAmLjd19vSrS");

#[program]
pub mod nonce_savings {

    use super::*;

    pub fn initialize_savings(
        ctx: Context<InitializeSavings>,
        name: String,
        lock_duration: i64,
        _amount: u64,
    ) -> Result<()> {
        require!(name.len() <= 50, NonceError::NameTooLong);

        let savings_account = &mut ctx.accounts.savings_account;
        savings_account.name = name;
        savings_account.user = ctx.accounts.user.key();
        savings_account.sol_balance = 0;
        savings_account.usdc_balance = 0;
        // savings_account

        let clock = Clock::get()?;
        savings_account.lock_duration = lock_duration as u64;
        savings_account.unlock_time = clock.unix_timestamp + lock_duration;

        Ok(())
    }

    pub fn initialize_program_account(_ctx: Context<InitializeProgramAccount>) -> Result<()> {
        msg!("Program token account initialized");
        Ok(())
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64, lock_duration: u64) -> Result<()> {
        let savings_account = &mut ctx.accounts.savings_account;

        let user = &mut ctx.accounts.user;

        require!(
            lock_duration >= 1 && lock_duration <= 365,
            NonceError::FundsStillLocked
        );
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: user.to_account_info(),
                to: savings_account.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, amount)?;
        Ok(())
    }

    pub fn deposit_usdc(ctx: Context<DepositUSDC>, lock_duration: u64, amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        let savings_account = &mut ctx.accounts.savings_account;
        require!(
            lock_duration > 1 && lock_duration <= 365,
            NonceError::FundsStillLocked
        );
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.program_token_account.to_account_info(),
            authority: user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount);

        savings_account.usdc_balance += amount;

        let current_time = Clock::get()?.unix_timestamp;
        savings_account.unlock_time = current_time + (lock_duration as i64 * 24 * 60 * 60);
        savings_account.lock_duration = lock_duration;

        Ok(())
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        let savings_account = &mut ctx.accounts.savings_account;
        let user = &ctx.accounts.user;

        let current_time = Clock::get()?.unix_timestamp;

        require!(
            current_time >= savings_account.unlock_time,
            NonceError::FundsStillLocked
        );

        require!(
            current_time >= savings_account.unlock_time,
            NonceError::FundsStillLocked
        );

        require!(
            savings_account.sol_balance >= amount,
            NonceError::InsufficientFunds
        );

        let bumps = ctx.bumps.savings_account;

        let binding = user.key();
        let seeds = &[
            b"sol_savings",
            binding.as_ref(),
            savings_account.name.as_bytes(),
            &[bumps],
        ];

        let signer = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: savings_account.to_account_info(),
                to: user.to_account_info(),
            },
            signer,
        );
        anchor_lang::system_program::transfer(cpi_context, amount);
        // Update savings account SOL balance
        savings_account.sol_balance -= amount;

        Ok(())
    }

    pub fn withdraw_usdc(ctx:Context<WithdrawUSDC>,amount:u64)->Result<()>{
        let savings_account = &mut ctx.accounts.savings_account;
        let current_time = Clock::get()?.unix_timestamp;

        require!(current_time >= savings_account.unlock_time, NonceError::FundsStillLocked);

        require!(savings_account.usdc_balance >=amount , NonceError::InsufficientFunds);

        let seeds = &[
            b"program_usdc_account".as_ref(),
            &[ctx.bumps.program_token_account]
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.program_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.program_token_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        // Update savings account balance
        savings_account.usdc_balance -= amount;

        msg!("Withdrawn {} USDC from savings account '{}'", amount, savings_account.name);
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
pub struct InitializeProgramAccount<'info> {
    #[account(mut)]
    pub initialiazer: Signer<'info>,
    #[account(
        init,
        payer=initialiazer,
        seeds=[b"program_usdc_account"],
        bump,
        token::mint = mint,
        token::authority = program_authority
    )]
    pub program_account_account: Account<'info, TokenAccount>,
    #[account(seeds =[b"program_authority"],bump)]
    pub program_authority: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DepositSol<'info> {
    #[account(
        mut,
        seeds=[b"sol_savings",user.key().as_ref(),name.as_bytes()],
        bump,
        has_one= user @ NonceError::Unauthorized,
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
        has_one = user @ NonceError::Unauthorized
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"program_usdc_account"],
        bump,
    )]
    pub program_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(
        mut,
        seeds = [b"sol_savings", user.key().as_ref(), savings_account.name.as_bytes()],
        bump,
        has_one = user @ NonceError::Unauthorized
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawUSDC<'info> {
    #[account(
        mut,
        seeds=[b"usdc_savings"],
        bump,
        has_one = user @ NonceError::Unauthorized
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"program_usdc_account"],
        bump
    )]
    pub program_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct SavingsAccount {
    #[max_len(32)]
    pub name: String,
    pub user: Pubkey,
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
