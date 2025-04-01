use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{self, Burn, Token},
    token_interface::{Mint, Token2022, TokenAccount},
};
use raydium_clmm_cpi::{cpi, program::RaydiumClmm, states::PoolState};

#[derive(Accounts)]
#[instruction(tick_lower_index: i32, tick_upper_index: i32,tick_array_lower_start_index:i32,tick_array_upper_start_index:i32)]
pub struct ProxyOpenPosition<'info> {
    pub clmm_program: Program<'info, RaydiumClmm>,
    /// Pays to mint the position
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Receives the position NFT
    pub position_nft_owner: UncheckedAccount<'info>,

    /// CHECK: Unique token mint address, random keypair
    #[account(mut)]
    pub position_nft_mint: Signer<'info>,

    /// CHECK: Token account where position NFT will be minted
    /// This account created in the contract by cpi to avoid large stack variables
    #[account(mut)]
    pub position_nft_account: UncheckedAccount<'info>,

    /// To store metaplex metadata
    /// CHECK: Safety check performed inside function body
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// Add liquidity for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECK: Store the information of market marking in range
    #[account(mut)]
    pub protocol_position: UncheckedAccount<'info>,

    /// CHECK: Account to mark the lower tick as initialized
    #[account(mut)]
    pub tick_array_lower: UncheckedAccount<'info>,

    /// CHECK:Account to store data for the position's upper tick
    #[account(mut)]
    pub tick_array_upper: UncheckedAccount<'info>,

    /// CHECK: personal position state
    #[account(mut)]
    pub personal_position: UncheckedAccount<'info>,

    /// The token_0 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_0.mint
    )]
    pub token_account_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The token_1 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_1.mint
    )]
    pub token_account_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
        constraint = token_vault_0.key() == pool_state.load()?.token_vault_0
    )]
    pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
        constraint = token_vault_1.key() == pool_state.load()?.token_vault_1
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Sysvar for token mint and ATA creation
    pub rent: Sysvar<'info, Rent>,

    /// Program to create the position manager state account
    pub system_program: Program<'info, System>,

    /// Program to create mint account and mint tokens
    pub token_program: Program<'info, Token>,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Program to create NFT metadata
    /// CHECK: Metadata program address constraint applied
    pub metadata_program: Program<'info, Metadata>,
    /// Program to create mint account and mint tokens
    pub token_program_2022: Program<'info, Token2022>,
    /// The mint of token vault 0
    #[account(
        address = token_vault_0.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,
    /// The mint of token vault 1
    #[account(
        address = token_vault_1.mint
    )]
    pub vault_1_mint: Box<InterfaceAccount<'info, Mint>>,
}

pub fn proxy_open_position<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ProxyOpenPosition<'info>>,
    tick_lower_index: i32,
    tick_upper_index: i32,
    tick_array_lower_start_index: i32,
    tick_array_upper_start_index: i32,
    liquidity: u128,
    amount_0_max: u64,
    amount_1_max: u64,
    with_matedata: bool,
    base_flag: bool,
) -> Result<()> {

    Ok(())
}
