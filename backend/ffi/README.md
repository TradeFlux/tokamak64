# tokamak_ffi

FFI bindings for TOKAMAK64 nucleus crate. Exposes transparent types and core game mechanics for Dart/Flutter integration.

## Types

All exported types are PODs (Plain Old Data) with `#[repr(C)]` or `#[repr(transparent)]`:

- **Value types**: `Gluon` (u64), `Q824` (u32), `Q1648` (u64), `AddressBytes` ([u8; 32])
- **Core entities**: `ElementIndex`, `Coordinates`, `Element`, `Artefact`, `Charge`, `Wallet`, `Board`, `Curve`

No serialization required; safe to pass directly across FFI boundary.

## Functions

- **Game mechanics**: `rebind()`, `claim()`, `compress()`
- **Fees**: `bind_fee()`, `unbind_fee()`, `rebind_fee()`, `compression_fee()`, `fee_multiplier()`
- **Utilities**: `round_divide()`

## Building

```bash
# Check
cargo check -p tokamak_ffi

# Release dylib
cargo build -p tokamak_ffi --release
# Output: target/release/libtokamak_ffi.dylib
```

## Flutter Integration

Configure `flutter_rust_bridge.yaml` in your Flutter project to generate Dart bindings from this crate's public API. See the YAML config in this directory for available build targets.
