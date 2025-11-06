use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked, transfer_checked};
use crate::state::*;
use crate::errors::CrossPayError;

/// Context for finalizing a withdrawal
#[derive(Accounts)]
pub struct FinalizeWithdrawal<'info> {
    #[account(
        mut,
        seeds = [
            b"withdrawal_request",
            withdrawal_request.freelancer.as_ref(),
            &withdrawal_request.nonce.to_le_bytes()
        ],
        bump = withdrawal_request.bump,
        constraint = withdrawal_request.status == WithdrawalStatus::ProviderSelected @ CrossPayError::InvalidWithdrawalStatus,
        has_one = freelancer
    )]
    pub withdrawal_request: Account<'info, WithdrawalRequest>,

    #[account(
        mut,
        seeds = [
            b"liquidity_provider",
            withdrawal_request.selected_provider.unwrap().as_ref()
        ],
        bump = liquidity_provider.bump,
        constraint = liquidity_provider.authority == provider_authority.key()
    )]
    pub liquidity_provider: Account<'info, LiquidityProvider>,

    #[account(
        mut,
        constraint = freelancer_token_account.owner == freelancer.key(),
        constraint = freelancer_token_account.mint == withdrawal_request.mint
    )]
    pub freelancer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = provider_token_account.owner == provider_authority.key(),
        constraint = provider_token_account.mint == withdrawal_request.mint
    )]
    pub provider_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, anchor_spl::token_interface::Mint>,

    pub freelancer: Signer<'info>,

    #[account(mut)]
    pub provider_authority: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// Finalize the withdrawal after fiat is received
pub fn finalize_withdrawal(ctx: Context<FinalizeWithdrawal>) -> Result<()> {
    let withdrawal_request = &mut ctx.accounts.withdrawal_request;
    let liquidity_provider = &mut ctx.accounts.liquidity_provider;
    let clock = Clock::get()?;

    // Transfer tokens from freelancer to liquidity provider via CPI using transfer_checked
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.freelancer_token_account.to_account_info(),
        to: ctx.accounts.provider_token_account.to_account_info(),
        authority: ctx.accounts.freelancer.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(cpi_ctx, withdrawal_request.amount, ctx.accounts.mint.decimals)?;

    // Update withdrawal status
    withdrawal_request.status = WithdrawalStatus::Completed;
    withdrawal_request.completed_at = Some(clock.unix_timestamp);

    // Update liquidity provider stats
    liquidity_provider.total_volume += withdrawal_request.amount;
    liquidity_provider.completed_transactions += 1;
    liquidity_provider.available_liquidity = liquidity_provider
        .available_liquidity
        .saturating_sub(withdrawal_request.amount);

    msg!("Withdrawal finalized: {} tokens", withdrawal_request.amount);

    Ok(())
}