#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("HvR3wzpj2MCnS4jgCTqHiZE4fGaM3w9BBAtA7UvCWTS6");

#[program]
pub mod licence_me {
    use super::*;

  pub fn close(_ctx: Context<CloseLicenceMe>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.licence_me.count = ctx.accounts.licence_me.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.licence_me.count = ctx.accounts.licence_me.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeLicenceMe>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.licence_me.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeLicenceMe<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + LicenceMe::INIT_SPACE,
  payer = payer
  )]
  pub licence_me: Account<'info, LicenceMe>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseLicenceMe<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub licence_me: Account<'info, LicenceMe>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub licence_me: Account<'info, LicenceMe>,
}

#[account]
#[derive(InitSpace)]
pub struct LicenceMe {
  count: u8,
}