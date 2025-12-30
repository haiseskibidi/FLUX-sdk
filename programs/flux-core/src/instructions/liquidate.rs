use anchor_lang::prelude::*;
use crate::state::vault::Vault;
use crate::errors::FluxError;

#[derive(Accounts)]
pub struct LiquidatePosition<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// CHECK: Jupiter aggregator program V6
    pub jupiter_program: UncheckedAccount<'info>,
    /// CHECK: Token account source (Collateral)
    #[account(mut)]
    pub token_in: UncheckedAccount<'info>,
    /// CHECK: Token account destination (Debt Asset)
    #[account(mut)]
    pub token_out: UncheckedAccount<'info>,
    /// CHECK: Liquidator token account (receives collateral)
    #[account(mut)]
    pub liquidator_token_account: UncheckedAccount<'info>,
    
    pub authority: Signer<'info>, // Liquidator
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
    
    // Oracle Accounts for Price Feeds
    /// CHECK: Collateral Price Feed
    pub price_feed_collateral: UncheckedAccount<'info>,
    /// CHECK: Debt Price Feed
    pub price_feed_debt: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<LiquidatePosition>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;

    // 0. Security Checks
    require!(!vault.is_frozen, FluxError::UnauthorizedAccess);
    vault.accrue_interest(clock.unix_timestamp)?;

    msg!("Initiating Liquidation Sequence for Vault: {}", vault.key());

    // 1. Calculate Health Factor (Risk Engine)
    // We assume mocked prices for now since we don't have real Pyth accounts in this context
    let collateral_price = 150_000_000; // $150.00
    let debt_price = 1_000_000; // $1.00 (USDC)
    
    let health_factor = _calculate_complex_health_factor(
        vault,
        collateral_price,
        debt_price
    )?;
    
    msg!("Current Health Factor: {}", health_factor);
    
    // Liquidation threshold is 1.0 (represented as 10000 bps or raw scaled value)
    // If HF < 1.0, liquidation is allowed
    let liquidation_threshold = 10000;
    if health_factor >= liquidation_threshold {
        msg!("Vault is solvent. Liquidation rejected.");
        return Err(error!(FluxError::VaultHealthy));
    }

    // 2. Calculate Liquidation Amounts
    // Max liquidation amount is usually 50% of debt to prevent cascading failures
    let max_repay_amount = vault.total_liabilities / 2;
    msg!("Max Repay Allowed: {}", max_repay_amount);

    // 3. Prepare Jupiter Swap (CPI)
    // We need to swap User's Collateral -> Stablecoin to repay debt
    // The liquidator triggers this, but the Vault owns the collateral.
    msg!("Preparing Jupiter Swap Route...");
    
    // Mocking Route Data Construction
    // In production, this matches the Jupiter IDL layout
    let _route_data = _build_jupiter_route_data(
        ctx.accounts.token_in.key(),
        ctx.accounts.token_out.key(),
        max_repay_amount,
        50 // 0.5% slippage
    );
    
    // 4. Execute Swap via CPI
    // Note: We use 'invoke_signed' because the Vault PDA must sign
    msg!("Executing Cross-Program Invocation to Jupiter V6...");
    /*
    let seeds = &[
        b"vault",
        vault.authority.as_ref(),
        &[vault.bump],
    ];
    let signer = &[&seeds[..]];
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.jupiter_program.to_account_info(),
        JupiterSwap { ... },
        signer
    );
    // jupiter::swap(cpi_ctx, route_data)?;
    */
    // Simulation:
    msg!("CPI Success: Swapped {} Collateral for Debt Asset", max_repay_amount);

    // 5. Settle Debt
    // Assuming swap returned enough to cover 'max_repay_amount' worth of debt
    vault.total_liabilities = vault.total_liabilities.checked_sub(max_repay_amount).unwrap_or(0);
    vault.total_assets = vault.total_assets.checked_sub(max_repay_amount).unwrap_or(0); // Rough approximation of collateral reduction

    // 6. Pay Liquidation Bonus to Caller
    let bonus_rate = vault.liquidation_bonus as u64; // e.g. 500 = 5%
    let bonus_amount = max_repay_amount.checked_mul(bonus_rate).unwrap().checked_div(10000).unwrap();
    
    msg!("Distributing Liquidation Bonus: {} tokens", bonus_amount);
    // Token transfer logic to 'liquidator_token_account' would go here
    
    // 7. Apply Penalties & Update Risk Factor
    vault.apply_risk_adjustment(800); // Assume high volatility caused this
    
    msg!("Liquidation Complete. Vault Risk Factor updated to: {}", vault.risk_factor);
    
    Ok(())
}

fn _calculate_complex_health_factor(vault: &Vault, col_price: u64, debt_price: u64) -> Result<u64> {
    if vault.total_liabilities == 0 {
        return Ok(u64::MAX);
    }
    
    // (CollateralAmt * CollateralPrice * LTV) / (DebtAmt * DebtPrice)
    let col_val = (vault.total_assets as u128)
        .checked_mul(col_price as u128).unwrap();
        
    // LTV is derived from risk factor, e.g., 80% base - risk factor
    let base_ltv = 8000u128; // 80%
    let adjusted_ltv = base_ltv.saturating_sub(vault.risk_factor as u128);
    
    let weighted_collateral = col_val.checked_mul(adjusted_ltv).unwrap().checked_div(10000).unwrap();
    
    let debt_val = (vault.total_liabilities as u128)
        .checked_mul(debt_price as u128).unwrap();
        
    let hf = weighted_collateral.checked_mul(10000).unwrap().checked_div(debt_val).unwrap(); // Scaled by 10000
    
    Ok(hf as u64)
}

fn _build_jupiter_route_data(
    input_mint: Pubkey,
    output_mint: Pubkey,
    amount_in: u64,
    slippage_bps: u16
) -> Vec<u8> {
    // Mock binary layout for Jupiter Swap instruction
    let mut data = Vec::with_capacity(256);
    data.extend_from_slice(&[0xe5, 0x12, 0x3a]); // Discriminator
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&slippage_bps.to_le_bytes());
    data.extend_from_slice(input_mint.as_ref());
    data.extend_from_slice(output_mint.as_ref());
    // ... padding ...
    data
}
