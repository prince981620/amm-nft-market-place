use anchor_lang::prelude::*;

#[event]
pub struct InitializeEvent {
    pub admin: Pubkey,
    pub marketplace:  Pubkey,
    pub treasury: Pubkey,
    pub reward_mint: Pubkey,
    pub name: String,
    pub fee: u16,
}

#[event]
pub struct ListEvent {
    pub maker: Pubkey,
    pub maker_ata: Pubkey,
    pub marketplace:  Pubkey,
    pub maker_mint: Pubkey,
    pub vault: Pubkey,
    pub collection_mint: Pubkey,
    pub listing: Pubkey,
    pub name: String,
    pub price: u64,
    pub fee: u16,
}

#[event]
pub struct PurchaseEvent {
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub taker_ata: Pubkey,
    pub marketplace:  Pubkey,
    pub treasury: Pubkey,
    pub reward_mint: Pubkey,
    pub maker_rewards_ata: Pubkey,
    pub taker_rewards_ata: Pubkey,
    pub maker_mint: Pubkey,
    pub vault: Pubkey,
    pub listing: Pubkey,
    pub name: String,
    pub price: u64,
    pub fee: u16,
}

#[event]
pub struct DelistEvent {
    pub maker: Pubkey,
    pub maker_ata: Pubkey,
    pub marketplace:  Pubkey,
    pub maker_mint: Pubkey,
    pub vault: Pubkey,
    pub collection_mint: Pubkey,
    pub listing: Pubkey,
    pub name: String,
    pub price: u64,
    pub fee: u16,
}