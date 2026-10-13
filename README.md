# SplitPay Backend & Smart Contract 💸

[![CI](https://github.com/oak-splitpay/splitpay-backend-contract/actions/workflows/ci.yml/badge.svg)](https://github.com/oak-splitpay/splitpay-backend-contract/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Soroban](https://img.shields.io/badge/Soroban-v22-blue.svg)](https://soroban.stellar.org/)

The core on-chain payment engine and backend coordination service for **SplitPay** — a decentralized, zero-custody bill splitting application built natively on the Stellar blockchain.

---

## 🔗 Architecture & Ecosystem

SplitPay consists of two coordinated repositories:
1. **`oak-splitpay/splitpay-backend-contract` (This Repository)**: Houses the Soroban smart contract (`contracts/`) which enforces on-chain state, duplicate payment prevention, and token transfers, alongside the NestJS indexing service (`src/`) that manages split session metadata.
2. **[`oak-splitpay/splitpay-frontend`](https://github.com/oak-splitpay/splitpay-frontend)**: The responsive Next.js user interface with Freighter wallet integration, bill creation wizard, QR code generators, and live payment status tracking.

```
┌─────────────────────────────────┐
│     splitpay-frontend (Next.js) │
│  - Freighter Wallet Connect     │
│  - Split Creation & QR Codes    │
│  - Live Payment Tracker         │
└───────────────┬─────────────────┘
                │
        ┌───────┴────────┐
        ▼                ▼
┌──────────────────┐  ┌──────────────────────────────────┐
│ NestJS Indexer   │  │ Soroban Smart Contract           │
│ (src/)           │  │ (contracts/src/lib.rs)           │
│ - Session Meta   │  │ - Multi-Session State            │
│ - REST Endpoints │  │ - Peer-to-Peer Token Settlement  │
│ - Status Cache   │  │ - Double-Payment Prevention Guard│
└──────────────────┘  └──────────────────────────────────┘
```

---

## ⚡ Smart Contract Specification

The Soroban smart contract is implemented in Rust (`contracts/src/lib.rs`) using Soroban SDK `v22.0.8`.

### Storage Architecture
- Multi-session isolation via indexed keys:
  - `DataKey::Split(u64)`: Stores `SplitInfo` containing total amount, per-person share, total participants, paid count, receiver address, optional SEP-41 token address, and completion status.
  - `DataKey::Payment(u64, Address)`: Stores `bool` recording whether a specific payer has already contributed to a given split.

### Methods

| Method | Parameters | Description |
| :--- | :--- | :--- |
| `init_split` | `split_id: u64, total_amount: i128, total_participants: u32, receiver: Address, token: Option<Address>` | Initializes a new bill split session. Enforces positive amounts and at least 2 participants. Computes exact per-person share. |
| `record_payment` | `split_id: u64, payer: Address` | Authenticates payer, verifies the split is active, checks for double-payment, executes SEP-41 token transfer to receiver, and increments paid count. |
| `get_split` | `split_id: u64` | Retrieves full `SplitInfo` metadata for a given split session. |
| `get_payment_status` | `split_id: u64, payer: Address` | Returns `bool` indicating whether a specific payer has paid their share. |
| `is_split_complete` | `split_id: u64` | Returns `bool` indicating if all participants have completed payments. |

---

## 🧪 Testing & Verification

### 1. Smart Contract Unit Tests (6/6 Passing)
```bash
cargo test --manifest-path contracts/Cargo.toml --verbose
```
**Test Matrix**:
- `test_init_split_success`: Verifies initial parameter recording and exact per-person division.
- `test_init_split_invalid_params`: Verifies rejection of zero amounts and single-participant splits.
- `test_init_split_duplicate_id`: Ensures unique split IDs cannot be overwritten.
- `test_record_payment_and_completion`: Simulates complete payment lifecycle from pending to complete, verifying duplicate payment rejection.
- `test_record_payment_with_token_settlement`: Verifies on-chain SEP-41 token transfer from payer to receiver.
- `test_multiple_independent_splits`: Proves state isolation across concurrent split sessions.

### 2. WASM Compilation
```bash
cargo build --manifest-path contracts/Cargo.toml --target wasm32-unknown-unknown --release
```
Builds `contracts/target/wasm32-unknown-unknown/release/splitpay_contract.wasm` (14.8 KB).

### 3. NestJS Backend Build & Typecheck
```bash
npm install
npm run typecheck
npm run build
```

---

## 🌐 Live Stellar Testnet Evidence

Live end-to-end verification of the payment workflow was executed on Stellar Testnet:
- **Verified Timestamp**: `2026-10-13T15:03:43Z`
- **Network**: Stellar Testnet (`https://horizon-testnet.stellar.org`)
- **Split Session ID**: `1791903815791`
- **Ledger Sequence**: `5176047`
- **Organizer**: [`GD667KTLEPMJ4MLRIH23SWKHIFFZBWPTSODN5RFPCCWNJ6RHJG7OXF4M`](https://stellar.expert/explorer/testnet/account/GD667KTLEPMJ4MLRIH23SWKHIFFZBWPTSODN5RFPCCWNJ6RHJG7OXF4M)
- **Participant 1 (Payer)**: [`GCN74D5M3ORI6N5KHOOZECJJK5CDPPAIMYI33ZQ6CUSVY5NULSH7ALNR`](https://stellar.expert/explorer/testnet/account/GCN74D5M3ORI6N5KHOOZECJJK5CDPPAIMYI33ZQ6CUSVY5NULSH7ALNR)
- **Receiver**: [`GCLIQJUZ3GBBFAAC244RGJ3AENPAMF2S2ROU6P5KTPNXBPV3TGJV7QDA`](https://stellar.expert/explorer/testnet/account/GCLIQJUZ3GBBFAAC244RGJ3AENPAMF2S2ROU6P5KTPNXBPV3TGJV7QDA)
- **Settlement Transaction Hash**: [`44a7e8b0d1110ae839981e0f2e1de003d4081cc693d9c629c811b36255c56c91`](https://stellar.expert/explorer/testnet/tx/44a7e8b0d1110ae839981e0f2e1de003d4081cc693d9c629c811b36255c56c91)
- **Observed Balances**:
  - Participant 1 debited: `9949.9999900 XLM` (-50 XLM payment + 0.00001 XLM network fee)
  - Receiver credited: `10050.0000000 XLM` (+50 XLM payment received)
  - Duplicate payment prevention guard successfully verified on subsequent transaction attempt.

---

## 🚀 Running the NestJS API

```bash
npm install
npm run dev
```
API runs on `http://localhost:3001`.

### API Endpoints
- `POST /api/splits`: Creates a new split session.
- `GET /api/splits/:id`: Fetches split details and participant statuses.
- `POST /api/splits/:id/payments`: Records an incoming payment settlement transaction hash.

---

## 🤝 Contributing

We welcome community contributions! Please review [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidelines.

### Open Issues & Contributor Roadmap
- Persistent PostgreSQL database adapter for split metadata indexing.
- Soroban contract event listener & webhook dispatch service.
- Multi-asset exchange rate oracle integration (SDEX pricing).

---

## 📜 License

This project is licensed under the [Apache-2.0 License](./LICENSE).
