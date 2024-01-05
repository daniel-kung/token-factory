use arrayref::array_ref;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    sysvar,
    keccak::hashv,
    program_error::ProgramError,
    pubkey::Pubkey
};

use crate::{ferror, state::*, utils::*};

pub fn process_configure(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ConfigureArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let round_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let mint_vault = next_account_info(account_info_iter)?;
    let transfer_auth = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    let bump = assert_config(&program_id, &config_info, args.round.clone())?;

    let round_bump = assert_round(program_id, round_info)?;
    let round_seed = [
        program_id.as_ref(),
        "round".as_bytes(),
        &[round_bump],
    ];
    let mint_vault_bump = assert_mint_vault(program_id, mint_info, mint_vault)?;
    let mint_vault_seed = [
        program_id.as_ref(),
        mint_info.key.as_ref(),
        "mint_vault".as_bytes(),
        &[mint_vault_bump],
    ];
    let auth_bump = assert_tranfer_authority(program_id, mint_info, transfer_auth)?;
    let authority_seed = [
        program_id.as_ref(),
        mint_info.key.as_ref(),
        "transfer_auth".as_bytes(),
        &[auth_bump],
    ];
    let mut is_created = true;
    if config_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            config_info,
            rent_info,
            system_info,
            signer_info,
            ConfigureData::LEN,
            &[
                program_id.as_ref(),
                "config".as_bytes(),
                args.round.as_bytes(),
                &[bump],
            ],
        )?;
        msg!("create mint vault");
        spl_token_create_account(
            &token_program_info,
            &signer_info,
            &mint_info,
            &mint_vault,
            &transfer_auth,
            &mint_vault_seed,
            &authority_seed,
            &rent_info,
        )?;
        is_created = false;
    }

    if round_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            round_info,
            rent_info,
            system_info,
            signer_info,
            RoundData::LEN,
            &[
                program_id.as_ref(),
                "round".as_bytes(),
                &[round_bump],
            ],
        )?;
    }
    let mut round_data = RoundData::from_account_info(round_info)?;
    let mut config_data = ConfigureData::from_account_info(config_info)?;

    if is_created {
        if config_data.authority != *signer_info.key {
            return ferror!("invalid authority");
        }
        assert_owned_by(config_info, &program_id)?;
    }

    let hash = hashv(&[&now_timestamp().to_be_bytes().as_slice(), &program_id.as_ref()]);
    let kep_bytes = hash.to_bytes();
    let bts = array_ref![kep_bytes, 0, 16];
    let num = u128::from_be_bytes(*bts);
    let fnum = (num % 1000000) as u64;

    config_data.target = fnum;
    config_data.authority = args.authority;
    config_data.start_time = args.start_time;
    config_data.round = args.round.parse().unwrap();
    config_data.total_reward = args.total_reward;
    config_data.charge_addr = args.charge_addr;
    config_data.token = mint_info.key.clone();
    config_data.serialize(&mut &mut config_info.data.borrow_mut()[..])?;

    round_data.round = args.round.parse().unwrap();
    round_data.serialize(&mut &mut round_info.data.borrow_mut()[..])?;

    Ok(())
}
