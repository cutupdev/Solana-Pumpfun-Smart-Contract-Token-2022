use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenAccount, TokenInterface},
};

use crate::{
    consts::FEE_DIVIDER,
    events::RewardDistributed,
    states::{InitializeConfiguration, RewardState},
    utils::token_transfer_user,
};

#[derive(Accounts)]
pub struct RewardDistribute<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,

    ///CHECK
    #[account()]
    pub creator: UncheckedAccount<'info>,

    /// CHECK
    #[account(mut)]
    pub global_configuration: Box<Account<'info, InitializeConfiguration>>,

    /// CHECK
    #[account(mut)]
    pub reward_state: Box<Account<'info, RewardState>>,

    /// CHECK
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,

    /// CHECK
    #[account(mut)]
    pub token_0_mint: AccountInfo<'info>,

    /// CHECK
    #[account(mut)]
    pub token_1_mint: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = token_0_mint,
        associated_token::authority = operator,
        associated_token::token_program = token_0_program,
    )]
    pub operator_token_0_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_1_mint,
        associated_token::authority = operator,
        associated_token::token_program = token_1_program,
    )]
    pub operator_token_1_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK
    #[account(mut)]
    pub creator_token_0_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK
    #[account(mut)]
    pub creator_token_1_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK
    #[account(
        init_if_needed,
        payer = operator,
        associated_token::mint = token_0_mint,
        associated_token::authority = fee_account,
        associated_token::token_program = token_0_program
    )]
    pub fee_token_0_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK
    #[account(
        init_if_needed,
        payer = operator,
        associated_token::mint = token_1_mint,
        associated_token::authority = fee_account,
        associated_token::token_program = token_1_program
    )]
    pub fee_token_1_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_0_program: Interface<'info, TokenInterface>,
    pub token_1_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> RewardDistribute<'info> {
    pub fn process(self) -> Result<()> {
        let creator_0_amount = self.creator_token_0_account.amount
            * self.global_configuration.user_reward_percent
            / FEE_DIVIDER;
        let fee_0_amount = self.creator_token_0_account.amount - creator_0_amount;
        token_transfer_user(
            self.token_0_mint.to_account_info(),
            self.operator_token_0_account.to_account_info(),
            &self.operator,
            self.creator_token_0_account.to_account_info(),
            &self.token_0_program,
            creator_0_amount,
        )?;
        msg!(
            "Reward Toknen0 transfered to the creator. {}",
            creator_0_amount
        );

        token_transfer_user(
            self.token_0_mint.to_account_info(),
            self.operator_token_0_account.to_account_info(),
            &self.operator,
            self.fee_token_0_account.to_account_info(),
            &self.token_0_program,
            fee_0_amount,
        )?;

        msg!(
            "Reward Toknen0 transfered to the fee account. {}",
            fee_0_amount
        );

        let creator_1_amount = self.creator_token_1_account.amount
            * self.global_configuration.user_reward_percent
            / FEE_DIVIDER;
        let fee_1_amount = self.creator_token_1_account.amount - creator_1_amount;
        token_transfer_user(
            self.token_1_mint.to_account_info(),
            self.operator_token_1_account.to_account_info(),
            &self.operator,
            self.creator_token_1_account.to_account_info(),
            &self.token_1_program,
            creator_1_amount,
        )?;

        msg!(
            "Reward Toknen1 transfered to the creator. {}",
            creator_1_amount
        );

        token_transfer_user(
            self.token_1_mint.to_account_info(),
            self.operator_token_1_account.to_account_info(),
            &self.operator,
            self.fee_token_1_account.to_account_info(),
            &self.token_1_program,
            fee_1_amount,
        )?;

        msg!(
            "Reward Toknen1 transfered to the fee account. {}",
            fee_1_amount
        );

        emit!(RewardDistributed {
            creator: self.creator.key(),
            token_0: self.token_0_mint.key(),
            token_1: self.token_1_mint.key()
        });

        Ok(())
    }
}
