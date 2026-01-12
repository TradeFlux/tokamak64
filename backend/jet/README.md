# jet

FlatBuffers serialization bindings for TOKAMAK64. Enables cross-language interop (Rust, TypeScript, etc.).

For game design, see the [main README](../../README.md).

## Structure

| File | Purpose |
|------|---------|
| `lib.rs` | Main exports, pub re-exports of generated code |
| `convert.rs` | Bidirectional type conversions (nucleus ↔ FlatBuffers) |
| `generated/` | Auto-generated from `../schemas/*.fbs` (do not edit) |

## Schemas

Schema files live in `/schemas/`:

| File | Contents |
|------|----------|
| `types.fbs` | `AddressBytes`, primitives |
| `board.fbs` | `Curve`, `Element`, `Board`, `Artefact` |
| `player.fbs` | `Wallet`, `Charge` |
| `game.fbs` | `Game` (root snapshot type) |
| `api.fbs` | API request/response types |

## Building

```bash
# Regenerate from schemas
cargo build -p jet

# Run tests
cargo test -p jet
```

Schema changes in `/schemas/` trigger code regeneration automatically.

## Usage

```rust
use jet::convert::*;
use flatbuffers::FlatBufferBuilder;

// Serialize: nucleus → FlatBuffers
let element: nucleus::Element = /* ... */;
let mut builder = FlatBufferBuilder::new();
// ... build and serialize

// Deserialize: FlatBuffers → nucleus
let fb_element = root_as_element(bytes)?;
let element: nucleus::Element = fb_element.into();
```

## Dependencies

- **Imports**: `nucleus`, `curve`, `flatbuffers`
- **Exports to**: FFI, TypeScript SDK, external clients
