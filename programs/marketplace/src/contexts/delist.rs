use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022::transfer_checked, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, CloseAccount, close_account}};

use crate::states::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    marketplace: Box<Account<'info, Marketplace>>,
    maker_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program
    )]
    vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        close = maker,
        seeds = [
            b"listing", 
            marketplace.key().as_ref(), 
            maker_mint.key().as_ref()
        ],
        bump,
    )]
    listing: Account<'info, Listing>,
    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>
}

impl<'info> Delist<'info> {
    pub fn withdraw_and_close(&mut self) -> Result<()> {
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let account = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), account, signer_seeds);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;

        let cpi_account = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info()
        };

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_account, signer_seeds);

        close_account(ctx)
    }
}