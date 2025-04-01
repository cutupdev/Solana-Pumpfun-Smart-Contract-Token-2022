use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};

pub fn update_account_lamports_to_minimum_balance<'inf>(
    account: AccountInfo<'inf>,
    payer: AccountInfo<'inf>,
    system_program: AccountInfo<'inf>,
) -> Result<()> {
    let extra_lamports;

    if Rent::get()?.minimum_balance(account.data_len()) - account.get_lamports() > 0 {
        extra_lamports = Rent::get()?.minimum_balance(account.data_len()) - account.get_lamports();
    } else {
        extra_lamports = account.get_lamports() - Rent::get()?.minimum_balance(account.data_len());
    }

    if extra_lamports > 0 {
        invoke(
            &transfer(payer.key, account.key, extra_lamports),
            &[payer, account, system_program],
        )?;
    }
    Ok(())
}
