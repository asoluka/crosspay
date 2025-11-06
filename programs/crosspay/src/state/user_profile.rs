use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,    // 32
    pub role: UserRole,       // 1 + 1 (enum discriminator)
    pub kyc_verified: bool,   // 1
    pub kyc_hash: [u8; 32],   // 32
    pub country_code: String, // 4 + max 3 = 7
    pub created_at: i64,      // 8
    pub total_sent: u64,      // 8
    pub total_received: u64,  // 8
    pub bump: u8,             // 1
}

impl UserProfile {
    pub const LEN: usize = 8 + 32 + 2 + 1 + 32 + 7 + 8 + 8 + 8 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    Sender,
    Receiver,
    Both,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Receiver
    }
}
