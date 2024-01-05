use arrayref::array_ref;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    keccak::hashv,
    program_error::ProgramError,
    pubkey::Pubkey
};

use crate::{ferror, state::*, utils::*};

pub fn process_close(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let round_info = next_account_info(account_info_iter)?;
    let new_config_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    assert_owned_by(config_info, &program_id)?;
    let mut config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(signer_info, &config_data.authority)?;

    let mut round_data = RoundData::from_account_info(round_info)?;
    if config_data.authority != *signer_info.key {
        return ferror!("invalid authority");
    }
    let mut total_allocated = 0;
    if config_data.match1 > 0 {
        total_allocated += config_data.total_reward * 2 / 100;
    }
    if config_data.match2 > 0 {
        total_allocated += config_data.total_reward * 3 / 100;
    }
    if config_data.match3 > 0 {
        total_allocated += config_data.total_reward * 5 / 100;
    }
    if config_data.match4 > 0 {
        total_allocated += config_data.total_reward * 20 / 100;
    }
    if config_data.match5 > 0 {
        total_allocated += config_data.total_reward * 30 / 100;
    }
    if config_data.match6 > 0 {
        total_allocated += config_data.total_reward * 40 / 100;
    }
    
    let new_round = config_data.round + 1;
    let bump = assert_config(&program_id, &new_config_info, new_round.to_string().clone())?;
    if new_config_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            new_config_info,
            rent_info,
            system_info,
            signer_info,
            ConfigureData::LEN,
            &[
                program_id.as_ref(),
                "config".as_bytes(),
                new_round.to_string().as_bytes(),
                &[bump],
            ],
        )?;
    }

    
    config_data.closed = true;
    config_data.allocated = total_allocated;
    

    let mut new_config_data = ConfigureData::from_account_info(config_info)?; 

    let hash = hashv(&[&now_timestamp().to_be_bytes().as_slice(), &program_id.as_ref()]);
    let kep_bytes = hash.to_bytes();
    let bts = array_ref![kep_bytes, 0, 16];
    let num = u128::from_be_bytes(*bts);
    let fnum = (num % 1000000) as u64;
    
    new_config_data.target = fnum;
    config_data.serialize(&mut &mut config_info.data.borrow_mut()[..])?;

    new_config_data.authority = config_data.authority;
    new_config_data.start_time = now_timestamp();
    new_config_data.round = new_round;
    new_config_data.total_reward = config_data.total_reward;
    new_config_data.charge_addr = config_data.charge_addr;
    new_config_data.serialize(&mut &mut new_config_info.data.borrow_mut()[..])?;

    round_data.round += 1;
    round_data.serialize(&mut &mut round_info.data.borrow_mut()[..])?;

    Ok(())
}
