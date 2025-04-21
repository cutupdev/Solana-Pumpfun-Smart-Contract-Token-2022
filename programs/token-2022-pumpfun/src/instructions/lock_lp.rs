use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::Token,
    token_interface::{Mint, TokenAccount},
};

use raydium_locking_cpi::{cpi, program::RaydiumLiquidityLocking};

use crate::{events::PlatformClaimed, states::RewardState};

pub const LOCK_CP_AUTH_SEED: &str = "lock_cp_authority_seed";
// Seed for LockedCpLiquidityState account
pub const LOCKED_LIQUIDITY_SEED: &str = "locked_liquidity";
// Seed for LockedClmmPositionState account
pub const LOCKED_POSITION_SEED: &str = "locked_position";
#[account]
#[derive(Default, Debug)]
pub struct LockedCpLiquidityState {
    /// The Locked liquidity amount without claimed lp fee
    pub locked_lp_amount: u64,
    /// Claimed lp fee amount
    pub claimed_lp_amount: u64,
    /// Unclaimed lp fee amount
    pub unclaimed_lp_amount: u64,
    /// Last updated cp pool lp total supply
    pub last_lp: u64,
    /// Last updated cp pool k
    pub last_k: u128,
    /// Account update recent epoch
    pub recent_epoch: u64,
    /// The ID of the pool with which this record is connected
    pub pool_id: Pubkey,
    /// nft mint to check who has authority to collect fee
    pub fee_nft_mint: Pubkey,
    /// The owner who has locked liquidity
    pub locked_owner: Pubkey,
    /// The mint of locked lp token
    pub locked_lp_mint: Pubkey,
    /// Unused bytes for future upgrades.
    pub padding: [u64; 8],
}

impl LockedCpLiquidityState {
    pub const LEN: usize = 8 + 4 * 8 + 16 + 8 + 32 * 4 + 8 * 8;
}

#[derive(Accounts)]
pub struct LockingLp<'info> {
    /// CHECK: the authority of token vault that cp is locked
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,

    /// who want to lock liquidity
    #[account(mut)]
    pub liquidity_owner: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub reward_state: Box<Account<'info, RewardState>>,

    /// CHECK: locked liquidity allow who to collect fee
    pub fee_nft_owner: UncheckedAccount<'info>,

    /// Create a unique fee nft mint
    #[account(mut)]
    pub fee_nft_mint: Signer<'info>,

    /// CHECK: Token account where fee nft will be minted to, init by locking program
    #[account(mut)]
    pub fee_nft_account: UncheckedAccount<'info>,

    /// CHECK: Indicates which pool the locked liquidity belong to
    #[account()]
    pub pool_state: UncheckedAccount<'info>,

    /// CHECK: Store the locked information of liquidity
    #[account(mut)]
    pub locked_liquidity: UncheckedAccount<'info>,

    /// The mint of liquidity token
    /// address = pool_state.lp_mint
    /// CHECK:
    #[account(mut)]
    pub lp_mint: Box<InterfaceAccount<'info, Mint>>,

    /// liquidity owner lp token account
    /// CHECK
    #[account(mut)]
    pub liquidity_owner_lp: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Locked lp token deposit to
    /// CHECK
    #[account(mut)]
    pub locked_lp_vault: UncheckedAccount<'info>,

    /// The address that holds pool tokens for token_0
    /// address = pool_state.token_0_vault
    /// CHECK
    #[account(mut)]
    pub token_0_vault: UncheckedAccount<'info>,

    /// The address that holds pool tokens for token_1
    /// address = pool_state.token_1_vault
    /// CHECK
    #[account(mut)]
    pub token_1_vault: UncheckedAccount<'info>,

    /// To store metaplex metadata
    /// CHECK: Safety check performed inside function body
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// Sysvar for token mint and ATA creation
    pub rent: Sysvar<'info, Rent>,

    /// Program to create the new account
    pub system_program: Program<'info, System>,

    /// Program to create/transfer mint/token account
    pub token_program: Program<'info, Token>,

    /// Program to create an ATA for receiving fee NFT
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Program to create NFT metadata accunt
    /// CHECK: Metadata program address constraint applied
    pub metadata_program: Program<'info, Metadata>,

    /// CHECK
    pub locking_program: Program<'info, RaydiumLiquidityLocking>,
}

pub fn lock_cp_liquidity_process<'info>(ctx: Context<LockingLp>) -> Result<()> {
    

    Ok(())
}
