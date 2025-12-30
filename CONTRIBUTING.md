# Contributing to FLUX Protocol

Thank you for your interest in contributing to FLUX! We welcome contributions from the community to help make this the most robust liquidity engine on Solana.

> [!NOTE]
> Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before participating in our community.

## 🛠 Development Process

We use a modified **Gitflow** workflow.

1.  **Fork** the repository.
2.  **Clone** your fork locally.
3.  Create a **feature branch** (`git checkout -b feature/amazing-feature`).
4.  Commit your changes (`git commit -m 'feat: Add amazing feature'`).
5.  Push to the branch (`git push origin feature/amazing-feature`).
6.  Open a **Pull Request**.

### 🧪 Testing Requirements

All Pull Requests must pass the full test suite. We enforce a **zero-regression policy**.

- **Unit Tests**: `cargo test` inside program directories.
- **Integration Tests**: `anchor test` using the TypeScript suite.
- **Fuzz Tests**: Required for any math-heavy logic updates.

```bash
# Run the fuzzing harness
cd tests/fuzz
cargo run
```

### 🎨 Style Guide

We enforce strict linting rules to maintain code quality.

*   **Rust**: Follows standard [Rustfmt](https://github.com/rust-lang/rustfmt) conventions.
    *   Run `cargo fmt --all` before committing.
    *   Run `cargo clippy` to check for common mistakes.
*   **TypeScript**: Follows [Prettier](https://prettier.io/) standards.
    *   Run `yarn lint` to verify.

## 📦 Project Structure

<details>
<summary>Click to expand directory tree</summary>

```text
flux-protocol/
├── programs/           # Solana Smart Contracts (Rust)
│   ├── flux-core/      # Main protocol logic (Vaults, Risk)
│   └── flux-incinerator # Fee burning mechanism
├── sdk/                # TypeScript Client SDK
├── app/                # Frontend Reference Implementation
├── tests/              # Integration & Fuzz Tests
└── docs/               # Architecture & Audits
```

</details>

## 🔐 Security

If you discover a potential security vulnerability, please **DO NOT** report it on the public issue tracker.

*   Email: `security@flux.protocol`
*   PGP Key: See [SECURITY.md](SECURITY.md)

We offer a generous bounty program for responsible disclosure.
