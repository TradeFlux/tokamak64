# curve

Bonding curve LUT for TOKAMAK64. For game design, see the [main README](../../README.md).

## Fixed-Point Formats

| Type | Format | Usage |
|------|--------|-------|
| `x` | Q8.24 (u32) | Saturation [0, 6] |
| `s` | Q16.48 (u64) | Cumulative cost |
| `c` | u64 | Capacity (Gluon) |

Deltas use two's complement for negative values.

## Public API

### Constants (`consts`)
- `LUT_X_MIN`, `LUT_X_MAX` — Domain bounds (Q8.24)
- `LUT_S_MAX` — Maximum cumulative cost (Q16.48)

The sigmoid curve has its inflection point at x=3.0.

### Functions
#### `dx_for_dc(x0, s0, dc, cmax) -> (u32, u64)`
Calculate saturation change from capacity delta. Returns `(dx, ds)`.

#### `dc_for_dx(x0, dx, cmax) -> u64`
Calculate capacity cost for a saturation change.

Both functions clamp results to stay within `[LUT_X_MIN, LUT_X_MAX]`.

## LUT Generation

```bash
cargo run -p curve --bin lutgen --release > curve/src/lut.rs
```
