use crate::{
    consts::FEE_DIVIDER,
    states::{BondingCurve, InitializeConfiguration, RewardState},
    utils::{sol_transfer_with_signer, token_transfer_with_signer},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct TransferMigrationFee<'info> {
    #[account(
        seeds = [ b"global_config"],
        bump
    )]
    pub global_configuration: Box<Account<'info, InitializeConfiguration>>,

    ///CHECK
    #[account(mut)]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    ///CHECK
    #[account(mut)]
    pub token_deployer: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = RewardState::SIZE,
        seeds = [&token_deployer.key().to_bytes(), &mint.key().to_bytes(), RewardState::SEED_PREFIX],
        bump
    )]
    pub reward_state: Box<Account<'info, RewardState>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    ///CHECK
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    ///CHECK
    #[account(
        mut,
        seeds = [ &mint.key().to_bytes() , b"sol_pool".as_ref() ],
        bump)]
    pub sol_pool: AccountInfo<'info>,

    ///CHECK
    #[account(mut)]
    pub pool_token_ata: AccountInfo<'info>,

    ///CHECK
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = fee_account,
        associated_token::token_program = token_program
    )]
    pub fee_acccount_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> TransferMigrationFee<'info> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let binding = self.mint.key();
        let mint_bytes = binding.as_ref();

        // Create the signer seeds structure
        let seeds = &[mint_bytes, b"sol_pool".as_ref(), &[bump]];

        // Create the outer structure required by the function
        let signer_seeds = &[&seeds[..]];

        let fee_sol_amount = self.bonding_curve.sol_reserves
            * self.global_configuration.migration_fee_percent
            / FEE_DIVIDER;
        let migration_sol_amount = self.bonding_curve.sol_reserves - fee_sol_amount;

        let migration_token_amount = (migration_sol_amount as u128
            * self.bonding_curve.token_reserves as u128
            / (self.bonding_curve.sol_reserves + self.bonding_curve.init_virtual_sol) as u128)
            as u64;
        let fee_token_amount = self.bonding_curve.token_reserves - migration_token_amount;

        msg!(
            "Raydium Input:: Token: {:?}  Sol: {:?}",
            migration_token_amount,
            migration_sol_amount
        );
        msg!(
            "Fee percent: {:?}",
            self.global_configuration.migration_fee_percent
        );
        msg!(
            "Fee:: Token: {:?}  Sol: {:?}",
            fee_token_amount,
            migration_sol_amount
        );

        self.reward_state.init(self.mint.key())?;

        sol_transfer_with_signer(
            self.sol_pool.to_account_info(),
            self.fee_account.to_account_info(),
            &self.system_program,
            signer_seeds,
            fee_sol_amount,
        )?;

        token_transfer_with_signer(
            self.mint.to_account_info(),
            self.pool_token_ata.to_account_info(),
            self.sol_pool.to_account_info(),
            self.fee_acccount_token_ata.to_account_info(),
            &self.token_program,
            signer_seeds,
            fee_token_amount,
        )?;

        Ok(())
    }
}
