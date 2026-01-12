# TOKAMAK64 Developer Documentation

Technical documentation for integrating with and extending TOKAMAK64.

---

## 📚 Documentation Index

### By Integration Path

1. **[Direct RPC](#direct-rpc-integration)**
   - Fetching account data
   - Building transactions
   - Account parsing

2. **[FlatBuffers (jet)](#flatbuffers-integration)**
   - Cross-language serialization
   - Schema definitions
   - Code generation

3. **[FFI Bindings](#ffi-integration)**
   - Client-side computation
   - Flutter (Dart) bindings
   - WASM (JavaScript) bindings

---

## 📖 Core Documents

### [ARCHITECTURE.md](ARCHITECTURE.md)

**Technical overview** of codebase structure, data flow, and design decisions.

- Workspace structure and crate dependency graph
- Crate responsibilities (`curve`, `nucleus`, `program`, `jet`, `ffi`)
- Data flow: Client → RPC → Program → Nucleus → Action
- Design principles (blockchain-agnostic core, zero-copy access)
- Local compute via FFI/WASM for fee preview

**Best for:** Understanding how the system fits together.

---

### [INSTRUCTIONS.md](INSTRUCTIONS.md)

**Complete reference** for all 13 on-chain instructions.

- **Program ID** and discriminator format
- Account layouts for each instruction
- Instruction data formats
- Error conditions and codes
- Effects and validation rules

| Instructions | Purpose |
|--------------|---------|
| InitWallet, InitCharge | Account initialization |
| Charge, Discharge | Balance management |
| Infuse, Extract | Stablecoin conversion |
| Bind, Unbind, Rebind | Board movement |
| Compress, Vent | Pot manipulation |
| Overload, Claim | Reset operations |

**Best for:** Building transaction builders and SDKs.

---

### [INTEGRATION.md](INTEGRATION.md)

**Step-by-step integration guide** for client applications.

- Fetching account data via RPC
- Parsing account data (Wallet, Charge, Element, Board)
- **Account sizes** and memory layouts
- Building transactions with correct discriminators
- Calculating PDAs (Program Derived Addresses)
- Error handling patterns
- Fee preview using local computation

**Best for:** Building wallets, explorers, and trading interfaces.

---

## 🚀 Quick Start by Use Case

### Building a Wallet Interface

1. Read **[ARCHITECTURE.md](ARCHITECTURE.md#crate-responsibilities)** — Understand account types
2. Read **[INSTRUCTIONS.md](INSTRUCTIONS.md#wallet--balance-management)** — Account initialization and funding
3. Read **[INTEGRATION.md](INTEGRATION.md#fetching-account-data)** — Fetch and parse accounts
4. Reference: **[INSTRUCTIONS.md](INSTRUCTIONS.md#error-codes)** — Handle errors

### Building an Explorer

1. Read **[ARCHITECTURE.md](ARCHITECTURE.md)** — System overview
2. Read **[INTEGRATION.md](INTEGRATION.md#account-parsing)** — Parse account data and account sizes
3. Read **[INSTRUCTIONS.md](INSTRUCTIONS.md)** — Interpret transactions
4. Reference: **[../players/REFERENCE.md](../players/REFERENCE.md)** — Game mechanics for display

### Building a Trading Bot

1. Read **[../players/CONCEPTS.md](../players/CONCEPTS.md)** — Game mechanics
2. Read **[../players/STRATEGY.md](../players/STRATEGY.md)** — Strategic considerations
3. Read **[INTEGRATION.md](INTEGRATION.md#fee-preview)** — Local fee calculation
4. Reference: **[ARCHITECTURE.md](ARCHITECTURE.md#local-compute-wasmffi)** — Use FFI for simulation

### Building an SDK

1. Read **[INSTRUCTIONS.md](INSTRUCTIONS.md)** — Complete instruction reference
2. Read **[INTEGRATION.md](INTEGRATION.md#building-transactions)** — Transaction construction
3. Reference: **[ARCHITECTURE.md](ARCHITECTURE.md)** — Type sizes and layouts
4. Optional: **[INTEGRATION.md](INTEGRATION.md#flatbuffers-integration)** — Use jet crate

---

## 🔧 Technical Reference

### Program ID

**See:** [INSTRUCTIONS.md](INSTRUCTIONS.md#program-id)

```
DuJrE9ZB4TqcMByw9g4CiDQdNQosPQCQw2ECWGfLiyi
```

### Discriminators

All instructions use an 8-byte discriminator (little-endian u64):

| Index | Instruction | Discriminator |
|-------|-------------|--------------|
| 0 | InitWallet | `0` |
| 1 | InitCharge | `1` |
| 2 | Charge | `2` |
| 3 | Claim | `3` |
| 4 | Compress | `4` |
| 5 | Extract | `5` |
| 6 | Discharge | `6` |
| 7 | Rebind | `7` |
| 8 | Unbind | `8` |
| 9 | Bind | `9` |
| 10 | Overload | `10` |
| 11 | Infuse | `11` |
| 12 | Vent | `12` |

**See:** [INSTRUCTIONS.md](INSTRUCTIONS.md) for complete instruction reference.

### Account Sizes & PDA Seeds

**See:** [INTEGRATION.md](INTEGRATION.md#account-parsing) for complete account size information and interface definitions.

**See:** [ARCHITECTURE.md](ARCHITECTURE.md#pda-derivation) for PDA seed definitions.

---

## 📊 Data Flow

```
┌─────────────┐
│   Client    │ (Flutter/TS/etc)
└──────┬──────┘
       │ 1. Fetch accounts
       ▼
┌─────────────────────────────────────┐
│         Solana RPC                  │
│  getAccountInfo, getMultipleAccounts│
└──────┬──────────────────────────────┘
       │ 2. Build transaction
       ▼
┌─────────────────────────────────────┐
│      Client SDK                     │
│  (instruction builder, PDAs)        │
└──────┬──────────────────────────────┘
       │ 3. Submit transaction
       ▼
┌─────────────────────────────────────┐
│    program/src/lib.rs               │
│  process_instruction()              │
└──────┬──────────────────────────────┘
       │ 4. Parse & route
       ▼
┌─────────────────────────────────────┐
│  nucleus/src/action.rs              │
│  Execute game logic                 │
└─────────────────────────────────────┘
```

**See:** [ARCHITECTURE.md](ARCHITECTURE.md#data-flow) for detailed data flow explanation.

---

## 🎯 Fee Preview (Local Compute)

For fee simulation without touching the chain:

```
┌─────────────┐
│   Client    │
└──────┬──────┘
       │ Clone account data
       ▼
┌─────────────────────────────────────┐
│   ffi (WASM) / nucleus              │
│  rebind_fee(charge, src, dst)       │
│  (read-only, returns u64)           │
└─────────────────────────────────────┘
```

**See:** [ARCHITECTURE.md](ARCHITECTURE.md#local-compute-wasmffi) for FFI setup.

---

## 📋 Reference

### Player Documentation

Developers building interfaces should understand the game mechanics:

- **[../players/README.md](../players/README.md)** — Player documentation index
- **[../players/CONCEPTS.md](../players/CONCEPTS.md)** — How the game works
- **[../players/REFERENCE.md](../players/REFERENCE.md)** — Exact formulas, constants, and board layout
- **[../players/OPERATIONS.md](../players/OPERATIONS.md)** — User operations
- **[../players/STRATEGY.md](../players/STRATEGY.md)** — Strategic considerations

### Game Constants

**See:** [../players/REFERENCE.md](../players/REFERENCE.md#constants) for complete list of game constants.

Key constants include:
- `MAX_ATOMIC_NUMBER`: 26 (Fe at center)
- `MAX_SATURATION`: 100,663,296 (100% in Q8.24 fixed-point)
- `MIN_FEE`: 0.1 Gluon
- `DENOMINATOR`: 68,048,388,096 (fee calculation)
- `SPEED_DECAY_SLOTS`: 1024 (~51 seconds for full decay)

---

## 🔍 Common Tasks

### Calculate PDA Address

```typescript
import { PublicKey } from '@solana/web3.js';

function findWalletPda(authority: PublicKey, mint: PublicKey): PublicKey {
  const [pda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from('wallet'),
      authority.toBuffer(),
      mint.toBuffer(),
    ],
    PROGRAM_ID,
  );
  return pda;
}
```

**See:** [INTEGRATION.md](INTEGRATION.md) for more PDA examples.

### Parse Element Account

```typescript
function parseElement(data: Buffer): Element {
  return {
    pot: data.readBigUInt64LE(0),
    index: data.readBigUInt64LE(8),
    curve: {
      capacity: data.readBigUInt64LE(16),
      saturation: data.readUInt32LE(24),
    },
    coordinates: data.readBigUInt64LE(80),
  };
}
```

**See:** [INTEGRATION.md](INTEGRATION.md#account-parsing) for complete account parsing examples.

### Build Rebind Instruction

```typescript
function buildRebindInstruction(
  authority: PublicKey,
  charge: PublicKey,
  source: PublicKey,
  destination: PublicKey,
): TransactionInstruction {
  const data = Buffer.alloc(8);
  data.writeBigUInt64LE(7n, 0); // Discriminator for Rebind

  return new TransactionInstruction({
    keys: [
      { pubkey: authority, isSigner: true, isWritable: false },
      { pubkey: charge, isSigner: false, isWritable: true },
      { pubkey: source, isSigner: false, isWritable: true },
      { pubkey: destination, isSigner: false, isWritable: true },
    ],
    programId: PROGRAM_ID,
    data,
  });
}
```

**See:** [INTEGRATION.md](INTEGRATION.md#building-transactions) for more transaction building examples.

---

## 📝 Document Status

| Document | Last Updated | Status |
|----------|--------------|--------|
| ARCHITECTURE.md | 2026-01-13 | ✅ Current |
| INSTRUCTIONS.md | 2026-01-13 | ✅ Current |
| INTEGRATION.md | 2026-01-13 | ✅ Current |

---

## 🤝 Contributing

When modifying code:

1. **Backend changes**: Update relevant sections in ARCHITECTURE.md
2. **Instruction changes**: Update INSTRUCTIONS.md (account layouts, data formats)
3. **API changes**: Update INTEGRATION.md (examples, type sizes)
4. **Schema changes**: Regenerate FlatBuffers bindings in `jet/` crate

---

**TOKAMAK64** is built on Solana with Rust, using a blockchain-agnostic core for maximum flexibility and testability.
