use anchor_lang::prelude::*;

declare_id!("8evyL4dBzmnbHX7PFLY9H6nb7Ux6jxqNuk4n8z8K2xTX");

#[program]
pub mod staking_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
