use anchor_lang::prelude::*;

// Copyright (c) 2025 FLUX Protocol. All rights reserved.

#[account]
pub struct UserProfile {
    pub owner: Pubkey,
    pub reputation_score: u8,
    pub active_loans: u32,
    pub total_borrowed_lifetime: u64,
    pub total_repaid_lifetime: u64,
    pub liquidation_count: u16,
    pub last_active_timestamp: i64,
    pub role: UserRole,
    
    // History Tracking
    pub action_history: [UserAction; 50],
    pub history_idx: u8, // Circular buffer index
    
    // KYC/AML Flags
    pub kyc_verified: bool,
    pub aml_flagged: bool,
    pub country_code: [u8; 2],
    
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum UserRole {
    Standard,
    Premium,
    Institutional,
    Auditor,
    Blacklisted,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Default)]
pub struct UserAction {
    pub action_type: ActionType,
    pub amount: u64,
    pub timestamp: i64,
    pub tx_signature_hash: [u8; 8], // Partial hash for storage efficiency
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum ActionType {
    None,
    Deposit,
    Withdraw,
    Borrow,
    Repay,
    Liquidated,
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::None
    }
}

impl UserProfile {
    pub const LEN: usize = 8 + 32 + 1 + 4 + 8 + 8 + 2 + 8 + 1 + (50 * (1 + 8 + 8 + 8)) + 1 + 1 + 1 + 2 + 1;

    pub fn record_action(&mut self, action_type: ActionType, amount: u64, clock: &Clock) {
        let idx = self.history_idx as usize;
        
        self.action_history[idx] = UserAction {
            action_type,
            amount,
            timestamp: clock.unix_timestamp,
            tx_signature_hash: [0u8; 8], // In real app, hash the sig
        };
        
        self.history_idx = ((idx + 1) % 50) as u8;
        self.last_active_timestamp = clock.unix_timestamp;
        
        // Update aggregate stats
        match action_type {
            ActionType::Borrow => {
                self.total_borrowed_lifetime = self.total_borrowed_lifetime.saturating_add(amount);
                self.active_loans = self.active_loans.saturating_add(1);
            },
            ActionType::Repay => {
                self.total_repaid_lifetime = self.total_repaid_lifetime.saturating_add(amount);
                if self.active_loans > 0 {
                    self.active_loans -= 1;
                }
            },
            ActionType::Liquidated => {
                self.liquidation_count = self.liquidation_count.saturating_add(1);
                // Penalty on reputation
                self.reputation_score = self.reputation_score.saturating_sub(10);
            },
            _ => {}
        }
        
        // Dynamic reputation adjustment
        self.update_reputation();
    }
    
    fn update_reputation(&mut self) {
        // If user repays more than borrows, increase rep
        if self.total_repaid_lifetime > self.total_borrowed_lifetime {
            self.reputation_score = self.reputation_score.saturating_add(1);
        }
        
        // Cap at 100
        if self.reputation_score > 100 {
            self.reputation_score = 100;
        }
    }
    
    pub fn is_eligible_for_premium(&self) -> bool {
        self.reputation_score > 80 && self.total_borrowed_lifetime > 1000_000_000_000 // 1000 SOL
    }
}
