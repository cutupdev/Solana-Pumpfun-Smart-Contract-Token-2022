use anchor_lang::{
    prelude::*,
    solana_program::{self, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::spl_token,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use raydium_clmm_cpi::{
    cpi,
    program::RaydiumClmm,
    states::{AmmConfig, POOL_SEED, POOL_TICK_ARRAY_BITMAP_SEED, POOL_VAULT_SEED},
};

use crate::consts::SOL_POOL_SEED;

#[derive(Accounts)]
pub struct ProxyInitialize<'info> {
    
}

pub fn proxy_initialize(
    ctx: Context<ProxyInitialize>,
    sqrt_price_x64: u128,
    open_time: u64,
) -> Result<()> {
    msg!("Transfer to Op Address");

    msg!(
        "clmm {}",
        ctx.accounts
            .clmm_program
            .to_account_info()
            .key()
            .to_string()
    );

    let cpi_context = CpiContext::new(ctx.accounts.clmm_program.to_account_info(), cpi_accounts);
    cpi::create_pool(cpi_context, sqrt_price_x64, open_time)
}
