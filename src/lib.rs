#![allow(unexpected_cfgs)]
// src/lib.rs
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
use processor::Processor;

#[cfg(not(feature = "no-entrypoint"))]

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}
