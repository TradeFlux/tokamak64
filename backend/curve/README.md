# curve

A small math crate that provides a fixed lookup table (LUT) for a symmetric
sigmoid‑shaped curve and a set of deterministic mapping helpers. It is designed
for environments like smart contracts where inputs are sanitized and runtime
checks are avoided.

The LUT stores cumulative cost values over a symmetric domain in fixed‑point:
- `x` is Q16.16 (integer steps represent 1/65536 units)
- `s` is Q16.48 (high‑precision cumulative cost)

## What’s in the crate

- `lut`: the generated LUT data and domain constants (`LUT_X_MIN`, `LUT_X_MAX`,
  `LUT_S_MAX`).
- `math`: public mapping helpers that convert between deltas in capacity,
  cumulative cost, and `x`.
- `lutgen`: a generator binary that rebuilds the LUT source file.

## Public API (math)

All functions assume in‑domain inputs and do not perform runtime validation.
See each doc comment for constraints.

- `dx_for_dc(x0, s0, dc, cmax) -> i32`
- `dc_for_dx(x0, dx, cmax) -> i64`

## LUT generation

Rebuild the LUT source:

```bash
cargo run -p curve --bin lutgen --release > lut.rs
```

## Testing

```bash
cargo test -p curve
```

