use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    account_info::AccountInfo
};


entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // Your program logic goes here
    msg!("Hello World");

    Ok(())
}