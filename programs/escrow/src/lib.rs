use anchor_lang::prelude::*;

mod contexts;
mod states;

use contexts::*;
declare_id!("HMP6azeaQAa36etFSPzjZttopYNtdzFZ7ToE3s4cXaFh");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64) -> Result<()> {       
        ctx.accounts.save_escrow(seed, &ctx.bumps)?;
        ctx.accounts.deposit_to_vault(amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.settle_and_close()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()
    }
}