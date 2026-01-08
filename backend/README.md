# TOKAMAK64 Backend

Rust implementation of TOKAMAK64. For game design and mechanics, see the [main README](../README.md).

## Crates

```
backend/
├── curve/     # Bonding curve LUT and math
├── nucleus/   # Core types and logic (no blockchain deps)
└── program/   # Solana on-chain program
```

| Crate | Purpose |
|-------|---------|
| `curve` | Precomputed sigmoid LUT, saturation/pressure mapping |
| `nucleus` | `Charge`, `Element`, `Board`, fees, actions |
| `program` | Solana entrypoint, instruction dispatch |

## Building

```bash
cargo build --workspace           # Build all
cargo test --workspace            # Test all
cargo build-sbf -p program        # Solana BPF target
```

## Implementation Notes

**Speed Tax Timing**: `MAX_DELTA_TIMESTAMP = 1024` slots. At ~400ms/slot, full decay takes ~7 minutes.

**Fee Routing (Rebind)**:
- Inward (`src.index < dst.index`): fee → destination pot
- Outward (`src.index > dst.index`): fee → source pot
