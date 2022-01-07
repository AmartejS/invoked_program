
  
use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    instruction::{AccountMeta, Instruction},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey, pubkey::Pubkey, 
    program::invoke
};

//use core::convert::From;






#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct instructiondata {
   
    pub input_a: u32,
    pub input_b: u32,
    pub program_id: String,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    _instruction_data: &[u8],
     
) -> ProgramResult {
    msg!("Program Invoked");
   

    Ok(())
  }
