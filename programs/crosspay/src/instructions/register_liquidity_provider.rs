use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::CrossPayError;
use crate::constants::*;

/// Context for registering a new liquidity provider
#[derive(Accounts)]
pub struct RegisterLiquidityProvider<'info> {
    #[account(
        init,
        payer = authority,
        space = LiquidityProvider::LEN,
        seeds = [b"liquidity_provider", authority.key().as_ref()],
        bump
    )]
    pub liquidity_provider: Account<'info, LiquidityProvider>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Register as a liquidity provider
pub fn register_liquidity_provider(
    ctx: Context<RegisterLiquidityProvider>,
    location: String,
    exchange_rate: u64,
) -> Result<()> {
    require!(location.len() <= MAX_LOCATION_LEN, CrossPayError::InvalidLocation);
    require!(exchange_rate > 0, CrossPayError::InvalidAmount);

    let liquidity_provider = &mut ctx.accounts.liquidity_provider;
    let clock = Clock::get()?;

    liquidity_provider.authority = ctx.accounts.authority.key();
    liquidity_provider.location = location;
    liquidity_provider.exchange_rate = exchange_rate;
    liquidity_provider.available_liquidity = 0;
    liquidity_provider.total_volume = 0;
    liquidity_provider.completed_transactions = 0;
    liquidity_provider.trust_score = DEFAULT_TRUST_SCORE;
    liquidity_provider.is_active = true;
    liquidity_provider.created_at = clock.unix_timestamp;
    liquidity_provider.bump = ctx.bumps.liquidity_provider;

    msg!("Liquidity provider registered: {}", ctx.accounts.authority.key());

    Ok(())
}

/// Context for updating liquidity provider availability
#[derive(Accounts)]
pub struct UpdateProviderAvailability<'info> {
    #[account(
        mut,
        seeds = [b"liquidity_provider", authority.key().as_ref()],
        bump = liquidity_provider.bump,
        has_one = authority
    )]
    pub liquidity_provider: Account<'info, LiquidityProvider>,

    pub authority: Signer<'info>,
}

/// Update liquidity provider availability and status
pub fn update_provider_availability(
    ctx: Context<UpdateProviderAvailability>,
    available_liquidity: u64,
    is_active: bool,
) -> Result<()> {
    let liquidity_provider = &mut ctx.accounts.liquidity_provider;

    liquidity_provider.available_liquidity = available_liquidity;
    liquidity_provider.is_active = is_active;

    msg!(
        "Provider availability updated - Liquidity: {}, Active: {}",
        available_liquidity,
        is_active
    );

    Ok(())
}