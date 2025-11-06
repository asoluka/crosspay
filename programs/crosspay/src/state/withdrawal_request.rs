use anchor_lang::prelude::*;

#[account]
pub struct WithdrawalRequest {
    pub freelancer: Pubkey,                // 32
    pub amount: u64,                       // 8
    pub mint: Pubkey,                      // 32
    pub payout_method: PayoutMethod,       // 1 + 1
    pub selected_provider: Option<Pubkey>, // 1 + 32
    pub status: WithdrawalStatus,          // 1 + 1
    pub created_at: i64,                   // 8
    pub completed_at: Option<i64>,         // 1 + 8
    pub nonce: u64,                        // 8
    pub bump: u8,                          // 1
}

impl WithdrawalRequest {
    pub const LEN: usize = 8 + 32 + 8 + 32 + 2 + 33 + 2 + 8 + 9 + 8 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PayoutMethod {
    MobileMoney,
    BankTransfer,
    Cash,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum WithdrawalStatus {
    Pending,
    ProviderSelected,
    AwaitingConfirmation,
    Completed,
    Failed,
}
