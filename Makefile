# FLUX Protocol Makefile

.PHONY: build test clean deploy-localnet

# Variables
ANCHOR_BIN := anchor
SOLANA_BIN := solana

# Build all programs
build:
	@echo "Building Flux Core and Incinerator..."
	$(ANCHOR_BIN) build --verifiable

# Run integration tests
test:
	@echo "Running Integration Suite..."
	$(ANCHOR_BIN) test

# Linting
lint:
	@echo "Linting Rust..."
	cargo clippy --all-targets -- -D warnings
	@echo "Linting TypeScript..."
	yarn lint

# Deploy to Localnet
deploy-local:
	@echo "Starting Localnet..."
	$(ANCHOR_BIN) localnet

# Clean artifacts
clean:
	$(ANCHOR_BIN) clean
	rm -rf node_modules

