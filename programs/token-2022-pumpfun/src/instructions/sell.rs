use std::ops::{Div, Mul};

use crate::{
    consts::SOL_POOL_SEED, errors::RaydiumPumpfunError, states::{BondingCurve, InitializeConfiguration}
};
use anchor_lang::{
    prelude::*,
    solana_program::{native_token::LAMPORTS_PER_SOL, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct Sell<'info> {
    
}

impl<'info> Sell<'info> {
    pub fn process(&mut self, in_token_amount: u64, expected_amt: u64, bump: u8) -> Result<()> {
        let estimated_out_sol = ((in_token_amount as f64)
            .mul((self.bonding_curve.init_virtual_sol + self.bonding_curve.sol_reserves) as f64)
            .div(self.bonding_curve.token_reserves as f64)) as u64;

        msg!("{} > {}", estimated_out_sol, expected_amt);
        msg!(
            "Sell : {} Token => {} Sol ( Price : {} )",
            in_token_amount.div(LAMPORTS_PER_SOL),
            estimated_out_sol.div(LAMPORTS_PER_SOL),
            ((self.bonding_curve.init_virtual_sol + self.bonding_curve.sol_reserves) as f64)
                .div(self.bonding_curve.token_reserves as f64)
        );

        if estimated_out_sol < expected_amt {
            err!(RaydiumPumpfunError::SlippageExcceded).unwrap()
        }

        Ok(())
    }
}
