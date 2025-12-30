#!/bin/bash

# Build for mainnet
anchor build --verifiable --provider.cluster mainnet

# Deploy Core
solana program deploy target/deploy/flux_core.so --program-id FluxCore1111111111111111111111111111111111111

# Deploy Incinerator
solana program deploy target/deploy/flux_incinerator.so --program-id Burner11111111111111111111111111111111111111

echo "Deployment complete."

