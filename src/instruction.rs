use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
    sysvar::rent,
};

use crate::state::*;

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AppInstruction {
    Configure(ConfigureArgs),
    BuyTickets(BuyTicketsArgs),
    CloseRound(),
    Claim(ClaimArgs),
    Clear(ClearArgs)
}

pub fn configure(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    mint_info: &Pubkey,
    mint_vault: &Pubkey,
    transfer_auth: &Pubkey,
    args: ConfigureArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*mint_info, false),
        AccountMeta::new(*mint_vault, false),
        AccountMeta::new(*transfer_auth, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Configure(args).try_to_vec().unwrap(),
    })
}

pub fn buy(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    user_info: &Pubkey, 
    charge_info: &Pubkey, 
    args: BuyTicketsArgs
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*user_info, false), 
        AccountMeta::new(*charge_info, false), 
        AccountMeta::new_readonly(rent::id(), false),  
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::BuyTickets(args).try_to_vec().unwrap(),
    })
}

pub fn close(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    new_config_info: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*new_config_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::CloseRound().try_to_vec().unwrap(),
    })
}

pub fn claim(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    mint_info: &Pubkey,
    user_info: &Pubkey, 
    mint_vault: &Pubkey,
    transfer_auth: &Pubkey,
    token_account: &Pubkey, 
    args: ClaimArgs
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*mint_info, false),
        AccountMeta::new(*user_info, false), 
        AccountMeta::new(*mint_vault, false), 
        AccountMeta::new(*transfer_auth, false), 
        AccountMeta::new(*token_account, false), 
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(rent::id(), false),  
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Claim(args).try_to_vec().unwrap(),
    })
}

pub fn clear(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    mint_info: &Pubkey,
    mint_vault: &Pubkey,
    transfer_auth: &Pubkey,
    token_account: &Pubkey, 
    args: ClearArgs
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*mint_info, false), 
        AccountMeta::new(*mint_vault, false), 
        AccountMeta::new(*transfer_auth, false), 
        AccountMeta::new(*token_account, false), 
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(rent::id(), false),  
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Clear(args).try_to_vec().unwrap(),
    })
}