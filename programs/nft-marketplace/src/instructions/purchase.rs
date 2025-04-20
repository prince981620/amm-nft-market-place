use anchor_lang::{prelude::*,
     system_program::{transfer, Transfer}
};

use anchor_spl::{
    token::{close_account, mint_to, CloseAccount, MintTo}, 
    token_interface::{
        transfer_checked, 
        Mint, 
        TokenAccount, 
        TokenInterface, 
        TransferChecked
    }
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase <'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        // init_if_needed,  // avoid using init_if_needed
        // payer = taker,
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        // init_if_needed,  // avoid using init_if_needed create and init on client side using getorcreateata
        // payer = taker,
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = taker
    )]
    pub taker_rewards_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        // init_if_needed,  // avoid using init_if_needed create and init on client side using getorcreateata
        // payer = taker,
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = maker
    )]
    pub maker_rewards_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [ marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    // we need treasury to send platform fees
    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
        mint::decimals = 6,
        mint::authority = marketplace
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl <'info> Purchase <'info> {
    pub fn purchase(&mut self) -> Result<()> {
        self.send_sol()?;
        self.send_nft()?;
        self.close_mint_vault()?;
        self.reward_both_parties()?;

        Ok(())
    }
    pub fn send_sol (&mut self) -> Result<()> {

        let marketplace_fee = (self.marketplace.fee as u64)
            .checked_mul(self.listing.price)
            .unwrap()
            .checked_div(10000_u64)
            .unwrap();

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let amount = self.listing.price.checked_sub(marketplace_fee).unwrap();

        transfer(cpi_ctx,amount)?;
        
        // now we transfer the markte-place fee form taker to treasury
        // you can keep the variable names same due to rust shadowing feature
        // let shadowing creates a new binding, which can even have a different type.

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, marketplace_fee)

    }
    // now we need to send nft from vault to taker ata
    pub fn send_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let seeds: &[&[u8]; 3] = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds= &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)
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

    pub fn reward_both_parties (&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        
        let mint_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.maker_rewards_ata.to_account_info(),
            authority: self.marketplace.to_account_info()
        };

        let seeds:&[&[u8]] = &[
            b"marketplace",
            &self.marketplace.name.as_str().as_bytes(),
            &[self.marketplace.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, mint_accounts, signer_seeds);

        mint_to(cpi_ctx, self.listing.price)?;

        let cpi_program = self.token_program.to_account_info();
        
        let mint_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.taker_rewards_ata.to_account_info(),
            authority: self.marketplace.to_account_info()
        };

        let seeds:&[&[u8]] = &[
            b"marketplace",
            &self.marketplace.name.as_str().as_bytes(),
            &[self.marketplace.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, mint_accounts, signer_seeds);

        mint_to(cpi_ctx, self.listing.price)?;

        Ok(())

    }
}