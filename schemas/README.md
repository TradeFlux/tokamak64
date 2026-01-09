# TOKAMAK64 FlatBuffer Schemas

This directory contains FlatBuffer schema definitions (`.fbs` files) for serializing and deserializing TOKAMAK64 game state.

## Schema Hierarchy

```
schemas/
├── types.fbs      # Fundamental types (AddressBytes, ElementIndex, Coordinates)
├── board.fbs      # Board state (Curve, Element, Board, Artefact)
├── player.fbs     # Player accounts (Wallet, Charge)
└── game.fbs       # Root table (Game) - complete board snapshot
```

## Schema Files

### `types.fbs`
Defines core primitive types used throughout the system:
- **AddressBytes**: 32-byte Solana public key encoded as 4 `uint64` words (struct)

Note: ElementIndex and Coordinates are represented as `uint64` primitives directly in the schemas rather than nested types.

### `board.fbs`
Defines board-related state:
- **Curve**: Bonding curve state (capacity, TVL, pressure, saturation, shares)
- **Element**: Single board element with pot, index, curve, and coordinates
- **Board**: Global singleton tracking TVL, quantum pocket, charge count, quantum index
- **Artefact**: Snapshot of reset element for reward distribution

### `player.fbs`
Defines player account types (stored separately, not included in `Game` root):
- **Wallet**: Liquid Gluon account with balance, authority, mint, charge count
- **Charge**: Allocated Gluon bound to element with balance, timestamp, index, share, authority, mint

### `game.fbs`
Root schema defining complete observable game state:
- **Game**: Root table containing Board + all Elements + metadata
  - Includes global board state
  - Includes all 26 elements (Z=1 to Z=26)
  - Includes snapshot timestamp and optional slot number
  - **Does not include** player accounts (Charge/Wallet) which are stored separately

## Usage

### Compilation

Compile schemas to your target language using `flatc`:

```bash
# Generate Rust code
flatc --rust -o generated/ game.fbs

# Generate TypeScript code
flatc --ts -o generated/ game.fbs

# Generate Python code
flatc --python -o generated/ game.fbs
```

### Design Notes

1. **Fixed-size arrays as structs**: Types like `AddressBytes` ([u8;32]) are encoded as structs with word-sized fields for efficient alignment and access.

2. **Primitives vs nested types**: Single-value types (`ElementIndex`, `Coordinates`) use `uint64` primitives directly rather than wrapping structs, reducing nesting complexity.

3. **Struct vs Table**:
   - `struct`: Fixed-size, inline, no versioning (AddressBytes only)
   - `table`: Variable-size, versioned, can add fields (all state types)

4. **Root type**: `Game` is the root type, representing a complete board snapshot. Player accounts are accessed separately by their account addresses.

5. **Separation of concerns**: Board state (public, observable) is separate from player state (private, account-specific).

## Schema Versioning

FlatBuffers supports schema evolution:
- New fields can be added to tables without breaking compatibility
- Fields can be deprecated but not removed
- Structs cannot be versioned (fixed layout)

## Integration with Rust Types

These schemas mirror the types defined in:
- `backend/nucleus/src/types.rs`
- `backend/nucleus/src/board.rs`
- `backend/nucleus/src/player.rs`

The binary layout in FlatBuffers may differ from Rust's `#[repr(C)]` types, but semantic equivalence is maintained.
