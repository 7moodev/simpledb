use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,
}
entrypoint!(process_instruction);
// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    println!("Welcome to the program");
    // Iterating accounts
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account = next_account_info(accounts_iter)?;
    let _program_id_account = next_account_info(accounts_iter)?;
    let _system_program_account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("The account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Deserialize instruction data
    let user_data = UserData::try_from_slice(instruction_data)?;
    msg!("Account Data: {:?}", account.data.borrow());
    msg!("Storing user data");
    // Serialize user data and store it in the account 
    user_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    msg!("Account Data: {:?}", account.data.borrow());
    Ok(())

}

