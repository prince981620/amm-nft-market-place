use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata, MetadataAccount, MasterEditionAccount},
    token_interface::{TokenInterface, Mint, TokenAccount, TransferChecked, transfer_checked},
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info> {
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
        init,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// The verified collection mint for this listing
    pub collection_mint: InterfaceAccount<'info, Mint>,

    /// Listing account PDA storing listing data
    #[account(
        init,
        payer = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
        space = Listing::INIT_SPACE
    )]
    pub listing: Account<'info, Listing>,

    /// Metadata account for the NFT; must belong to `collection_mint`
    #[account(
        seeds = [
            b"metadata", 
            metadata_program.key().as_ref(), 
            maker_mint.key().as_ref()
            ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,

    /// Master edition for the NFT
    #[account(
        seeds = [
            b"metadata", 
            metadata_program.key().as_ref(), 
            maker_mint.key().as_ref(), 
            b"edition"
            ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    /// Metaplex Token Metadata program
    pub metadata_program: Program<'info, Metadata>,

    /// Token interface for CPI
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl <'info> List <'info> {
        pub fn create_listing (&mut self, price:u64, bumps: &ListBumps) -> Result<()> {
            self.listing.set_inner(Listing { 
                maker: self.maker.key(), 
                maker_mint: self.maker_mint.key(), 
                price, 
                bump: bumps.listing
            });

            Ok(())
        }

        pub fn deposit_nft (&mut self) -> Result<()> {
            // deposit the nft into the vault simple transfer using the token program

            let cpi_program = self.token_program.to_account_info();

            let cpi_accounts = TransferChecked{
                from: self.maker_ata.to_account_info(),
                mint: self.maker_mint.to_account_info(),
                to: self.vault.to_account_info(),
                authority: self.maker.to_account_info()
            };

            let ctx = CpiContext::new(cpi_program,cpi_accounts);

            transfer_checked(ctx, self.maker_ata.amount, self.maker_mint.decimals)
        }
}