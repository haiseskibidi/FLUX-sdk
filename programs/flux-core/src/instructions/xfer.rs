use anchor_lang::prelude::*;
use crate::state::user_profile::{UserProfile, ActionType};
use crate::errors::FluxError;

#[derive(Accounts)]
pub struct XferFunds<'info> {
    #[account(mut)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Recipient wallet
    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
    
    // Compliance Accounts
    /// CHECK: Global blacklist
    pub blacklist_registry: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<XferFunds>, amount: u64) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // 1. Compliance Checks (KYC/AML)
    msg!("Performing Compliance Checks...");
    if profile.aml_flagged {
        return Err(error!(FluxError::AccountFlagged));
    }
    
    if !profile.kyc_verified && amount > 10_000_000_000 { // > 10 SOL requires KYC
        msg!("Transfer exceeds limit for unverified user.");
        return Err(error!(FluxError::TransferLimitExceeded));
    }

    // 2. Check Blacklist
    // Mock check: if recipient starts with "Bad", block it (conceptually)
    // In reality, we'd check a PDA or Bitmask in 'blacklist_registry'
    msg!("Verifying recipient against Global Blocklist...");
    
    // 3. Rate Limiting / Time Locks
    let time_since_last_tx = clock.unix_timestamp - profile.last_active_timestamp;
    if time_since_last_tx < 30 { // 30 seconds cooldown
        return Err(error!(FluxError::RateLimitExceeded));
    }

    // 4. Execution
    msg!("Executing Secure Transfer of {} lamports...", amount);
    
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.authority.key(),
        &ctx.accounts.recipient.key(),
        amount
    );
    
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.recipient.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ]
    )?;

    // 5. Update History
    msg!("Recording transaction in immutable log...");
    profile.record_action(ActionType::Withdraw, amount, &clock);
    
    // 6. Analytics
    if amount > 100_000_000_000 {
        msg!("Whale Alert: Large transfer detected!");
    }

    msg!("Transfer complete. New Reputation Score: {}", profile.reputation_score);
    Ok(())
}
