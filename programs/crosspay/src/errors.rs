use anchor_lang::prelude::*;

#[error_code]
pub enum CrossPayError {
    #[msg("KYC verification is required to perform this action")]
    KycNotVerified, // 6000

    #[msg("Invalid transfer amount - must be greater than 0")]
    InvalidAmount, // 6001

    #[msg("Insufficient token balance")]
    InsufficientBalance, // 6002

    #[msg("Invalid country code - must be 3 characters or less")]
    InvalidCountryCode, // 6003

    #[msg("Invalid transfer status for this operation")]
    InvalidTransferStatus, // 6004

    #[msg("Invalid withdrawal status for this operation")]
    InvalidWithdrawalStatus, // 6005

    #[msg("Liquidity provider is not active")]
    ProviderNotActive, // 6006

    #[msg("Insufficient liquidity available from provider")]
    InsufficientLiquidity, // 6007

    #[msg("Unauthorized action - you don't have permission")]
    Unauthorized, // 6008

    #[msg("Transfer request not found")]
    TransferNotFound, // 6009

    #[msg("Withdrawal request not found")]
    WithdrawalNotFound, // 6010

    #[msg("Provider already selected for this withdrawal - cannot change")]
    ProviderAlreadySelected, // 6011

    #[msg("Invalid location string - maximum 50 characters allowed")]
    InvalidLocation, // 6012
    
    #[msg("Invalid fee calculation")]
    InvalidFeeCalculation,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

}
