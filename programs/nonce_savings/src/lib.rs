use anchor_lang::prelude::*;

declare_id!("8JP6GACTPuFPoQzr3Y6Yx9aKtwTUkdZPzAmLjd19vSrS");

#[program]
pub mod nonce_savings {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
