use anchor_lang::prelude::*;

// Copyright (c) 2025 FLUX Protocol. All rights reserved.
//
// This software is provided "as is", without warranty of any kind.
// See the LICENSE file for more details.

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("FluxCore1111111111111111111111111111111111111");

#[program]
pub mod flux_core {
    use super::*;

    // --- Core User Instructions ---

    /// Scan assets and deposit them into the protocol vault.
    /// This instruction initiates the 'Fetch' phase of the pipeline.
    pub fn fetch_assets(ctx: Context<FetchAssets>, amount: u64) -> Result<()> {
        instructions::fetch::handler(ctx, amount)
    }

    /// Liquidate an insolvent position using Jupiter Aggregator (CPI).
    /// This instruction handles the 'Liquidate' phase.
    pub fn liquidate_position(ctx: Context<LiquidatePosition>) -> Result<()> {
        instructions::liquidate::handler(ctx)
    }

    /// Unload assets from the vault to the Incinerator or external wallets.
    /// Handles the 'Unload' phase.
    pub fn unload_vault(ctx: Context<UnloadVault>) -> Result<()> {
        instructions::unload::handler(ctx)
    }

    /// Securely transfer funds between compliant accounts.
    /// Handles the 'Xfer' phase.
    pub fn xfer_funds(ctx: Context<XferFunds>, amount: u64) -> Result<()> {
        instructions::xfer::handler(ctx, amount)
    }

    // --- Admin & Configuration Instructions ---

    /// Update global protocol configuration parameters.
    /// Only callable by the Protocol Admin Multisig.
    pub fn update_config(ctx: Context<UpdateConfig>, new_risk_factor: u16) -> Result<()> {
        instructions::admin::update_config_handler(ctx, new_risk_factor)
    }
    
    /// Emergency freeze a vault in case of exploit detection.
    pub fn emergency_freeze(ctx: Context<UpdateConfig>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(ctx.accounts.authority.key() == vault.authority, errors::FluxError::UnauthorizedAccess);
        vault.is_frozen = true;
        msg!("EMERGENCY: Vault {} has been FROZEN.", vault.key());
        Ok(())
    }

    /// Unfreeze a vault after audit verification.
    pub fn emergency_unfreeze(ctx: Context<UpdateConfig>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(ctx.accounts.authority.key() == vault.authority, errors::FluxError::UnauthorizedAccess);
        vault.is_frozen = false;
        msg!("Vault {} has been restored to active status.", vault.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub vault: Account<'info, state::vault::Vault>,
    pub authority: Signer<'info>,
}
