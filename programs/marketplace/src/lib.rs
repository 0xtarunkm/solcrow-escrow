use anchor_lang::prelude::*;

mod contexts;
mod states;
mod error;

use contexts::*;

declare_id!("ChS5HG5cidaXzTJF9qb9huKqXGWbyGUZFvZ2SRqdfkpk");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }

    pub fn listing(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_and_close()
    }
}