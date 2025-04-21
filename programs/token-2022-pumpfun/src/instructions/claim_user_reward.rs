use crate::{
    consts::FEE_DIVIDER,
    events::UserClaimed,
    states::{InitializeConfiguration, RewardState},
    utils::token_transfer_user,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_2022::Token2022,
    token_interface::{TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct ClaimUserReward<'info> {
    ///CHECK
    #[account(
        seeds = [ b"global_config"],
        bump
    )]
    pub global_configuration: Box<Account<'info, InitializeConfiguration>>,

    ///CHECK
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    ///CHECK
    #[account(
        mut,
        seeds = [user.key().as_ref(), mint.key().as_ref(), RewardState::SEED_PREFIX],
        bump
    )]
    pub reward_state: Box<Account<'info, RewardState>>,

    ///CHECK
    #[account(mut)]
    pub token_0_mint: AccountInfo<'info>,

    ///CHECK
    #[account(mut)]
    pub token_1_mint: AccountInfo<'info>,

    ///CHECK
    #[account(mut)]
    pub user: Signer<'info>,

    ///CHECK
    #[account(
        init_if_needed,
        payer = user,
        associated_token::authority = user,
        associated_token::mint = token_0_mint
    )]
    pub user_token_0_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    ///CHECK
    #[account(
        init_if_needed,
        payer = user,
        associated_token::authority = user,
        associated_token::mint = token_1_mint
    )]
    pub user_token_1_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    ///CHECK
    #[account(mut)]
    pub operator: Signer<'info>,

    ///CHECK
    #[account(mut)]
    pub operator_token_0_ata: AccountInfo<'info>,

    ///CHECK
    #[account(mut)]
    pub operator_token_1_ata: AccountInfo<'info>,

    ///CHECK
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,

    ///CHECK
    #[account(mut)]
    pub fee_account_token_0_ata: AccountInfo<'info>,

    ///CHECK
    #[account(mut)]
    pub fee_account_token_1_ata: AccountInfo<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    /// token Program
    pub token_program: Program<'info, Token>,

    /// Spl token program or token program 2022
    pub token_0_program: Interface<'info, TokenInterface>,
    /// Spl token program or token program 2022
    pub token_1_program: Interface<'info, TokenInterface>,
    /// Token program 2022
    pub token_program_2022: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimUserReward<'info> {
    pub fn process(&mut self) -> Result<()> {
        

        Ok(())
    }
}
