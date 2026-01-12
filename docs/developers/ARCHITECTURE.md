# Architecture

Technical overview of TOKAMAK64's codebase structure, data flow, and design decisions.

## Workspace Structure

```
tokamak/
├── backend/
│   ├── curve/      # Bonding curve math (no deps)
│   ├── nucleus/    # Core types and game logic (no blockchain deps)
│   ├── program/    # Solana on-chain program
│   ├── jet/        # FlatBuffers serialization
│   └── ffi/        # Flutter/WASM bindings
├── schemas/        # FlatBuffers schema definitions
└── docs/           # Documentation
```

## Crate Dependency Graph

```
curve (bottom layer, no dependencies)
  │
  ↓
nucleus (core types + logic)
  │
  ├─────────────────┬────────────────┐
  ↓                 ↓                ↓
program           jet              ffi
(Solana)       (FlatBuffers)    (Flutter/WASM)
```

**Design principle**: `curve` and `nucleus` have no blockchain dependencies. This allows:
- Local simulation without Solana
- Client-side fee preview
- Testing without network

## Crate Responsibilities

### curve

**Purpose**: Sigmoid bonding curve mathematics.

| File | Contents |
|------|----------|
| `lut.rs` | Precomputed lookup table (65,536 entries) |
| `math.rs` | Fixed-point arithmetic, interpolation |
| `consts.rs` | LUT bounds, precision constants |

**Key functions**:
- `dx_for_dc(x0, s0, dc, cmax)` — Saturation change from capacity delta
- `dc_for_dx(x0, dx, cmax)` — Capacity cost for saturation change

**Fixed-point formats**:
- Q8.24 (u32): Saturation values [0, 6]
- Q16.48 (u64): Cumulative costs

### nucleus

**Purpose**: Core game types and logic, blockchain-agnostic.

| Module | Contents |
|--------|----------|
| `types.rs` | `Gluon`, `ElementIndex`, `Coordinates`, `Q824`, `Q1648` |
| `board.rs` | `Element`, `Curve`, `Board`, `Artefact` |
| `player.rs` | `Wallet`, `Charge` |
| `consts.rs` | Game constants (`MAX_ATOMIC_NUMBER`, `MIN_FEE`, etc.) |
| `fees.rs` | Fee calculation functions |
| `action.rs` | State mutation functions (`rebind`, `claim`, `compress`) |

**Key principle**: All types are `Pod` + `Copy` (via `bytemuck`), enabling zero-copy account access.

### program

**Purpose**: Solana on-chain program (BPF target).

| File | Contents |
|------|----------|
| `lib.rs` | Entrypoint, error handling |
| `instruction.rs` | `TokamakInstruction` enum, parsing |
| `accounts.rs` | Account validation |
| `addresses.rs` | PDA derivation |
| `processors/*.rs` | Instruction handlers |

See [Instructions](INSTRUCTIONS.md) for detailed instruction reference.

### jet

**Purpose**: FlatBuffers serialization for cross-language interop.

| File | Contents |
|------|----------|
| `lib.rs` | Public exports |
| `convert.rs` | Bidirectional type conversions |
| `generated/` | Auto-generated code (do not edit) |

**Schema files** in `/schemas/`:
- `types.fbs` — Primitive wrappers
- `board.fbs` — Board state types
- `player.fbs` — Player account types
- `game.fbs` — Root snapshot type
- `api.fbs` — API request/response types

### ffi

**Purpose**: Foreign function interface bindings.

| Feature | Target |
|---------|--------|
| `flutter` | Flutter via flutter_rust_bridge |
| `wasm` | WebAssembly via wasm-bindgen |

Exposes nucleus types and functions for client-side computation.

## Data Flow

### On-Chain Transaction

```
Client
  │
  │ 1. Build transaction
  ↓
┌─────────────────────────────────────┐
│         Solana RPC                  │
└─────────────────┬───────────────────┘
                  │ 2. Submit
                  ↓
┌─────────────────────────────────────┐
│    program/src/lib.rs               │
│    process_instruction()            │
└─────────────────┬───────────────────┘
                  │ 3. Parse discriminator
                  ↓
┌─────────────────────────────────────┐
│    instruction.rs                   │
│    TokamakInstruction::parse()      │
└─────────────────┬───────────────────┘
                  │ 4. Route to processor
                  ↓
┌─────────────────────────────────────┐
│    processors/mod.rs                │
│    (dispatch by instruction)        │
└─────────────────┬───────────────────┘
                  │ 5. Validate accounts
                  ↓
┌─────────────────────────────────────┐
│    accounts.rs                      │
│    (signer, PDA, owner checks)      │
└─────────────────┬───────────────────┘
                  │ 6. Execute game logic
                  ↓
┌─────────────────────────────────────┐
│    nucleus/src/action.rs            │
│    rebind(), claim(), compress()    │
└─────────────────────────────────────┘
```

