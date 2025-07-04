use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use borsh::{BorshDeserialize, BorshSerialize};

use crate::{
    error::UserManagerError,
    instruction::UserInstruction,
    state::{UserPreferences, UserProfile},
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = UserInstruction::try_from_slice(instruction_data)?;

        match instruction {
            UserInstruction::CreateProfile { username, email } => {
                Self::process_create_profile(program_id, accounts, username, email);
            }
            UserInstruction::GetProfile => {
                Self::process_get_profile(program_id, accounts);
            }
            UserInstruction::UpdateProfile {
                username,
                email,
                preferences,
            } => {
                Self::process_update_profile(program_id, accounts, username, email, preferences);
            }
            UserInstruction::UpdateBalance { amount, is_deposit } => {
                Self::process_update_balance(program_id, accounts, amount, is_deposit);
            }
            UserInstruction::DeleteProfile => {
                Self::process_delete_profile(program_id, accounts);
            }
        }

        Ok(())
    }

    pub fn process_create_profile(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        username: String,
        email: String,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account = next_account_info(account_info_iter)?;
        let payer_account = next_account_info(account_info_iter)?;

        //validations
        //username
        if username.len() > 32 {
            return Err(UserManagerError::UsernameTooLong.into());
        }
        //email
        if email.len() > 64 {
            return Err(UserManagerError::EmailTooLong.into());
        }
        //check is email is valid
        if !email.contains("@") {
            return Err(UserManagerError::InvalidEmail.into());
        }
        //check if payer is signer
        if !payer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        if user_account.data.borrow().len() != 0 {
            return Err(UserManagerError::AlreadyInitialized.into());
        }

        let rent: Rent = Rent::get()?;
        let required_lamports = rent.minimum_balance(UserProfile::MAX_SIZE);

        let create_account_ix = system_instruction::create_account(
            payer_account.key,
            user_account.key,
            required_lamports,
            UserProfile::MAX_SIZE as u64,
            program_id,
        );

        invoke(
            &create_account_ix,
            &[payer_account.clone(), user_account.clone()],
        )?;

        let clock: Clock = Clock::get()?;
        let current_timestamp: i64 = clock.unix_timestamp;
        let user_id: u32 = current_timestamp as u32;
        let user_profile: UserProfile = UserProfile::new(user_id, username, email);
        let mut updated_profile = user_profile;
        updated_profile.created_at = current_timestamp;
        updated_profile.last_login = current_timestamp;

        updated_profile.serialize(&mut &mut user_account.data.borrow_mut()[..])?;
        msg!("User created successfully. ID: {}", user_id);
        Ok(())
    }

    pub fn process_get_profile(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account = next_account_info(account_info_iter)?;

        //validations

        if user_account.data.borrow().len() != 0 {
            return Err(UserManagerError::AlreadyInitialized.into());
        }

        if user_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId.into());
        }
        let user_profile: UserProfile = UserProfile::try_from_slice(&user_account.data.borrow())?;

        msg!("User Profile: {:?}", user_profile);
        Ok(())
    }

    pub fn process_update_profile(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        username: Option<String>,
        email: Option<String>,
        preferences: Option<UserPreferences>,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account = next_account_info(account_info_iter)?;

        //validations
        //check if user is signer
        if !user_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        if user_account.data.borrow().len() != 0 {
            return Err(UserManagerError::AlreadyInitialized.into());
        }

        if user_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId.into());
        }

        let mut user_profile: UserProfile =
            UserProfile::try_from_slice(&user_account.data.borrow())?;

        if let Some(new_username) = username {
            if new_username.len() > 32 {
                return Err(UserManagerError::UsernameTooLong.into());
            }
            user_profile.username = new_username;
            msg!(
                "Username updated successfully to {}.",
                user_profile.username
            );
        }

        if let Some(new_email) = email {
            if new_email.len() > 64 {
                return Err(UserManagerError::EmailTooLong.into());
            }
            if !new_email.contains("@") {
                return Err(UserManagerError::InvalidEmail.into());
            }
            user_profile.email = new_email;
            msg!("Email updated successfully to {}.", user_profile.email);
        }

        if let Some(new_preferences) = preferences {
            if new_preferences.privacy_level > 5 {
                return Err(UserManagerError::InvalidPrivacyLevel.into());
            }
            user_profile.preferences = new_preferences;
            msg!("Preferences updated successfully.");
        }

        let clock: Clock = Clock::get()?;
        let current_timestamp: i64 = clock.unix_timestamp;
        user_profile.last_login = current_timestamp;
        user_profile.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

        msg!("User preferences updated successfully.");
        Ok(())
    }

    pub fn process_update_balance(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        is_deposit: bool,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account = next_account_info(account_info_iter)?;

        //validations
        //check if user is signer
        if !user_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        if user_account.data.borrow().len() != 0 {
            return Err(UserManagerError::AlreadyInitialized.into());
        }

        if user_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId.into());
        }

        let mut user_profile: UserProfile =
            UserProfile::try_from_slice(&user_account.data.borrow())?;

        //check if deposit
        if is_deposit {
            user_profile.balance = user_profile.balance.saturating_add(amount);
            msg!(
                "Deposited {} tokens successful. New balance: {}",
                amount,
                user_profile.balance
            );
        } else {
            if user_profile.balance < amount {
                return Err(UserManagerError::InsufficientFunds.into());
            }
            user_profile.balance = user_profile.balance.saturating_sub(amount);
            msg!(
                "Withdrew {} tokens successful. New balance: {}",
                amount,
                user_profile.balance
            );
        }

        let clock: Clock = Clock::get()?;
        let current_timestamp: i64 = clock.unix_timestamp;
        user_profile.last_login = current_timestamp;
        user_profile.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

        msg!("User balance updated successfully.");
        Ok(())
    }

    pub fn process_delete_profile(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account = next_account_info(account_info_iter)?;
        let destination_account = next_account_info(account_info_iter)?;

        if !user_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        if user_account.data.borrow().len() != 0 {
            return Err(UserManagerError::AlreadyInitialized.into());
        }

        if user_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId.into());
        }

        let mut user_profile: UserProfile =
            UserProfile::try_from_slice(&user_account.data.borrow())?;
        msg!("Eliminating user: {}", user_profile.username);

        let dest_starting_lamports = destination_account.lamports();
        **destination_account.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(user_account.lamports())
            .ok_or(ProgramError::ArithmeticOverflow)?;
        **user_account.lamports.borrow_mut() = 0;

        let mut data = user_account.data.borrow_mut();
        data.fill(0);
        msg!("User profile deleted successfully.");
        Ok(())
    }
}
