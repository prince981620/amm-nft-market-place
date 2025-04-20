use anchor_lang::prelude::*;



#[account]
pub struct Marketplace { // this is the configuration of market place which includes details like admins, fee etc
    pub admin: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub treasury_bump: u8, // to put the fee we are collecting
    pub rewards_bump: u8,   // to reward the users interacting with our platform
    pub name: String
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + 3*1 + (4+32); // since sting is allocated in heap we need to seperetly put 4 bytes to store pointer to the heap memeory allocated
}