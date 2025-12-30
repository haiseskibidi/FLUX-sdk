use anchor_lang::prelude::*;

pub fn update_config_handler(ctx: Context<crate::UpdateConfig>, new_risk_factor: u16) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    require!(ctx.accounts.authority.key() == vault.authority, crate::errors::FluxError::UnauthorizedAccess);
    
    vault.risk_factor = new_risk_factor;
    msg!("Vault configuration updated. New risk factor: {}", new_risk_factor);
    
    Ok(())
}

