# Integration Guide

How to integrate with TOKAMAK64 from client applications.

## Overview

TOKAMAK64 provides multiple integration paths:

| Path | Use Case | Language |
|------|----------|----------|
| Direct RPC | Full control, custom clients | Any |
| FlatBuffers (jet) | Cross-language state parsing | Rust, TypeScript, etc. |
| FFI (ffi) | Client-side computation | Flutter (Dart), WASM (JS) |

## Direct RPC Integration

### Fetching Account Data

Use Solana RPC to fetch account data:

```typescript
// Fetch multiple accounts in one call
const accounts = await connection.getMultipleAccountsInfo([
  walletPda,
  chargePda,
  elementPda,
  boardPda,
]);
```

### Account Parsing

Account data is raw bytes in `Pod` layout. Parse using the known struct sizes:

```typescript
// Wallet: 72 bytes
interface Wallet {
  balance: bigint;        // u64 (8 bytes)
  authority: Uint8Array;  // [u8; 32]
  mint: Uint8Array;       // [u8; 32]
  chargeCount: bigint;    // u64 (8 bytes) - Note: may have padding
}

// Charge: 96 bytes
interface Charge {
  balance: bigint;        // u64
  timestamp: bigint;      // u64
  index: bigint;          // u64 (ElementIndex)
  share: bigint;          // u64 (Q16.48)
  authority: Uint8Array;  // [u8; 32]
  mint: Uint8Array;       // [u8; 32]
}

// Element: 128 bytes
interface Element {
  pot: bigint;            // u64
  index: bigint;          // u64 (ElementIndex)
  curve: Curve;           // Curve struct
  coordinates: bigint;    // u64
}
```

### Building Transactions

Construct instructions with the correct discriminator and accounts:

```typescript
import { TransactionInstruction, PublicKey } from '@solana/web3.js';

function buildRebindInstruction(
  authority: PublicKey,
  charge: PublicKey,
  source: PublicKey,
  destination: PublicKey,
): TransactionInstruction {
  // Discriminator: 7 (Rebind)
  const data = Buffer.alloc(8);
  data.writeBigUInt64LE(7n, 0);

  return new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: authority, isSigner: true, isWritable: false },
      { pubkey: charge, isSigner: false, isWritable: true },
      { pubkey: source, isSigner: false, isWritable: true },
      { pubkey: destination, isSigner: false, isWritable: true },
    ],
    data,
  });
}
```

### PDA Derivation

Derive PDAs client-side:

```typescript
function deriveWalletPda(authority: PublicKey, mint: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('wallet'), authority.toBuffer(), mint.toBuffer()],
    PROGRAM_ID,
  );
}

function deriveChargePda(authority: PublicKey, index: bigint): [PublicKey, number] {
  const indexBuffer = Buffer.alloc(8);
  indexBuffer.writeBigUInt64LE(index, 0);

  return PublicKey.findProgramAddressSync(
    [Buffer.from('charge'), authority.toBuffer(), indexBuffer],
    PROGRAM_ID,
  );
}
```

---

## FlatBuffers Integration (jet)

Use FlatBuffers for structured state parsing and cross-language support.

### Schema Compilation

Generate bindings for the target language:

```bash
# TypeScript
flatc --ts -o generated/ schemas/game.fbs

# Python
flatc --python -o generated/ schemas/game.fbs

# Rust (automatic via jet crate build)
cargo build -p jet
```

### TypeScript Usage

```typescript
import { Game } from './generated/tokamak/game';
import * as flatbuffers from 'flatbuffers';

// Parse game state from bytes
function parseGameState(bytes: Uint8Array): Game {
  const buf = new flatbuffers.ByteBuffer(bytes);
  return Game.getRootAsGame(buf);
}

// Access nested data
const game = parseGameState(data);
const board = game.board();
const elements = [];
for (let i = 0; i < game.elementsLength(); i++) {
  elements.push(game.elements(i));
}
```

### Rust Usage

```rust
use jet::{Game, Element};
use flatbuffers::root;

// Parse from bytes
let game = root::<Game>(bytes)?;

// Access data
let board = game.board().unwrap();
let tvl = board.tvl();

for i in 0..game.elements_len() {
    let element = game.elements().get(i);
    println!("Element {}: pot = {}", element.index(), element.pot());
}
```

---

## FFI Integration (ffi)

Use FFI for client-side computation without network calls.

### Flutter (Dart)

The `ffi` crate uses `flutter_rust_bridge` for Dart bindings.

**Setup**:
1. Add the generated Dart files to the Flutter project
2. Configure `flutter_rust_bridge.yaml`
3. Build the native library

**Usage**:
```dart
import 'package:tokamak_ffi/tokamak_ffi.dart';

// Calculate fee preview
final fee = rebindFee(charge, sourceElement, destElement);
final multiplier = feeMultiplier(charge, currentSlot);
final totalFee = fee * multiplier;

// Check if move is affordable
if (charge.balance >= totalFee) {
  // Proceed with transaction
}
```

### WASM (JavaScript)

The `ffi` crate supports WASM via `wasm-bindgen`.

**Build**:
```bash
wasm-pack build backend/ffi --target web --features wasm
```

