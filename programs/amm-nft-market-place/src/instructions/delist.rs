use anchor_lang::prelude::*;
use anchor_spl::{
    token::{close_account, CloseAccount},
    associated_token::AssociatedToken,
    token_interface::{TokenInterface, Mint, TokenAccount, TransferChecked, transfer_checked},
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    /// The user listing their NFT
    #[account(mut)]
    pub maker: Signer<'info>,

    /// The marketplace PDA
    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// The mint of the NFT being listed
    pub maker_mint: InterfaceAccount<'info, Mint>,

    /// User's existing NFT ATA
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    /// Vault ATA (destination) for the listing PDA
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// The verified collection mint for this listing
    pub collection_mint: InterfaceAccount<'info, Mint>,

    /// Listing account PDA storing listing data
    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    /// Token interface for CPI
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl <'info> Delist <'info> {

        pub fn refund_nft (&mut self) -> Result<()> {
            // deposit the nft into the vault simple transfer using the token program

            let cpi_program = self.token_program.to_account_info();

            let cpi_accounts = TransferChecked{
                from: self.vault.to_account_info(),
                mint: self.maker_mint.to_account_info(),
                to: self.maker_ata.to_account_info(),
                authority: self.listing.to_account_info()
            };

            let seeds: &[&[u8]; 3] = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
            ];

            let signer_seeds= &[&seeds[..]];

            let ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);

            transfer_checked(ctx, self.maker_ata.amount, self.maker_mint.decimals)
        }

        pub fn close_mint_vault(&mut self) -> Result<()> {
            let seeds: &[&[u8]; 3] = &[
                &self.marketplace.key().to_bytes()[..],
                &self.maker_mint.key().to_bytes()[..],
                &[self.listing.bump],
            ];
    
            let signer_seeds= &[&seeds[..]];
    
            let cpi_program = self.token_program.to_account_info();
    
            let close_accounts = CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.maker.to_account_info(),
                authority: self.listing.to_account_info(),
            };
    
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, close_accounts, signer_seeds);
    
            close_account(cpi_ctx)
        }

}