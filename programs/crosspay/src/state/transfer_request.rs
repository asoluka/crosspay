use anchor_lang::prelude::*;

#[account]
pub struct TransferRequest {
    pub sender: Pubkey,              // 32
    pub receiver: Pubkey,            // 32
    pub amount: u64,                 // 8
    pub mint: Pubkey,                // 32 (stablecoin mint address)
    pub status: TransferStatus,      // 1 + 1
    pub created_at: i64,             // 8
    pub completed_at: Option<i64>,   // 1 + 8
    pub nonce: u64,                  // 8
    pub bump: u8,                    // 1
}

impl TransferRequest {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 32 + 2 + 8 + 9 + 8 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TransferStatus {
    Pending,
    Confirmed,
    Completed,
    Failed,
    Cancelled,
}