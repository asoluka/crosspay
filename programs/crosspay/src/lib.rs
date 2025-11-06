use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;
use state::{PayoutMethod, UserRole};

declare_id!("4fy5wximsVYsVYwLp5VrgjqfUq8NyEXG1nisKuwkS8Vq");

#[program]
pub mod crosspay {
    use super::*;

    /// Initialize a user profile (sender or receiver)
    pub fn initialize_user(
        ctx: Context<InitializeUser>,
        role: UserRole,
        country_code: String,
    ) -> Result<()> {
        instructions::initialize_user(ctx, role, country_code)
    }

    /// Update KYC verification status
    pub fn update_kyc_status(
        ctx: Context<UpdateKycStatus>,
        kyc_verified: bool,
        kyc_hash: [u8; 32],
    ) -> Result<()> {
        instructions::update_kyc_status(ctx, kyc_verified, kyc_hash)
    }

    /// Initiate a transfer request
    pub fn initiate_transfer(
        ctx: Context<InitiateTransfer>,
        amount: u64,
        receiver: Pubkey,
    ) -> Result<()> {
        instructions::initiate_transfer(ctx, amount, receiver)
    }

    /// Confirm and execute the transfer
    pub fn confirm_transfer(ctx: Context<ConfirmTransfer>) -> Result<()> {
        instructions::confirm_transfer(ctx)
    }

    /// Register as a liquidity provider
    pub fn register_liquidity_provider(
        ctx: Context<RegisterLiquidityProvider>,
        location: String,
        exchange_rate: u64,
    ) -> Result<()> {
        instructions::register_liquidity_provider(ctx, location, exchange_rate)
    }

    /// Update liquidity provider availability
    pub fn update_provider_availability(
        ctx: Context<UpdateProviderAvailability>,
        available_liquidity: u64,
        is_active: bool,
    ) -> Result<()> {
        instructions::update_provider_availability(ctx, available_liquidity, is_active)
    }

    /// Request a withdrawal to local currency
    pub fn request_withdrawal(
        ctx: Context<RequestWithdrawal>,
        amount: u64,
        payout_method: PayoutMethod,
    ) -> Result<()> {
        instructions::request_withdrawal(ctx, amount, payout_method)
    }

    /// Select a liquidity provider for withdrawal
    pub fn select_provider(ctx: Context<SelectProvider>, provider_key: Pubkey) -> Result<()> {
        instructions::select_provider(ctx, provider_key)
    }

    /// Finalize withdrawal after fiat received
    pub fn finalize_withdrawal(ctx: Context<FinalizeWithdrawal>) -> Result<()> {
        instructions::finalize_withdrawal(ctx)
    }
}
