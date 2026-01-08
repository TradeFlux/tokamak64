# nucleus

Core game logic for TOKAMAK64. Blockchain-agnostic. For game design, see the [main README](../../README.md).

## Modules

| Module | Contents |
|--------|----------|
| `types` | `Gluon`, `ElementIndex`, `Coordinates`, `Q824`, `Q1648` |
| `board` | `Element`, `Curve`, `Board`, `Artefact` |
| `player` | `Wallet`, `Charge` |
| `consts` | `MAX_ATOMIC_NUMBER`, `MIN_FEE`, `MAX_SPEED_MULTIPLIER`, etc. |
| `fees` | `injection_fee`, `ejection_fee`, `rebind_fee`, `compression_fee`, `fee_multiplier` |
| `action` | `rebind`, `claim`, `compress` |

## Feature Flags

- `bytemuck` (default): Pod/Zeroable derives for zero-copy account access

## Usage

```rust
use nucleus::{
    action::rebind,
    fees::{rebind_fee, fee_multiplier},
};

let base = rebind_fee(&charge, &src, &dst);
let multiplier = fee_multiplier(&charge, current_slot);
let total_fee = base * multiplier;

rebind(&mut charge, &mut src, &mut dst);
```
