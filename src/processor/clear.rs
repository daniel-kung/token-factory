use crate::{state::*, utils::*};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar,
};

pub fn process_clear(program_id: &Pubkey, accounts: &[AccountInfo], args: ClearArgs) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let mint_vault = next_account_info(account_info_iter)?;
    let transfer_auth = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;

    assert_owned_by(config_info, &program_id)?;
    let config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(signer_info, &config_data.authority)?;

    let auth_bump = assert_tranfer_authority(program_id, mint_info, transfer_auth)?;
    let authority_seed = [
        program_id.as_ref(),
        mint_info.key.as_ref(),
        "transfer_auth".as_bytes(),
        &[auth_bump],
    ];


    spl_token_transfer(
        token_program_info.clone(),
        mint_vault.clone(),
        token_account.clone(),
        transfer_auth.clone(),
        args.amt,
        &authority_seed
    )?;
    
    Ok(())
}
