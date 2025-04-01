use anchor_lang::prelude::*;
use crate::{
    consts::InitializeConfigurationParam,
    states::InitializeConfiguration,
};

#[derive(Accounts)]
#[instruction(params : InitializeConfigurationParam)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [InitializeConfiguration::SEEDS],
        payer = payer,
        space = InitializeConfiguration::SIZE,
        bump
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,

    /// CHECK:
    pub fee_account: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn process(&mut self, param: InitializeConfigurationParam) -> Result<()> {

        msg!("Global State : {:#?}" , param);

        let _ = self.global_configuration.set_value(param);

        Ok(())
    }
}
