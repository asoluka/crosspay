// ========================================
// PDA SEEDS
// ========================================

/// Seed for UserProfile PDA
pub const USER_PROFILE_SEED: &[u8] = b"user_profile";

/// Seed for TransferRequest PDA
pub const TRANSFER_REQUEST_SEED: &[u8] = b"transfer_request";

/// Seed for WithdrawalRequest PDA
pub const WITHDRAWAL_REQUEST_SEED: &[u8] = b"withdrawal_request";

/// Seed for LiquidityProvider PDA
pub const LIQUIDITY_PROVIDER_SEED: &[u8] = b"liquidity_provider";

// ========================================
// TRUST SCORE CONFIGURATION
// ========================================

/// Default trust score for new liquidity providers (70.00%)
/// Score is out of 10000 for 2 decimal precision
/// 7000 / 10000 = 0.70 = 70%
pub const DEFAULT_TRUST_SCORE: u16 = 7000;

/// Minimum trust score required for a provider to remain active (50.00%)
pub const MIN_TRUST_SCORE: u16 = 5000;

/// Maximum trust score (100.00%)
pub const MAX_TRUST_SCORE: u16 = 10000;

// ========================================
// STRING LENGTH LIMITS
// ========================================

/// Maximum length for country code (e.g., "USA", "NGA")
pub const MAX_COUNTRY_CODE_LEN: usize = 3;

/// Maximum length for location string (e.g., "Lagos, Nigeria")
pub const MAX_LOCATION_LEN: usize = 50;

// ========================================
// FEE CONFIGURATION
// ========================================

/// Platform fee in basis points (0.5% = 50 basis points)
/// 1 basis point = 0.01%
pub const PLATFORM_FEE_BPS: u16 = 50;

/// Fee divisor for basis points calculation
/// To calculate fee: (amount * fee_bps) / BASIS_POINTS_DIVISOR
pub const BASIS_POINTS_DIVISOR: u64 = 10_000;

// ========================================
// HELPER FUNCTIONS
// ========================================

/// Calculate platform fee for a given amount
/// Returns the fee amount in the same units as the input
pub fn calculate_platform_fee(amount: u64) -> u64 {
    amount
        .checked_mul(PLATFORM_FEE_BPS as u64)
        .unwrap_or(0)
        .checked_div(BASIS_POINTS_DIVISOR)
        .unwrap_or(0)
}

/// Calculate net amount after deducting platform fee
pub fn calculate_net_amount(amount: u64) -> u64 {
    amount.saturating_sub(calculate_platform_fee(amount))
}

/// Validate country code length
pub fn is_valid_country_code(code: &str) -> bool {
    !code.is_empty() && code.len() <= MAX_COUNTRY_CODE_LEN
}

/// Validate location string length
pub fn is_valid_location(location: &str) -> bool {
    !location.is_empty() && location.len() <= MAX_LOCATION_LEN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_platform_fee() {
        // Test with 1000 USDC (1000 * 10^6)
        let amount = 1_000_000_000;
        let fee = calculate_platform_fee(amount);
        // 0.5% of 1000 = 5 USDC = 5 * 10^6 = 5,000,000
        assert_eq!(fee, 5_000_000);
    }

    #[test]
    fn test_calculate_net_amount() {
        let amount = 1_000_000_000;
        let net = calculate_net_amount(amount);
        // 1000 - 5 = 995 USDC
        assert_eq!(net, 995_000_000);
    }

    #[test]
    fn test_country_code_validation() {
        assert!(is_valid_country_code("USA"));
        assert!(is_valid_country_code("NG"));
        assert!(!is_valid_country_code(""));
        assert!(!is_valid_country_code("USAA"));
    }

    #[test]
    fn test_location_validation() {
        assert!(is_valid_location("Lagos"));
        assert!(is_valid_location("Lagos, Nigeria"));
        assert!(!is_valid_location(""));
        let long_location = "a".repeat(51);
        assert!(!is_valid_location(&long_location));
    }
}
