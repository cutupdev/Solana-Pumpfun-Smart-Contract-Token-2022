use anchor_lang::prelude::*;

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

#[event]
pub struct SwapEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,

    pub amount_in: u64,
    pub direction: u8,
    pub minimum_receive_amount: u64,
    pub amount_out: u64,

    pub reserve_quote: u64,
    pub reserve_token: u64,
}

#[event]
pub struct CompleteEvent {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
}

#[event]
pub struct WithdrawEvent {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub quote_amount: u64,
    pub token_amount: u64,
}

#[event]
pub struct MigrateEvent {
    pub admin: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_in: u64,
    pub token_1_in: u64,
}

#[event]
pub struct PlatformClaimed {
    pub creator: Pubkey,
    pub fee_nft_mint: Pubkey,
}

#[event]
pub struct UserClaimed {
    pub user: Pubkey,
    pub reward_state: Pubkey,
    pub mint: Pubkey,
}
#[event]
pub struct RewardDistributed {
    pub creator: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
}

#[event]
pub struct LiquidityRemoved {
    pub creator: Pubkey,
    pub mint: Pubkey,
}
