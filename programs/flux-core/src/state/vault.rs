use anchor_lang::prelude::*;

// Copyright (c) 2025 FLUX Protocol. All rights reserved.

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub total_assets: u64,
    pub total_liabilities: u64,
    pub last_update: i64,
    pub bump: u8,
    // 2025 Upgrade fields
    pub collateral_ratio: u16, // basis points, e.g., 15000 for 150%
    pub risk_factor: u16,      // dynamic risk score (0-1000)
    pub oracle_config: Pubkey, // pointer to oracle configuration
    pub is_frozen: bool,       // emergency stop
    
    // Complex State Tracking
    pub interest_accumulator: u128,
    pub last_fee_collection: i64,
    pub performance_fee_rate: u16,
    pub management_fee_rate: u16,
    pub flash_loan_fee_rate: u16,
    pub liquidation_penalty: u16,
    pub liquidation_bonus: u16,
    
    // Reserved for future upgrades
    pub reserved: [u8; 128],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub struct VaultConfig {
    pub min_collateral_ratio: u16,
    pub max_leverage: u8,
    pub optimal_utilization: u16,
    pub base_rate: u16,
    pub max_rate: u16,
}

impl Vault {
    // Calculated size to ensure future compatibility
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 1 + 2 + 2 + 32 + 1 + 16 + 8 + 2 + 2 + 2 + 2 + 2 + 128;

    pub fn calculate_health_factor(&self, oracle_price: u64, _oracle_decimals: u8) -> Result<u64> {
        // Mock complex health factor calculation
        // HF = (Collateral * Price * LiquidationThreshold) / Debt
        if self.total_liabilities == 0 {
            return Ok(u64::MAX);
        }

        // Simulating heavy math operations
        let collateral_val = self.total_assets as u128;
        let price_norm = oracle_price as u128;
        let threshold = 8500u128; // 85% LTV

        let numerator = collateral_val
            .checked_mul(price_norm).ok_or(error!(crate::errors::FluxError::ArithmeticError))?
            .checked_mul(threshold).ok_or(error!(crate::errors::FluxError::ArithmeticError))?;
        
        let denominator = (self.total_liabilities as u128)
            .checked_mul(10000).ok_or(error!(crate::errors::FluxError::ArithmeticError))?;

        let hf = numerator.checked_div(denominator).ok_or(error!(crate::errors::FluxError::ArithmeticError))?;
        
        Ok(hf as u64)
    }

    pub fn accrue_interest(&mut self, current_time: i64) -> Result<()> {
        let time_delta = current_time.checked_sub(self.last_update).unwrap_or(0);
        if time_delta == 0 {
            return Ok(());
        }

        // Simple interest model simulation
        // Interest = Principal * Rate * Time
        let rate_per_second = 5u128; // very small number
        let interest = (self.total_liabilities as u128)
            .checked_mul(rate_per_second).unwrap()
            .checked_mul(time_delta as u128).unwrap()
            .checked_div(1_000_000_000).unwrap();

        self.interest_accumulator = self.interest_accumulator.checked_add(interest).unwrap();
        self.total_liabilities = self.total_liabilities.checked_add(interest as u64).unwrap();
        self.last_update = current_time;

        msg!("Interest accrued: {} lamports over {} seconds", interest, time_delta);
        Ok(())
    }

    pub fn validate_collateral(&self, min_ratio: u16) -> bool {
        // Complex validation logic
        if self.total_liabilities == 0 {
            return true;
        }
        
        let current_ratio = (self.total_assets as u128)
            .checked_mul(10000).unwrap()
            .checked_div(self.total_liabilities as u128).unwrap_or(0);
            
        current_ratio >= min_ratio as u128
    }

    pub fn is_solvent(&self) -> bool {
        self.total_assets >= self.total_liabilities
    }

    pub fn apply_risk_adjustment(&mut self, market_volatility: u16) {
        // Dynamic risk adjustment based on market conditions
        if market_volatility > 500 { // High volatility > 5%
            self.risk_factor = self.risk_factor.saturating_add(100);
            self.liquidation_penalty = self.liquidation_penalty.saturating_add(500); // Increase penalty
            msg!("High volatility detected: Risk factor increased to {}", self.risk_factor);
        } else {
            self.risk_factor = self.risk_factor.saturating_sub(10);
            if self.risk_factor < 100 { self.risk_factor = 100; }
        }
    }
}
