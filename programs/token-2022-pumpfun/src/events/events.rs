use anchor_lang::prelude::*;

#[event]
pub struct BondingCurveCompleted {
    pub mint_addr: Pubkey,
    pub user_ata: Pubkey,
    pub sol_pool: Pubkey,
    pub token_pool: Pubkey,
}

#[event]
pub struct LaunchEvent {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub metadata: Pubkey,
    pub decimals: u8,
    pub token_supply: u64,
    pub reserve_quote: u64,
    pub reserve_token: u64,
}
