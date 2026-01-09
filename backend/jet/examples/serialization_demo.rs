//! Demonstrates how *Args types are used to serialize nucleus types to FlatBuffers.

use flatbuffers::FlatBufferBuilder;
use jet::convert;
use jet::tokamak::board as fb_board;
use nucleus::board;
use nucleus::types::{Coordinates, ElementIndex};

fn main() {
    println!("=== FlatBuffers *Args Types Demo ===\n");

    // Create nucleus types
    let nucleus_curve = board::Curve {
        capacity: 1000,
        tvl: 500,
        pressure: 12345,
        saturation: 3 << 24,
        shares: 2 << 24,
    };

    let nucleus_board = board::Board {
        tvl: 10000,
        quantum_pocket: 5000,
        charge_count: 42,
        quantum_index: 7,
        _pad: [0; 3],
    };

    let nucleus_element = board::Element {
        pot: 2500,
        index: ElementIndex(0x0100000000000001), // Z=1, gen=1
        curve: nucleus_curve,
        coordinates: Coordinates(0xFF), // first 8 squares
    };

    println!("Step 1: Convert nucleus types to *Args");
    println!("---------------------------------------");

    // Convert to FlatBuffers Args
    let curve_args = convert::curve_args(&nucleus_curve);
    let board_args = convert::board_args(&nucleus_board);

    println!("✓ CurveArgs {{ capacity: {}, tvl: {} }}", curve_args.capacity, curve_args.tvl);
    println!("✓ BoardArgs {{ tvl: {}, charge_count: {} }}", board_args.tvl, board_args.charge_count);

    println!("\nStep 2: Build FlatBuffer using *Args");
    println!("-------------------------------------");

    // Create a FlatBufferBuilder
    let mut builder = FlatBufferBuilder::new();

    // Use *Args to create FlatBuffer offsets
    let curve_offset = fb_board::Curve::create(&mut builder, &curve_args);
    builder.finish(curve_offset, None);

    println!("✓ Built FlatBuffer with Curve::create(&mut builder, &curve_args)");
    println!("  Buffer size: {} bytes", builder.finished_data().len());

    println!("\nStep 3: Deserialize back to nucleus");
    println!("------------------------------------");

    // Read from buffer
    let buf = builder.finished_data();
    let fb_curve = flatbuffers::root::<fb_board::Curve>(buf).unwrap();

    // Convert back using From trait
    let roundtrip: board::Curve = fb_curve.into();

    println!("✓ Deserialized: capacity={}, tvl={}", roundtrip.capacity, roundtrip.tvl);
    assert_eq!(nucleus_curve.capacity, roundtrip.capacity);
    assert_eq!(nucleus_curve.tvl, roundtrip.tvl);

    println!("\n=== Key Points ===");
    println!("• *Args = Simple structs with public fields");
    println!("• Used as input to Type::create() for serialization");
    println!("• Generated automatically by flatc for each table");
    println!("• jet::convert provides nucleus → *Args helper functions");
}
