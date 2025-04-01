use std::ops::{Div, Mul};

use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct BondingCurve {
    pub init_virtual_sol: u64,
    pub init_virtual_token: u64,
    pub token_supply: u64,
    pub token_reserves: u64,
    pub sol_reserves: u64,
    pub k_param: u128,
}

impl BondingCurve {
    pub const POOL_SEED_PREFIX: &'static [u8] = b"bonding_curve";

    pub const SIZE: usize = 68 + 8;

    pub fn init(
        &mut self,
        init_virtual_sol: u64,
        init_virtual_token: u64,
        token_supply: u64,
    ) -> Result<()> {
        self.init_virtual_sol = init_virtual_sol;
        self.init_virtual_token = init_virtual_token;
        self.token_supply = token_supply;
        self.sol_reserves = 0;
        self.token_reserves = init_virtual_token;
        self.k_param = (init_virtual_sol as u128).mul(init_virtual_token as u128);

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        let token = self.init_virtual_token
            - self
                .k_param
                .div((self.init_virtual_sol + self.sol_reserves) as u128) as u64;

        let price = ((self.init_virtual_sol + self.sol_reserves) as f32)
            .div((self.init_virtual_token - token.clone()) as f32);

        let market_cap = (price as f32).mul(self.token_supply as f32);

        let initial_multiple = (self.init_virtual_sol as f32).div(self.init_virtual_token as f32);

        msg!(
            "Update Bonding Curve : {:#?} , price : {} , market_cap : {} , increase_multiple : {}",
            self.clone(),
            price,
            market_cap,
            price.div(initial_multiple)
        );

        Ok(())
    }

    pub fn get(&self) -> &BondingCurve {
        self
    }
}
