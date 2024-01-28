use anchor_lang::prelude::*;
use anchor_spl::
    token::{self, Mint, Token, TokenAccount};

declare_id!("8evyL4dBzmnbHX7PFLY9H6nb7Ux6jxqNuk4n8z8K2xTX");

#[program]
pub mod staking_program {
    use super::*;

    pub mod constants {
        pub const VAULT_SEED: &[u8] = b"vault";
        pub const STAKE_INFO_SSED: &[u8] = b"stake_info";
        pub const TOKEN_SEED: &[u8] = b"token";
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn Stake(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        Ok(())
    }


    pub fn DeStake(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }



}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds= [constants::VAULT_SEED],
        bump,
        payer=signer,
        token::mint = mint,
        token::authority= token_vault_account,

    )]

    pub token_vault_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info,System>,

}
