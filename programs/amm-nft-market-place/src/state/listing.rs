use anchor_lang::prelude::*;


#[account]
pub struct Marketplace { // this is the state that represent who listed 
    pub maker: Pubkey,
    pub maker_mint: Pubkey,
    pub price: u64,
    pub bump: u8
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32*2 + 2 + 1; // since sting is allocated in heap we need to seperetly put 4 bytes to store pointer to the heap memeory allocated
}