use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

use crate::consts::*;
use crate::instructions::*;

declare_id!("2bPn7Ya98gJdpAsvbrhW7Hd2viVjihcYghakDYKWP29J");

#[program]
pub mod token_2022_pumpfun {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, param: InitializeConfigurationParam) -> Result<()> {
        let _ = ctx.accounts.process(param);
        Ok(())
    }

    pub fn create(ctx: Context<Create>, args: CreateMintAccountArgs ) -> Result<()> {
        let _ = ctx.accounts.process(args);
        Ok(())
    }

    pub fn buy(ctx: Context<Buy>, in_amount: u64, expected_amt: u64) -> Result<()> {
        let _ = ctx.accounts
            .process(in_amount, expected_amt, ctx.bumps.sol_pool);
        Ok(())
    }

    pub fn sell(ctx: Context<Sell>, in_amount: u64, expected_amt: u64) -> Result<()> {
        let _ = ctx.accounts
            .process(in_amount, expected_amt, ctx.bumps.sol_pool);
        Ok(())
    }

    pub fn proxy_initialize(
        ctx: Context<ProxyInitialize>,
        sqrt_price_x64: u128,
        open_time: u64,
    ) -> Result<()> {
        instructions::proxy_initialize(ctx, sqrt_price_x64, open_time)
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
        instructions::proxy_open_position(
            ctx,
            tick_lower_index,
            tick_upper_index,
            tick_array_lower_start_index,
            tick_array_upper_start_index,
            liquidity,
            amount_0_max,
            amount_1_max,
            with_matedata,
            base_flag,
        )
    }
}
