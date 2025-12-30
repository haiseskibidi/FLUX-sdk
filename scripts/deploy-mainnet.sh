#!/bin/bash

# Build for mainnet
anchor build --verifiable --provider.cluster mainnet

# Deploy Core
solana program deploy target/deploy/flux_core.so --program-id Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS

# Deploy Incinerator
solana program deploy target/deploy/flux_incinerator.so --program-id 86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY

echo "Deployment complete."