**Usage**:
```javascript
import init, { rebind_fee, fee_multiplier } from 'tokamak-ffi';

await init();

// Calculate fees client-side
const baseFee = rebind_fee(chargeData, srcData, dstData);
const multiplier = fee_multiplier(chargeData, currentSlot);
const totalFee = baseFee * multiplier;
```

### Client-Side Simulation

Clone account data and compute locally:

```typescript
// 1. Fetch current state
const [charge, src, dst] = await connection.getMultipleAccountsInfo([
  chargePda, srcElementPda, dstElementPda
]);

// 2. Parse into typed structures
const chargeData = parseCharge(charge.data);
const srcData = parseElement(src.data);
const dstData = parseElement(dst.data);

// 3. Compute fee locally (no RPC)
const fee = rebindFee(chargeData, srcData, dstData);
const multiplier = feeMultiplier(chargeData, currentSlot);

// 4. Display to user before transaction
console.log(`This move will cost ${fee * multiplier} Gluon`);
```

---

## Common Patterns

### Atomic Reset Trigger

Bundle Rebind + Overload to atomically trigger a reset:

```typescript
const tx = new Transaction();

// 1. Rebind that pushes saturation over threshold
tx.add(buildRebindInstruction(authority, charge, src, dst));

// 2. Immediately trigger overload
tx.add(buildOverloadInstruction(authority, charge, dst, artefact, board));

// Submit as single transaction
await sendAndConfirmTransaction(connection, tx, [authorityKeypair]);
```

### Watching for Resets

Subscribe to account changes to detect resets:

```typescript
connection.onAccountChange(elementPda, (accountInfo) => {
  const element = parseElement(accountInfo.data);

  // Check if generation changed (reset occurred)
  if (element.generation > lastKnownGeneration) {
    console.log('Reset detected!');
    // Fetch artefact, submit claim if needed
  }
});
```

### Fee Preview UI

Display fee preview before user confirms:

```typescript
async function previewMove(charge: PublicKey, dst: PublicKey): Promise<FeePreview> {
  // Fetch current state
  const [chargeAccount, srcAccount, dstAccount] = await fetchAccounts(charge, dst);

  // Compute locally
  const baseFee = rebindFee(chargeAccount, srcAccount, dstAccount);
  const multiplier = feeMultiplier(chargeAccount, currentSlot);
  const totalFee = baseFee * multiplier;

  // Compute time until optimal (1x multiplier)
  const slotsUntilOptimal = 1024 - (currentSlot - chargeAccount.timestamp);
  const secondsUntilOptimal = slotsUntilOptimal * 0.05; // 50ms slots

  return {
    baseFee,
    multiplier,
    totalFee,
    optimalFee: baseFee, // At 1x multiplier
    secondsUntilOptimal: Math.max(0, secondsUntilOptimal),
  };
}
```

---

## Error Handling

### Transaction Errors

Program errors return custom error codes. Map them to messages:

```typescript
const ERROR_MESSAGES: Record<number, string> = {
  0: 'Invalid instruction data',
  1: 'Invalid account data',
  2: 'Missing required signature',
  3: 'Illegal account owner',
  4: 'Insufficient funds',
  5: 'Account already initialized',
  6: 'Charge is bound (must unbind first)',
  7: 'Charge is unbound (must bind first)',
  8: 'Not an edge element',
  9: 'Elements not adjacent',
  10: 'Index mismatch',
  11: 'Element not saturated',
  12: 'Invalid compression direction',
  13: 'Invalid authority',
  14: 'Invalid mint',
  15: 'Artefact already exists',
  16: 'Already claimed',
};

function parseError(err: any): string {
  const code = err?.InstructionError?.[1]?.Custom;
  return ERROR_MESSAGES[code] ?? 'Unknown error';
}
```

### Simulation Before Submit

Use `simulateTransaction` to catch errors before submitting:

```typescript
const simulation = await connection.simulateTransaction(tx);

if (simulation.value.err) {
  const errorMessage = parseError(simulation.value.err);
  throw new Error(`Transaction would fail: ${errorMessage}`);
}

// Safe to submit
await sendAndConfirmTransaction(connection, tx, [signer]);
```

---

## Testing

### Local Validator

Test against a local Solana validator:

```bash
# Start local validator
solana-test-validator

# Deploy program
solana program deploy target/deploy/program.so

# Run integration tests
cargo test -p program
```

### Mollusk SVM

The program tests use `mollusk-svm` for fast, in-process testing:

```rust
use mollusk_svm::Mollusk;

#[test]
fn test_rebind() {
    let mollusk = Mollusk::new(&program_id, "program");

    let result = mollusk.process_instruction(
        &instruction,
        &accounts,
    );

    assert!(result.is_ok());
}
```

---

## Resources

- **Program ID**: `DuJrE9ZB4TqcMByw9g4CiDQdNQosPQCQw2ECWGfLiyi`
- **[Instructions Reference](INSTRUCTIONS.md)** — Complete instruction documentation
- **[Architecture](ARCHITECTURE.md)** — Crate structure and data flow
- **Schema files**: `/schemas/*.fbs`
