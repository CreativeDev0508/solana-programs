use solana_program::{
    account_info::{ AccountInfo, next_account_info }, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};


entrypoint!(process_instruction);


fn print_account_info(account_info: AccountInfo) {
    msg!("  key: {:?}", account_info.key);
    msg!("  is_signer: {:?}", account_info.is_signer);
    msg!("  is_writable: {:?}", account_info.is_writable);
    msg!("  lamports: {:?}", account_info.lamports());
    msg!("  data: {:?}", account_info.data);
    msg!("  owner: {:?}", account_info.owner);
    msg!("  executable: {:?}", account_info.executable);
    msg!("  rent_epoch: {:?}", account_info.rent_epoch);
}


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    // You can verify the program ID from the instruction is in fact 
    //      the program ID of your program.
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId)
    };
    
    // You can verify the list has the correct number of accounts.
    // This error will get thrown by default if you 
    //      try to reach past the end of the iter.
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // Accounts passed in a vector must be in the expected order.
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.
    
    msg!("New account: {}", account_to_create.key);
    print_account_info(account_to_create.clone());
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized)
    };
    // (Create account...)

    // You can also make sure an account has been initialized.
    msg!("Account to change: {}", account_to_change.key);
    print_account_info(account_to_change.clone());
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount)
    };

    // If we want to modify an account's data, it must be owned by our program.
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };

    // You can also check pubkeys against constants.
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId)
    };

    Ok(())
}