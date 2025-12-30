use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn};

// Copyright (c) 2025 FLUX Protocol. All rights reserved.

declare_id!("Burner11111111111111111111111111111111111111");

#[program]
pub mod flux_incinerator {
    use super::*;

    pub fn initialize_registry(ctx: Context<InitRegistry>) -> Result<()> {
        let registry = &mut ctx.accounts.burn_registry;
        registry.total_burned = 0;
        registry.burn_count = 0;
        registry.last_burn_timestamp = Clock::get()?.unix_timestamp;
        registry.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn incinerate(ctx: Context<Incinerate>, amount: u64) -> Result<()> {
        msg!("Incinerating {} tokens...", amount);
        
        let registry = &mut ctx.accounts.burn_registry;
        
        // 1. Burn Tokens via SPL Token Program
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::burn(cpi_ctx, amount)?;
        
        // 2. Update Registry Statistics
        registry.total_burned = registry.total_burned.checked_add(amount as u128).unwrap();
        registry.burn_count += 1;
        registry.last_burn_timestamp = Clock::get()?.unix_timestamp;
        
        // Mock "Entropy" Calculation for randomness generation
        // (Just a complex hash of the burn event)
        let entropy_input = [
            amount.to_le_bytes(),
            registry.burn_count.to_le_bytes(),
        ].concat();
        let _entropy_hash = anchor_lang::solana_program::hash::hash(&entropy_input);

        // 3. Emit Event
        emit!(IncinerationEvent {
            amount,
            burner: ctx.accounts.authority.key(),
            timestamp: registry.last_burn_timestamp,
            total_burned_lifetime: registry.total_burned,
        });

        msg!("Burn complete. Entropy reduced.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRegistry<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 8 + BurnRegistry::LEN,
        seeds = [b"registry"],
        bump
    )]
    pub burn_registry: Account<'info, BurnRegistry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Incinerate<'info> {
    #[account(mut, seeds = [b"registry"], bump)]
    pub burn_registry: Account<'info, BurnRegistry>,
    
    /// CHECK: Token Mint
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: Token Account to burn from
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}

#[account]
pub struct BurnRegistry {
    pub authority: Pubkey,
    pub total_burned: u128,
    pub burn_count: u64,
    pub last_burn_timestamp: i64,
    pub epoch_stats: [u64; 10], // Last 10 epochs
}

impl BurnRegistry {
    pub const LEN: usize = 32 + 16 + 8 + 8 + (8 * 10);
}

#[event]
pub struct IncinerationEvent {
    pub amount: u64,
    pub burner: Pubkey,
    pub timestamp: i64,
    pub total_burned_lifetime: u128,
}
