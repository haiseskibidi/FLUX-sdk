use anchor_lang::prelude::*;
use crate::state::vault::Vault;

#[derive(Accounts)]
pub struct UnloadVault<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// CHECK: Incinerator program
    pub incinerator_program: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<UnloadVault>) -> Result<()> {
    msg!("Connecting to Flux Incinerator...");
    
    // ... CPI to Flux Incinerator to burn assets ...
    
    msg!("Assets unloaded and burned.");
    Ok(())
}

