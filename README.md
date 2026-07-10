# Splitpay Backend 

**Stellar Drip Wave Submission**

Backend API for Splitpay - a Stellar-based bill splitting application. Manages split sessions, payment tracking, and settlement coordination.

## Drip Wave Submission Details

### Product Scope
Splitpay Backend serves as the indexing and tracking API for the Splitpay ecosystem. It tracks real-time settlement status of Soroban contract payments via the Stellar Horizon API, facilitating seamless peer-to-peer bill splitting on-chain.

### Migration Plan
Splitpay is being built natively on Stellar to capitalize on the robust functionality of Soroban smart contracts, reducing costs for micro-transactions while offering near-instant settlement.


## Architecture

```
┌──────────────────────────────┐
│   API Controller Layer        │
│  (SplitController)            │
└──────────────┬───────────────┘
               │
┌──────────────▼───────────────┐
│   Service Layer              │
│  (SplitService)              │
└──────────────┬───────────────┘
               │
┌──────────────▼───────────────┐
│   Data Layer                 │
│  (InMemoryDatabaseService)   │
│  → PostgreSQL (Production)   │
└──────────────────────────────┘
```

## Project Structure

```
splitpay-backend/
├── src/
│   ├── main.ts                    # Application entry point
│   ├── app.module.ts              # Root module
│   ├── config/
│   │   └── config.ts              # Environment configuration
│   ├── database/
│   │   └── in-memory.service.ts   # Data persistence layer
│   └── split/
│       ├── split.module.ts        # Split feature module
│       ├── split.controller.ts    # HTTP endpoints
│       ├── split.service.ts       # Business logic
│       ├── entities/
│       │   ├── split.entity.ts    # Split session model
│       │   └── payment.entity.ts  # Payment record model
│       └── dtos/
│           ├── create-split.dto.ts
│           └── payment.dto.ts
├── package.json
├── tsconfig.json
└── README.md
```

## Quick Start

### Prerequisites
- Node.js v18+
- npm or yarn

### Installation

```bash
cd splitpay-backend
npm install
```

### Development

```bash
npm run dev
```

Backend runs on `http://localhost:3001`

### Build

```bash
npm run build
npm start
```

## API Endpoints

### Create Split Session
```
POST /api/splits
Content-Type: application/json

{
  "title": "Dinner",
  "totalAmount": 120,
  "asset": "USDC",
  "participants": 4,
  "receiverAddress": "G..."
}

Response:
{
  "message": "Split session created",
  "splitId": "uuid",
  "shareableLink": "https://splitpay.app/pay/uuid",
  "split": { ... }
}
```

### Get Split Details
```
GET /api/splits/:id

Response:
{
  "id": "uuid",
  "title": "Dinner",
  "totalAmount": 120,
  "asset": "USDC",
  "participants": 4,
  "amountPerPerson": 30,
  "status": "active",
  "payments": [],
  "createdAt": "2026-04-12T...",
  "expiresAt": "2026-04-19T..."
}
```

### Record Payment
```
POST /api/splits/:id/payments
Content-Type: application/json

{
  "splitId": "uuid",
  "senderAddress": "G...",
  "transactionHash": "...",
  "ledger": 12345
}

Response:
{
  "id": "payment-uuid",
  "splitId": "uuid",
  "senderAddress": "G...",
  "amount": 30,
  "asset": "USDC",
  "status": "confirmed",
  "transactionHash": "...",
  "confirmedAt": "2026-04-12T..."
}
```

### Get Split Status
```
GET /api/splits/:id/status

Response:
{
  "splitId": "uuid",
  "status": "active",
  "totalAmount": 120,
  "amountPerPerson": 30,
  "totalReceived": 90,
  "paymentsReceived": 3,
  "totalParticipants": 4,
  "progress": 75,
  "payments": [
    {
      "id": "...",
      "senderAddress": "G...",
      "amount": 30,
      "status": "confirmed",
      "confirmedAt": "2026-04-12T..."
    }
  ]
}
```

### Get Payments
```
GET /api/splits/:id/payments

Response: [
  {
    "id": "payment-uuid",
    "splitId": "uuid",
    "senderAddress": "G...",
    "amount": 30,
    "status": "confirmed",
    "transactionHash": "...",
    "confirmedAt": "2026-04-12T..."
  }
]
```

## Integration with Frontend

The backend supports the Splitpay frontend workflow:

1. **Create Split** - Frontend calls `POST /api/splits` with bill details
2. **Get Share Link** - Backend returns `shareableLink` with split ID
3. **Track Status** - Frontend polls `GET /api/splits/:id/status` for updates
4. **Record Payment** - After Stellar transaction, frontend calls `POST /api/splits/:id/payments`

## Stellar Integration

The backend:
- Stores split session metadata
- Records payment transactions from Stellar
- Tracks settlement progress
- Manages participant addresses

Frontend handles:
- Wallet connection via Freighter
- Transaction signing
- Stellar payment execution

The contract in `../contracts/` handles atomic payment logic on-chain.

## Contract Integration

This project includes a Soroban smart contract in the `contracts` folder. The contract is used to:
- Initialize split sessions on-chain
- Record individual participant payments
- Track payment completion status
- Emit events for off-chain listeners

The backend can connect to the contract by creating split sessions on-chain and reading contract events to verify settlement state.

## Data Models

### Split
- `id`: Unique session identifier
- `title`: Bill description
- `totalAmount`: Total bill amount
- `asset`: Stellar asset code (USDC, XLM, etc.)
- `participants`: Number of people splitting
- `amountPerPerson`: Calculated per-person amount
- `receiverAddress`: Stellar address receiving funds
- `status`: pending | active | completed | cancelled
- `payments`: Array of payment IDs
- `expiresAt`: Session expiration time

### Payment
- `id`: Unique payment identifier
- `splitId`: Associated split session ID
- `senderAddress`: Payer's Stellar address
- `amount`: Payment amount
- `asset`: Asset being paid
- `status`: pending | confirmed | failed
- `transactionHash`: Stellar transaction hash
- `ledger`: Stellar ledger number
- `confirmedAt`: Confirmation timestamp

## Security Notes

- Input validation on all endpoints
- No private key handling (frontend/wallet responsibility)
- Immutable payment records
- Session expiration after 7 days
- CORS enabled for multi-origin access

## Production Deployment

For production:

1. **Database**: Replace `InMemoryDatabaseService` with PostgreSQL/MongoDB
2. **Environment Variables**: Configure `.env` with proper network settings
3. **Authentication**: Add wallet signature verification
4. **Monitoring**: Integrate error tracking and metrics
5. **Rate Limiting**: Add request throttling
6. **Caching**: Implement Redis for status queries

## TypeScript

- Strict mode enabled
- Full type safety on all entities and DTOs
- No implicit any

## Contributing

Contributions welcome. Please ensure TypeScript compilation passes:

```bash
npm run typecheck
```


## Related Projects

- **Frontend**: `splitpay-frontend` - React/Next.js UI
- **Contracts**: `contracts/` - Soroban smart contract

