use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::state::*;
use crate::errors::*;

/// Context for initializing a new user profile
#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = authority,
        space = UserProfile::LEN,
        seeds = [b"user_profile", authority.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Initialize a new user profile
pub fn initialize_user(
    ctx: Context<InitializeUser>,
    role: UserRole,
    country_code: String,
) -> Result<()> {
    require!(country_code.len() <= 3, CrossPayError::InvalidCountryCode);

    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    user_profile.authority = ctx.accounts.authority.key();
    user_profile.role = role;
    user_profile.kyc_verified = false;
    user_profile.kyc_hash = [0; 32];
    user_profile.country_code = country_code;
    user_profile.created_at = clock.unix_timestamp;
    user_profile.total_sent = 0;
    user_profile.total_received = 0;
    user_profile.bump = ctx.bumps.user_profile;

    msg!("User profile initialized for: {}", ctx.accounts.authority.key());

    Ok(())
}

/// Context for updating KYC status
#[derive(Accounts)]
pub struct UpdateKycStatus<'info> {
    #[account(
        mut,
        seeds = [b"user_profile", authority.key().as_ref()],
        bump = user_profile.bump,
        has_one = authority
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub authority: Signer<'info>,
}

/// Update the KYC verification status for a user
pub fn update_kyc_status(
    ctx: Context<UpdateKycStatus>,
    kyc_verified: bool,
    kyc_hash: [u8; 32],
) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;

    user_profile.kyc_verified = kyc_verified;
    user_profile.kyc_hash = kyc_hash;

    msg!("KYC status updated for: {}", ctx.accounts.authority.key());

    Ok(())
}