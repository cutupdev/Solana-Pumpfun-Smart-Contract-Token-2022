use std::ops::{Div, Mul};

use anchor_lang::{
    prelude::*,
    solana_program::{native_token::LAMPORTS_PER_SOL, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    consts::SOL_POOL_SEED,
    errors::RaydiumPumpfunError,
    events::BondingCurveCompleted,
    states::{BondingCurve, InitializeConfiguration},
};

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(
        seeds = [InitializeConfiguration::SEEDS],
        bump
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,

    #[account(
        mut,
        seeds =[ &mint_addr.key().to_bytes() , BondingCurve::POOL_SEED_PREFIX ],
        bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    pub mint_addr: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_addr,
        associated_token::authority = payer,
        associated_token::token_program = token_program,
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK:
    #[account(
        mut,
        seeds = [ &mint_addr.key().to_bytes() , SOL_POOL_SEED ],
        bump
    )]
    pub sol_pool: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = mint_addr,
        associated_token::authority = sol_pool,
        associated_token::token_program = token_program,
    )]
    pub token_pool: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK:
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Buy<'info> {
    pub fn process(&mut self, sol_input_amt: u64, expected_amt: u64, bump: u8) -> Result<()> {

        Ok(())
    }
}
