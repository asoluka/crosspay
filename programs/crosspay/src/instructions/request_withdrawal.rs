use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};
use crate::state::*;
use crate::errors::CrossPayError;

/// Context for requesting a withdrawal
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct RequestWithdrawal<'info> {
    #[account(
        seeds = [b"user_profile", freelancer.key().as_ref()],
        bump = freelancer_profile.bump,
        has_one = authority
    )]
    pub freelancer_profile: Account<'info, UserProfile>,

    #[account(
        init,
        payer = authority,
        space = WithdrawalRequest::LEN,
        seeds = [
            b"withdrawal_request",
            freelancer.key().as_ref(),
            &freelancer_profile.total_received.to_le_bytes()
        ],
        bump
    )]
    pub withdrawal_request: Account<'info, WithdrawalRequest>,

    #[account(
        mut,
        constraint = freelancer_token_account.owner == authority.key(),
        constraint = freelancer_token_account.amount >= amount @ CrossPayError::InsufficientBalance
    )]
    pub freelancer_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Freelancer pubkey - used for PDA derivation
    pub freelancer: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

/// Request a withdrawal to local currency
pub fn request_withdrawal(
    ctx: Context<RequestWithdrawal>,
    amount: u64,
    payout_method: PayoutMethod,
) -> Result<()> {
    require!(amount > 0, CrossPayError::InvalidAmount);

    let withdrawal_request = &mut ctx.accounts.withdrawal_request;
    let clock = Clock::get()?;

    withdrawal_request.freelancer = ctx.accounts.freelancer.key();
    withdrawal_request.amount = amount;
    withdrawal_request.mint = ctx.accounts.freelancer_token_account.mint;
    withdrawal_request.payout_method = payout_method;
    withdrawal_request.selected_provider = None;
    withdrawal_request.status = WithdrawalStatus::Pending;
    withdrawal_request.created_at = clock.unix_timestamp;
    withdrawal_request.completed_at = None;
    withdrawal_request.nonce = ctx.accounts.freelancer_profile.total_received;
    withdrawal_request.bump = ctx.bumps.withdrawal_request;

    msg!("Withdrawal requested: {} tokens", amount);

    Ok(())
}