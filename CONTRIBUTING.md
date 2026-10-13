# Contributing to SplitPay Backend & Contracts 💸

Thank you for your interest in contributing to SplitPay! This repository houses both the on-chain Soroban smart contract and the NestJS indexing service.

## 🔗 Repository Ecosystem
- **Smart Contract & Backend API (This Repo)**: `oak-splitpay/splitpay-backend-contract`
- **Frontend dApp**: [`oak-splitpay/splitpay-frontend`](https://github.com/oak-splitpay/splitpay-frontend)

## 🛠️ Development Setup

### Rust & Soroban Smart Contract
```bash
cd contracts
# Run unit tests
cargo test --verbose
# Compile release WASM
cargo build --target wasm32-unknown-unknown --release
```

### NestJS Backend API
```bash
# Install dependencies
npm install
# Typecheck
npm run typecheck
# Build
npm run build
# Start development server
npm run dev
```

## 🌊 Issue Guidelines
- Backend tasks: Database persistence, Soroban event indexing, webhook notifications, multi-token price conversions.
- Frontend tasks belong in the [`splitpay-frontend`](https://github.com/oak-splitpay/splitpay-frontend) repository.
- Ensure all tests pass (`cargo test` and `npm run typecheck`) before submitting a pull request.
