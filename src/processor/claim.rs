use crate::{ferror, state::*, utils::*};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

pub fn process_claim(program_id: &Pubkey, accounts: &[AccountInfo], args: ClaimArgs) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?; //nft creator: pda
    let mint_info = next_account_info(account_info_iter)?;
    let user_info = next_account_info(account_info_iter)?;
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

    let round = args.round.to_string();
    assert_user_info(program_id, &signer_info.key, user_info, round.clone())?;
    assert_config(&program_id, &config_info, round.clone())?;
    assert_mint_vault(program_id, mint_info, mint_vault)?;

    let config_data = ConfigureData::from_account_info(config_info)?;
    if !config_data.closed {
        return ferror!("sale not closed");
    }
    let auth_bump = assert_tranfer_authority(program_id, mint_info, transfer_auth)?;
    let authority_seed = [
        program_id.as_ref(),
        mint_info.key.as_ref(),
        "transfer_auth".as_bytes(),
        &[auth_bump],
    ];

    let mut user_data = UserData::from_account_info(user_info)?;
    if user_data.claimed {
        return ferror!("claimed");
    }
    let mut reward = 0;
    if config_data.match1 > 0 {
        reward += config_data.total_reward * 2 / 100 * user_data.match1 / config_data.match1;
    }
    if config_data.match2 > 0 {
        reward += config_data.total_reward * 3 / 100 * user_data.match2 / config_data.match2;
    }
    if config_data.match3 > 0 {
        reward += config_data.total_reward * 5 / 100 * user_data.match3 / config_data.match3;
    }
    if config_data.match4 > 0 {
        reward += config_data.total_reward * 20 / 100 * user_data.match4 / config_data.match4;
    }
    if config_data.match5 > 0 {
        reward += config_data.total_reward * 30 / 100 * user_data.match5 / config_data.match5;
    }
    if config_data.match6 > 0 {
        reward += config_data.total_reward * 40 / 100 * user_data.match6 / config_data.match6;
    }

    spl_token_transfer(
        token_program_info.clone(),
        mint_vault.clone(),
        token_account.clone(),
        transfer_auth.clone(),
        reward as u64,
        &authority_seed
    )?;
    
    user_data.claimed = true;
    user_data.reward = reward;
    user_data.serialize(&mut *user_info.try_borrow_mut_data()?)?;
    Ok(())
}
