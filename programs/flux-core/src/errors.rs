use anchor_lang::prelude::*;

#[error_code]
pub enum FluxError {
    // General Errors
    #[msg("Insufficient liquidity in vault")]
    InsufficientLiquidity,
    #[msg("Unauthorized access to Flux Vault")]
    UnauthorizedAccess,
    #[msg("Invalid arithmetic operation (Overflow/Underflow)")]
    ArithmeticError,
    #[msg("Invalid amount specified")]
    InvalidAmount,

    // Liquidation Errors
    #[msg("Slippage tolerance exceeded during liquidation")]
    SlippageExceeded,
    #[msg("Vault is currently healthy, liquidation rejected")]
    VaultHealthy,
    #[msg("Vault is frozen due to emergency")]
    VaultFrozen,
    #[msg("Oracle price data is stale or invalid")]
    StaleOraclePrice,
    #[msg("Health factor calculation failed")]
    HealthFactorCheckFailed,

    // User Profile & Compliance
    #[msg("Account has been flagged for AML review")]
    AccountFlagged,
    #[msg("Transfer limit exceeded for unverified account")]
    TransferLimitExceeded,
    #[msg("Rate limit exceeded, please try again later")]
    RateLimitExceeded,
    #[msg("User is blacklisted")]
    UserBlacklisted,
    #[msg("Insufficient reputation score for this action")]
    LowReputation,

    // Configuration
    #[msg("Invalid risk factor configuration")]
    InvalidRiskFactor,
    #[msg("Configuration update cooldown active")]
    ConfigCooldown,
    
    // System
    #[msg("Protocol paused by administrator")]
    ProtocolPaused,
    #[msg("Feature not yet implemented")]
    NotImplemented,
}
