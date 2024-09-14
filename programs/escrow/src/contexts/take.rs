use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, CloseAccount, close_account}};

use crate::states::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    taker: Signer<'info>,
    #[account(mut)]
    maker: SystemAccount<'info>,
    #[account(
        mint::token_program = token_program
    )]
    mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    vault: InterfaceAccount<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> Take<'info> {
    pub fn settle_and_close(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let seeds = &[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_be_bytes(),
            &[self.escrow.bump]
        ];

        let signer_seeds = &[&seeds[..]];
        
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount, self.mint.decimals)?;

        let cpi_account = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_account, signer_seeds);

        close_account(ctx)
    }
}
