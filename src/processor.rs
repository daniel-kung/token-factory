use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::*;

pub mod configure;
pub use configure::*;

pub mod buy_tickets;
pub use buy_tickets::*;

pub mod close_round;
pub use close_round::*;

pub mod claim;
pub use claim::*;

pub mod clear;
pub use clear::*;


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = AppInstruction::try_from_slice(input)?;
    match instruction {
        AppInstruction::Configure(args) => {
            msg!("Instruction: Configure");
            process_configure(program_id, accounts, args)
        }
        AppInstruction::BuyTickets(args) => {
            msg!("Instruction: Buy Tickets");
            process_buy(program_id, accounts, args)
        }
        AppInstruction::CloseRound() => {
            msg!("Instruction: Start new round");
            process_close(program_id, accounts)
        }
        AppInstruction::Claim(args) => {
            msg!("Instruction: Claim");
            process_claim(program_id, accounts, args)
        }
        AppInstruction::Clear(args) => {
            msg!("Instruction: Clear");
            process_clear(program_id, accounts, args)
        }
        }
    }
