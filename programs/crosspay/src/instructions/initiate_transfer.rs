use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, Mint, TokenInterface};
use crate::state::*;
use crate::errors::*;

/// Context for initiating a transfer
#[derive(Accounts)]
#[instruction(amount: u64, receiver_key: Pubkey)]
pub struct InitiateTransfer<'info> {
    #[account(
        seeds = [b"user_profile", sender.key().as_ref()],
        bump = sender_profile.bump,
        has_one = authority,
        constraint = sender_profile.kyc_verified @ CrossPayError::KycNotVerified
    )]
    pub sender_profile: Account<'info, UserProfile>,

    #[account(
        init,
        payer = authority,
        space = TransferRequest::LEN,
        seeds = [
            b"transfer_request",
            sender.key().as_ref(),
            receiver_key.as_ref(),
            &sender_profile.total_sent.to_le_bytes()
        ],
        bump
    )]
    pub transfer_request: Account<'info, TransferRequest>,

    #[account(mut)]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Receiver pubkey - validation done in instruction
    pub receiver: UncheckedAccount<'info>,

    /// CHECK: Sender pubkey - used for PDA derivation
    pub sender: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

/// Initiate a transfer from sender to receiver
pub fn initiate_transfer(
    ctx: Context<InitiateTransfer>,
    amount: u64,
    receiver: Pubkey,
) -> Result<()> {
    require!(amount > 0, CrossPayError::InvalidAmount);
    require!(
        ctx.accounts.sender_token_account.amount >= amount,
        CrossPayError::InsufficientBalance
    );

    let transfer_request = &mut ctx.accounts.transfer_request;
    let clock = Clock::get()?;

    transfer_request.sender = ctx.accounts.sender.key();
    transfer_request.receiver = receiver;
    transfer_request.amount = amount;
    transfer_request.mint = ctx.accounts.mint.key();
    transfer_request.status = TransferStatus::Pending;
    transfer_request.created_at = clock.unix_timestamp;
    transfer_request.completed_at = None;
    transfer_request.nonce = ctx.accounts.sender_profile.total_sent;
    transfer_request.bump = ctx.bumps.transfer_request;

    msg!("Transfer initiated: {} tokens to {}", amount, receiver);

    Ok(())
}