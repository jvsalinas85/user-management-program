/// Imports the `UserPreferences` struct from the `state` module, which likely represents user-configurable settings or preferences within the application.
use crate::state::UserPreferences;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum UserInstruction {
    CreateProfile {
        username: String,
        email: String,
    },
    GetProfile,
    UpdateProfile {
        username: Option<String>,
        email: Option<String>,
        preferences: Option<UserPreferences>,
    },
    UpdateBalance {
        amount: u64,
        is_deposit: bool,
    },
    DeleteProfile,
}
