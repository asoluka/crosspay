use anchor_lang::prelude::*;

#[account]
pub struct LiquidityProvider {
    pub authority: Pubkey,           // 32
    pub location: String,            // 4 + max 50 = 54
    pub exchange_rate: u64,          // 8 (scaled by 10^6 for decimals)
    pub available_liquidity: u64,    // 8
    pub total_volume: u64,           // 8
    pub completed_transactions: u64, // 8
    pub trust_score: u16,            // 2 (out of 10000 for 2 decimals)
    pub is_active: bool,             // 1
    pub created_at: i64,             // 8
    pub bump: u8,                    // 1
}

impl LiquidityProvider {
    pub const LEN: usize = 8 + 32 + 54 + 8 + 8 + 8 + 8 + 2 + 1 + 8 + 1;
}