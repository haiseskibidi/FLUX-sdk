# System Context

FLUX Protocol operates within the high-frequency Solana DeFi ecosystem. It interacts with several external systems to maintain solvency and execute liquidations.

## External Dependencies

1.  **Solana Blockchain**: The underlying L1 ledger for state storage and transaction execution.
2.  **Jupiter Aggregator (v6)**: Provides optimal swap routes for liquidation events. Flux integrates via CPI (Cross-Program Invocation) to execute swaps atomically.
3.  **Pyth Network**: Primary oracle for real-time asset pricing.
4.  **Switchboard**: Backup oracle for redundancy.

## Internal Components

### Flux Router (Client-Side)
The "smart" client that determines the best action (deposit, withdraw, liquidate) based on on-chain state.

### Flux Core (On-Chain)
The heart of the protocol. Manages:
- **Vaults**: User collateral and debt positions.
- **Risk Engine**: Calculates health factors dynamic risk adjustments.
- **Liquidation Logic**: Permissionless instructions to rebalance insolvent vaults.

### Flux Incinerator (On-Chain)
A specialized program dedicated to burning FLUX tokens collected from protocol fees. It reduces the total supply, creating deflationary pressure.

## Data Flow: Liquidation Event

1.  **Monitor**: Keepers (bots) watch for vaults where `Health Factor < 1.0`.
2.  **Trigger**: Keeper calls `liquidate_position` on `Flux Core`.
3.  **Check**: `Flux Core` verifies the vault is underwater and not frozen.
4.  **Swap**: `Flux Core` CPIs into `Jupiter` to swap Collateral -> Stablecoin.
5.  **Repay**: Debt is repaid.
6.  **Burn**: Protocol fee is sent to `Flux Incinerator` and burned.
7.  **Reward**: Keeper receives a liquidation bonus.

