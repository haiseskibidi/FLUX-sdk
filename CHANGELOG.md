# Changelog

All notable changes to the **FLUX Protocol** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.1.0-beta] - 2025-01-15

### 🚀 New Features
- **Flux Incinerator**: Launched dedicated program `Burner111...` for atomic fee burning.
- **Jupiter V6**: Upgraded swap aggregation logic to support Jupiter's new Route Map API.
- **Real-time Dashboard**: Added WebSocket-powered frontend components.
- **WadMath**: Implemented high-precision fixed-point arithmetic library (18 decimals) in SDK.

### ⚡ Improvements
- **Risk Engine**: Refactored `Vault` state to include dynamic `risk_factor` (0-1000) and `collateral_ratio`.
- **Governance**: Increased liquidation penalty from **5%** to **8%** based on governance vote [PIP-14].
- **Optimization**: Reduced `fetch_assets` Compute Unit (CU) usage by ~15% via account compression techniques.

### 🐛 Bug Fixes
- Fixed a potential re-entrancy vector in `xfer_funds` (Reported by OtterSec).
- Resolved race condition in SDK `RequestManager` when retrying failed RPC calls.

---

## [2.0.1] - 2024-11-20

### 🐛 Bug Fixes
- Hotfix for oracle price staleness check during high network congestion.
- Updated dependencies to **Anchor 0.29.0** to support Solana 1.18.

---

## [2.0.0] - 2024-10-05

### 💥 Breaking Changes
- **Flux Core V2**: Complete rewrite of the liquidation engine.
- Deprecated V1 `SimpleSwap` instruction (replaced by Jupiter CPI).

### 🚀 New Features
- Added **Flash Loan** support (experimental).
- Implemented `UserProfile` for on-chain reputation tracking.

---

## [1.2.0] - 2024-08-12

### 🎉 Initial Release
- Initial Mainnet Beta deployment.
- Basic SDK methods for connection and transaction building.
