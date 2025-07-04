use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserProfile {
    pub user_id: u32,
    pub username: String,
    // max 32 chars
    pub email: String,
    // max 64 chars
    pub balance: u64,
    pub reputation: u32,
    pub is_verified: bool,
    pub created_at: i64,
    pub last_login: i64,
    pub preferences: UserPreferences,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserPreferences {
    pub theme: Theme,
    pub language: Language,
    pub notifications: bool,
    pub privacy_level: u8,
    // 0-5 scale
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum Theme {
    Light,
    // 0
    Dark,
    // 1
    Auto,
    // 2
}
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum Language {
    English,
    // 0
    Spanish,
    // 1
    French,
    // 2
    German,
    // 3
}
// Cada enum ocupa solo 1 byte en lugar de String

impl UserProfile {
    //Cálculo de espacio máximo
    pub const MAX_SIZE: usize = 4 +         // user_id (u32)
    4 + 32 +    // username (String max 32)
    4 + 64 +    // email (String max 64)
    8 +         // balance (u64)
    4 +         // reputation (u32)
    1 +         // is_verified (bool)
    8 +         // created_at (i64)
    8 +         // last_login (i64)
    UserPreferences::MAX_SIZE; // preferences (UserPreferences)

    pub fn new(user_id: u32, username: String, email: String) -> Self {
        Self {
            user_id,
            username,
            email,
            balance: 0,
            reputation: 0,
            is_verified: false,
            created_at: 0,
            last_login: 0,
            preferences: UserPreferences::default(),
        }
    }
}

impl UserPreferences {
    pub const MAX_SIZE: usize = 1 + // theme (Theme enum 1 byte)
    1 + // language (Language enum 1 byte)
    1 + // notifications (bool)
    1; // privacy_level (u8)
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            language: Language::English,
            notifications: true,
            privacy_level: 3,
        }
    }
}
