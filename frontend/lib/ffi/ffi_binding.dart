/// FFI binding initialization for TOKAMAK64 Rust backend.
///
/// This module provides access to the tokamak-ffi crate via flutter_rust_bridge.
/// Generated bindings will be placed in [lib/generated/] by the code generator.

import 'dart:ffi';

export 'generated/tokamak_ffi_bindings.dart';

/// Load the native library
///
/// Desktop platforms load the dylib/so/dll directly.
/// Mobile platforms (iOS/Android) would use different loading strategies.
DynamicLibrary loadNativeLibrary() {
  // On macOS/Linux/Windows, load from the system library path.
  // For development, you'll need to ensure the dylib is in:
  // - macOS: ~/Library/Developer/Xcode/DerivedData/tokamak-xxx/Build/Products/Release
  // - Linux: ~/.cargo/target/release
  // - Windows: %USERPROFILE%\.cargo\target\release
  //
  // For production, bundle the dylib in the application bundle/resources.
  
  final String libName = switch (Abi.current()) {
    Abi.macosArm64 || Abi.macosX64 => 'libtokamak_ffi.dylib',
    Abi.linuxArm64 || Abi.linuxX64 => 'libtokamak_ffi.so',
    Abi.windowsArm64 || Abi.windowsX64 => 'tokamak_ffi.dll',
    _ => throw UnsupportedError('Unsupported ABI: ${Abi.current()}'),
  };

  return DynamicLibrary.open(libName);
}
