use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh0_10::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::collections::HashMap;
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigureArgs {
    /// Contract admin
    pub authority: Pubkey,
    pub charge_addr: Pubkey,
    pub round: String,
    pub start_time: u64,
    pub total_reward: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigureData {
    /// Contract admin
    pub authority: Pubkey,
    pub charge_addr: Pubkey,
    pub token: Pubkey,
    pub round: u64,
    pub total_reward: u64,
    pub allocated: u64,
    pub target: u64,
    pub start_time: u64,
    pub total_shots: u64,
    pub match1: u64,
    pub match2: u64,
    pub match3: u64,
    pub match4: u64,
    pub match5: u64,
    pub match6: u64,
    pub closed: bool,
}

impl ConfigureData {
    pub const LEN: usize = 32 * 3 + 32 + 8 * 12  + 1;

    pub fn from_account_info(a: &AccountInfo) -> Result<ConfigureData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}


#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct UserData {
    pub shots: HashMap<[u8;6], u64>,
    pub total_shots: u64,
    pub round: u64,
    pub reward: u64,
    pub claimed: bool,
    pub match1: u64,
    pub match2: u64,
    pub match3: u64,
    pub match4: u64,
    pub match5: u64,
    pub match6: u64,
}

impl UserData {
    pub const LEN: usize = 200 + 1 + 8 * 9;

    pub fn from_account_info(a: &AccountInfo) -> Result<UserData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct BuyTicketsArgs {
    pub num: u64
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct ClaimArgs {
    pub round: u64
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct ClearArgs {
    pub amt: u64
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct RoundData {
    pub round: u64,
}

impl RoundData {
    pub const LEN: usize = 8;

    pub fn from_account_info(a: &AccountInfo) -> Result<RoundData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

