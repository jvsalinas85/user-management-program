use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum UserManagerError {
    // Invalid Instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not initialized account")]
    NotInitialized,
    #[error("Account already initialized")]
    AlreadyInitialized,
    #[error("Username too long. Max 32 characters")]
    UsernameTooLong,
    #[error("Email too long. Max 64 characters")]
    EmailTooLong,
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Unauthorized operation")]
    Unauthorized,
    #[error("Invalid privacy level")]
    InvalidPrivacyLevel,
    #[error("Insufficient funds")]
    InsufficientFunds,
}

impl From<UserManagerError> for ProgramError {
    fn from(e: UserManagerError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
