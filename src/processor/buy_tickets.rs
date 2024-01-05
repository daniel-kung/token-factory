use crate::{ferror, state::*, utils::*};
use arrayref::array_ref;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    keccak::hashv,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, sysvar,
};

pub fn process_buy(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: BuyTicketsArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let user_info = next_account_info(account_info_iter)?;
    let charge_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    assert_eq_pubkey_0(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey_1(&system_info, &solana_program::system_program::id())?;

    let mut config_data = ConfigureData::from_account_info(config_info)?;
    let now_ts = now_timestamp();
    //check sale state
    if config_data.start_time > now_ts || config_data.closed {
        return ferror!("sale not open");
    }
    let round = config_data.round.to_string();
    let user_bump = assert_user_info(program_id, &signer_info.key, user_info, round.clone())?;
    let user_seeds = [
        program_id.as_ref(),
        signer_info.key.as_ref(),
        "user_info".as_bytes(),
        round.as_bytes(),
        &[user_bump],
    ];

    if user_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            user_info,
            rent_info,
            system_info,
            signer_info,
            UserData::LEN,
            &user_seeds,
        )?;
    }

    let price = 50000000 as u64;
    let mut user_data = UserData::from_account_info(user_info)?;
    invoke(
        &system_instruction::transfer(&signer_info.key, &charge_info.key.clone(), price * args.num),
        &[
            signer_info.clone(),
            charge_info.clone(),
            system_info.clone(),
        ],
    )?;
    let shot: [u8; 6];
    if args.shot == None {
        let hash = hashv(&[
            &now_timestamp().to_be_bytes().as_slice(),
            &signer_info.key.as_ref(),
        ]);
        let kep_bytes = hash.to_bytes();
        let bts = array_ref![kep_bytes, 0, 16];
        let num = u128::from_be_bytes(*bts);
        let fnum = (num % 1000000) as u64;
        let array_u8: [u8; 6] = [
            ((fnum / 100000) % 10) as u8,
            ((fnum / 10000) % 10) as u8,
            ((fnum / 1000) % 10) as u8,
            ((fnum / 100) % 10) as u8,
            ((fnum / 10) % 10) as u8,
            (fnum % 10) as u8,
        ];
        shot = array_u8;
        user_data.shots.insert(array_u8, args.num);
    } else {
        user_data.shots.insert(args.shot.unwrap(), args.num);
        shot = args.shot.unwrap();
    }

    // let target_array: [u8; 6] = [
    //         ((config_data.target / 100000) % 10) as u8,
    //         ((config_data.target / 10000) % 10) as u8,
    //         ((config_data.target / 1000) % 10) as u8,
    //         ((config_data.target / 100) % 10) as u8,
    //         ((config_data.target / 10) % 10) as u8,
    //         (config_data.target % 10) as u8,
    //     ];

    // let matched = count_matching_elements_until_difference(&shot, &target_array) as u8;
    
    // match matched {
    //     1 => {
    //         user_data.match1 += args.num;
    //         config_data.match1 += args.num;
    //     }
    //     2 => {
    //         user_data.match2 += args.num;
    //         config_data.match2 += args.num;
    //     }
    //     3 => {
    //         user_data.match3 += args.num;
    //         config_data.match3 += args.num;
    //     }
    //     4 => {
    //         user_data.match4 += args.num;
    //         config_data.match4 += args.num;
    //     }
    //     5 => {
    //         user_data.match5 += args.num;
    //         config_data.match5 += args.num;
    //     }
    //     6 => {
    //         user_data.match6 += args.num;
    //         config_data.match6 += args.num;
    //     }

    //     _ => {}
    // }

    user_data.round = config_data.round;
    user_data.total_shots += args.num;
    user_data.serialize(&mut *user_info.try_borrow_mut_data()?)?;

    config_data.total_shots += args.num;
    config_data.serialize(&mut *config_info.try_borrow_mut_data()?)?;
    Ok(())
}
