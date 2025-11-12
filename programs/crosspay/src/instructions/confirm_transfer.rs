use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::CrossPayError;
use crate::state::*;
use crate::constants::*;

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

    // #[account(
    //     mut,
    //     seeds = [b"platform_fee", transfer_request.mint.as_ref()],
    //     bump,
    //     constraint = platform_fee_account.mint == transfer_request.mint
    // )]
    // pub platform_fee_account: Account<'info, TokenAccount>,

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

    // Validate sender has sufficient balance
    require!(
        ctx.accounts.sender_token_account.amount >= transfer_request.amount,
        CrossPayError::InsufficientBalance
    );

    // Validate fee calculations are correct
    let expected_platform_fee = calculate_platform_fee(transfer_request.amount);
    let expected_net_amount = calculate_net_amount(transfer_request.amount);

    require!(
        transfer_request.platform_fee == expected_platform_fee,
        CrossPayError::InvalidFeeCalculation
    );

    require!(
        transfer_request.net_amount == expected_net_amount,
        CrossPayError::InvalidFeeCalculation
    );

    // Verify amounts add up correctly
    require!(
        transfer_request.net_amount.checked_add(transfer_request.platform_fee)
            .unwrap_or(0) == transfer_request.amount,
        CrossPayError::InvalidFeeCalculation
    );

    // Transfer net amount to receiver
    let receiver_cpi_accounts = Transfer {
        from: ctx.accounts.sender_token_account.to_account_info(),
        to: ctx.accounts.receiver_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let receiver_cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        receiver_cpi_accounts
    );
    token::transfer(receiver_cpi_ctx, transfer_request.net_amount)?;

    // // Transfer platform fee to fee account (only if fee > 0)
    // if transfer_request.platform_fee > 0 {
    //     let fee_cpi_accounts = Transfer {
    //         from: ctx.accounts.sender_token_account.to_account_info(),
    //         to: ctx.accounts.platform_fee_account.to_account_info(),
    //         authority: ctx.accounts.authority.to_account_info(),
    //     };
    //     let fee_cpi_ctx = CpiContext::new(
    //         ctx.accounts.token_program.to_account_info(),
    //         fee_cpi_accounts
    //     );
    //     token::transfer(fee_cpi_ctx, transfer_request.platform_fee)?;
    // }

    // Update transfer status
    transfer_request.status = TransferStatus::Completed;
    transfer_request.completed_at = Some(clock.unix_timestamp);

    // Update user profiles:
    // - Sender tracks what they actually sent (gross amount)
    // - Receiver tracks what they actually received (net amount)
    ctx.accounts.sender_profile.total_sent = ctx.accounts.sender_profile.total_sent
        .checked_add(transfer_request.amount)  // Gross amount sent
        .ok_or(CrossPayError::ArithmeticOverflow)?;

    ctx.accounts.receiver_profile.total_received = ctx.accounts.receiver_profile.total_received
        .checked_add(transfer_request.net_amount)  // Net amount received
        .ok_or(CrossPayError::ArithmeticOverflow)?;

    msg!(
        "Transfer completed: {} tokens sent, {} received (fee: {})",
        transfer_request.amount,
        transfer_request.net_amount,
        transfer_request.platform_fee
    );

    Ok(())
}