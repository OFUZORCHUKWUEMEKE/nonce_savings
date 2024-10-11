mod constants;
mod errors;
mod instructions;
mod state;

use crate::state::SavingsType;

use instructions::*;

use anchor_lang::prelude::*;
// use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use errors::NonceError;

declare_id!("8JP6GACTPuFPoQzr3Y6Yx9aKtwTUkdZPzAmLjd19vSrS");

#[program]
pub mod nonce_savings {

    use anchor_lang::Bumps;

    use super::*;

    pub fn initializesol(
        ctx: Context<InitializeSolSavings>,
        random_seed: String,
        name: String,
        duration: i64,
        type_Of_savings: SavingsType,
        usd_price: Option<f64>,
    ) -> Result<()> {
        ctx.accounts.initialize(
            random_seed,
            name,
            duration,
            type_Of_savings,
            usd_price,
            &ctx.bumps,
        )?;
        Ok(())
    }

    pub fn initializeusdcsavings(
        ctx: Context<InitializeUSDCSavings>,
        random_seed: String,
        name: String,
        duration: i64,
        type_Of_savings: SavingsType,
        usd_price: Option<f64>,
    ) -> Result<()> {
        ctx.accounts.initializeusdcsavings(
            random_seed,
            name,
            duration,
            type_Of_savings,
            usd_price,
            &ctx.bumps,
        )?;
        Ok(())
    }

    pub fn deposit_sol (ctx:Context<DepositSol>,amount:u64)->Result<()>{
        ctx.accounts.deposit_sol(amount)?;
        Ok(())
    }
}
