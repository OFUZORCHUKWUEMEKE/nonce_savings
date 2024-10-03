mod constants;
mod state;
mod errors;
mod instructions;

use instructions::*;

use anchor_lang::prelude::*;
// use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use errors::NonceError;

declare_id!("8JP6GACTPuFPoQzr3Y6Yx9aKtwTUkdZPzAmLjd19vSrS");

#[program]
pub mod nonce_savings {

    use super::*;


}