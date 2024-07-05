use anchor_lang::prelude::*;

declare_id!("CwswgaYiDLpgN2sWQpbcyWYWTk7sVM8sfkvS6nSnWB63");

#[program]
pub mod solana_access_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
