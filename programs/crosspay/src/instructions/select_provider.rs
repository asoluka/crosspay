use anchor_lang::prelude::*;

use crate::errors::CrossPayError;
use crate::state::*;

/// Context for selecting a liquidity provider
#[derive(Accounts)]
#[instruction(provider_key: Pubkey)]
pub struct SelectProvider<'info> {
    #[account(
        mut,
        seeds = [
            b"withdrawal_request",
            withdrawal_request.freelancer.as_ref(),
            &withdrawal_request.nonce.to_le_bytes()
        ],
        bump = withdrawal_request.bump,
        constraint = withdrawal_request.status == WithdrawalStatus::Pending @ CrossPayError::InvalidWithdrawalStatus,
        has_one = freelancer
    )]
    pub withdrawal_request: Account<'info, WithdrawalRequest>,

    #[account(
        seeds = [b"liquidity_provider", provider_key.as_ref()],
        bump = liquidity_provider.bump,
        constraint = liquidity_provider.is_active @ CrossPayError::ProviderNotActive,
        constraint = liquidity_provider.available_liquidity >= withdrawal_request.amount @ CrossPayError::InsufficientLiquidity
    )]
    pub liquidity_provider: Account<'info, LiquidityProvider>,

    pub freelancer: Signer<'info>,
}

/// Select a liquidity provider for the withdrawal
pub fn select_provider(ctx: Context<SelectProvider>, provider_key: Pubkey) -> Result<()> {
    let withdrawal_request = &mut ctx.accounts.withdrawal_request;

    require!(
        withdrawal_request.selected_provider.is_none(),
        CrossPayError::ProviderAlreadySelected
    );

    withdrawal_request.selected_provider = Some(provider_key);
    withdrawal_request.status = WithdrawalStatus::ProviderSelected;

    msg!("Liquidity provider selected: {}", provider_key);

    Ok(())
}
