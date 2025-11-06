use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::CrossPayError;
use crate::state::*;

/// Context for confirming and executing a transfer
#[derive(Accounts)]
pub struct ConfirmTransfer<'info> {
    #[account(
        mut,
        seeds = [
            b"transfer_request",
            transfer_request.sender.as_ref(),
            transfer_request.receiver.as_ref(),
            &transfer_request.nonce.to_le_bytes()
        ],
        bump = transfer_request.bump,
        constraint = transfer_request.status == TransferStatus::Pending @ CrossPayError::InvalidTransferStatus
    )]
    pub transfer_request: Account<'info, TransferRequest>,

    #[account(
        mut,
        seeds = [b"user_profile", sender.key().as_ref()],
        bump = sender_profile.bump,
        has_one = authority
    )]
    pub sender_profile: Account<'info, UserProfile>,

    #[account(
        mut,
        seeds = [b"user_profile", transfer_request.receiver.as_ref()],
        bump = receiver_profile.bump
    )]
    pub receiver_profile: Account<'info, UserProfile>,

    #[account(
        mut,
        constraint = sender_token_account.owner == authority.key(),
        constraint = sender_token_account.mint == transfer_request.mint
    )]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = receiver_token_account.owner == transfer_request.receiver,
        constraint = receiver_token_account.mint == transfer_request.mint
    )]
    pub receiver_token_account: Account<'info, TokenAccount>,

    /// CHECK: Validated via seeds in sender_profile
    pub sender: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

/// Confirm and execute the transfer
pub fn confirm_transfer(ctx: Context<ConfirmTransfer>) -> Result<()> {
    let transfer_request = &mut ctx.accounts.transfer_request;
    let clock = Clock::get()?;

    // Transfer tokens via CPI
    let cpi_accounts = Transfer {
        from: ctx.accounts.sender_token_account.to_account_info(),
        to: ctx.accounts.receiver_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, transfer_request.amount)?;

    // Update transfer status
    transfer_request.status = TransferStatus::Completed;
    transfer_request.completed_at = Some(clock.unix_timestamp);

    // Update user profiles
    ctx.accounts.sender_profile.total_sent += transfer_request.amount;
    ctx.accounts.receiver_profile.total_received += transfer_request.amount;

    msg!("Transfer completed: {} tokens", transfer_request.amount);

    Ok(())
}