### Client-Side Simulation

```
Client
  │
  │ Clone account data
  ↓
┌─────────────────────────────────────┐
│    ffi (WASM) / nucleus             │
│    rebind_fee(), fee_multiplier()   │
│    (read-only, returns preview)     │
└─────────────────────────────────────┘
```

No network call needed for fee preview. Client clones relevant account data and computes locally.

## Account Model

### PDA Derivation

All player accounts are PDAs (Program Derived Addresses):

```rust
// Wallet PDA
seeds = ["wallet", authority_pubkey, mint_pubkey]

// Charge PDA
seeds = ["charge", authority_pubkey, charge_index]
```

Bumps are deterministic—re-derived each time, not stored.

### Account Types

| Account | Size | Owner | Contents |
|---------|------|-------|----------|
| Wallet | 72 bytes | Program | balance, authority, mint, charge_count |
| Charge | 96 bytes | Program | balance, timestamp, index, share, authority, mint |
| Element | 128 bytes | Program | pot, index, curve, coordinates |
| Board | 64 bytes | Program | tvl, quantum_pocket, charge_count, quantum_index |
| Artefact | 136 bytes | Program | Snapshot of reset Element |

### Serialization

All account types use `bytemuck` for zero-copy access:

```rust
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Charge {
    pub balance: Gluon,
    pub timestamp: u64,
    pub index: ElementIndex,
    pub share: Q1648,
    pub authority: AddressBytes,
    pub mint: AddressBytes,
}
```

No deserialization overhead—account data is cast directly to typed references.

## Element Identity

Elements use a compound index encoding identity and versioning:

```
┌─────────────────────────────────────────────────────────┬──────────┐
│              Generation (56 bits)                       │ Atomic # │
│                                                         │ (8 bits) │
└─────────────────────────────────────────────────────────┴──────────┘
```

- **Atomic Number** (lower 8 bits): 1–26, identifies which Element
- **Generation** (upper 56 bits): Increments on each reset

This enables detecting stale references—a Charge can only claim from an Artefact with matching index.

## Fee Calculation

Fees are computed in `nucleus/src/fees.rs`:

```rust
pub fn rebind_fee(charge: &Charge, src: &Element, dst: &Element) -> Gluon {
    let distance = (dst.atomic_number() - src.atomic_number()).abs();
    let saturation = if inward { dst.saturation() } else { src.saturation() };

    base_fee(charge.balance, saturation, distance)
}

pub fn fee_multiplier(charge: &Charge, current_slot: u64) -> u64 {
    let delta = current_slot.saturating_sub(charge.timestamp);
    speed_decay(delta)
}
```

Total fee = `rebind_fee() × fee_multiplier()`

## Testing

### Unit Tests

```bash
cargo test -p nucleus    # Core logic
cargo test -p curve      # Math functions
cargo test -p jet        # Serialization round-trips
```

### Integration Tests

```bash
cargo test -p program    # Uses mollusk-svm for Solana simulation
```

### BPF Build

```bash
cargo build-sbf -p program
```

Output: `target/deploy/program.so`

## Design Decisions

### Why No Blockchain in nucleus?

Keeping nucleus blockchain-agnostic enables:
- Client-side fee preview without RPC calls
- Testing without Solana runtime
- Potential portability to other chains

### Why LUT for Sigmoid?

The sigmoid curve requires expensive operations (exp, division). A precomputed lookup table with interpolation:
- Fits in ~512KB
- O(1) lookup
- Sufficient precision (16-bit interpolation)

### Why bytemuck?

Zero-copy account access eliminates serialization overhead:
- No allocations in instruction handlers
- Direct memory mapping
- Type safety via `Pod` trait

### Why FlatBuffers?

For cross-language client support:
- Zero-copy reads
- Schema evolution (add fields without breaking)
- Code generation for Rust, TypeScript, etc.

## Next Steps

- **[Instructions](INSTRUCTIONS.md)** — Detailed instruction reference
- **[Integration](INTEGRATION.md)** — SDK and client integration guide
