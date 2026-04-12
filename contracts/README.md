# Splitpay Soroban Contract

On-chain smart contract for Splitpay settlement logic. Manages split creation and payment tracking on the Stellar blockchain.

##  Overview

The Splitpay Soroban contract handles:
- **Split Initialization** - Create split sessions with total amount and participant count
- **Payment Recording** - Track individual participant payments
- **Completion Detection** - Determine when all payments received
- **Event Emission** - Emit events for off-chain tracking

##  Architecture

```
contract: SplitpayContract
├── init_split()          # Initialize a new split session
├── record_payment()      # Record a participant's payment
├── get_split_status()    # Retrieve current split state
└── is_split_complete()   # Check completion status
```

##  Data Types

### Split
```rust
pub struct Split {
    pub id: String,
    pub receiver: Address,
    pub total_amount: i128,
    pub num_participants: u32,
    pub amount_per_person: i128,
    pub payments_received: u32,
    pub is_completed: bool,
}
```

### Payment
```rust
pub struct Payment {
    pub split_id: String,
    pub payer: Address,
    pub amount: i128,
}
```

##  Contract Functions

### init_split
Initialize a new split session.

**Parameters:**
- `split_id`: Unique identifier (String)
- `receiver`: Receiver's Stellar address (Address)
- `total_amount`: Total amount in stroops (i128)
- `num_participants`: Number of participants (u32)

**Returns:** Amount each participant should pay (i128)

**Example:**
```
split_id: "split-001"
receiver: GXXXXX...
total_amount: 40_000_000 (40 XLM)
num_participants: 4
returns: 10_000_000 (10 XLM per person)
```

### record_payment
Record a payment from a participant.

**Parameters:**
- `split_id`: Split session ID (String)
- `payer`: Payer's address (Address) - must sign transaction
- `amount`: Payment amount (i128)

**Returns:** Boolean indicating if split is now complete

**Validation:**
- Payer must authorize the transaction
- Amount must equal calculated per-person amount
- Split must not already be completed

### get_split_status
Retrieve current state of a split.

**Returns:** Full Split object with current payment count and completion status

### is_split_complete
Check if all payments received.

**Returns:** Boolean - true if payments_received >= num_participants

##  Events

The contract emits events for off-chain tracking:

**Event: init**
```
topic: ("init", split_id)
data: Split object
```

**Event: payment**
```
topic: ("payment", split_id)
data: Payment object
```

##  Testing

Run tests with:
```bash
cargo test
```

Tests include:
- `test_init_split()` - Verify split initialization
- `test_record_payment()` - Verify payment recording and status updates

##  Building

### Prerequisites
- Rust 1.70+
- Soroban CLI

### Compile
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Deploy
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/splitpay_contract.wasm \
  --source <source-account> \
  --network <network>
```

##  Security Notes

- **Input Validation** - All amounts validated against per-person calculation
- **Authorization** - Payment recording requires payer signature
- **State Management** - Prevents double-counting and invalid state transitions
- **Deterministic** - Per-person amount calculated consistently
- **Immutable Records** - Payment events are immutable and on-chain

##  Storage

The contract uses persistent storage to maintain:
- Current split state
- Payment count and completion status

Data is keyed and retrieved during contract invocation.

##  Integration with Backend/Frontend

1. **Backend** calls `init_split` to create on-chain session
2. **Frontend** submits payment transactions that invoke `record_payment`
3. **Backend** polls `get_split_status` to track progress via events
4. **Frontend** listens to event stream for real-time updates



