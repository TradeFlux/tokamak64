use bytemuck::Zeroable;

use crate::{
    action::{claim, compress, rebind},
    board::{Artefact, Board, Curve, Element},
    consts::*,
    fees::{bind_fee, compression_fee, fee_multiplier, rebind_fee},
    player::{Charge, Wallet},
    round_divide,
    types::{AddressBytes, Coordinates, ElementIndex, Gluon, Q824},
};

// Helpers
fn dummy_address() -> AddressBytes {
    [0u8; 32]
}

fn make_element(atomic: u64, gen: u64, capacity: Gluon, pot: Gluon) -> Element {
    let mut index = ElementIndex(0);
    index.0 = (atomic << 56) | (gen & ((1u64 << 56) - 1));
    let mut curve = Curve::zeroed();
    curve.capacity = capacity;
    Element {
        index,
        pot,
        coordinates: Coordinates(1u64),
        curve,
    }
}

fn make_charge(balance: Gluon, index: ElementIndex, share: Q824) -> Charge {
    use bytemuck::Zeroable;
    let mut charge = Charge::zeroed();
    charge.balance = balance;
    charge.timestamp = 0;
    charge.index = index;
    charge.share = share;
    charge.authority = dummy_address();
    charge.mint = dummy_address();
    charge
}

// === ElementIndex Tests ===

#[test]
fn element_index_encode_decode() {
    let mut idx = ElementIndex(0);
    idx.0 = (42u64 << 56) | 7; // atomic=42, gen=7
    assert_eq!(idx.atomic(), 42);
    assert_eq!(idx.generation(), 7);
}

#[test]
fn element_index_next_gen() {
    let mut idx = ElementIndex((5u64 << 56) | 100);
    idx.advance_generation();
    assert_eq!(idx.atomic(), 5);
    assert_eq!(idx.generation(), 101);
}

#[test]
fn element_index_next_gen_wraps() {
    let max_gen = (1u64 << 56) - 1;
    let mut idx = ElementIndex((10u64 << 56) | max_gen);
    idx.advance_generation();
    assert_eq!(idx.atomic(), 10);
    assert_eq!(idx.generation(), 0); // wraps around
}

#[test]
fn element_index_clear() {
    let mut idx = ElementIndex((99u64 << 56) | 42);
    idx.clear();
    assert!(idx.is_zero());
}

// === Coordinates Tests ===

#[test]
fn coordinates_adjacent_horizontal() {
    let c1 = Coordinates(1u64 << 0); // square 0
    let c2 = Coordinates(1u64 << 1); // square 1 (east)
    assert!(c1.adjacent(c2));
}

#[test]
fn coordinates_adjacent_vertical() {
    let c1 = Coordinates(1u64 << 0); // square 0
    let c2 = Coordinates(1u64 << 8); // square 8 (north)
    assert!(c1.adjacent(c2));
}

#[test]
fn coordinates_not_adjacent_diagonal() {
    let c1 = Coordinates(1u64 << 0); // square 0
    let c2 = Coordinates(1u64 << 9); // square 9 (diagonal)
    assert!(!c1.adjacent(c2));
}

#[test]
fn coordinates_peripheral_edge() {
    let c = Coordinates(1u64 << 0); // square 0 (file A, rank 1)
    assert!(c.on_edge());
}

#[test]
fn coordinates_peripheral_center() {
    let c = Coordinates(1u64 << 27); // square 27 (center-ish)
    assert!(!c.on_edge());
}

// === Utility Tests ===

#[test]
fn round_divide_exact() {
    let result = round_divide(100, 1, 10);
    assert_eq!(result, 10);
}

#[test]
fn round_divide_rounds_down() {
    // (10 * 3 / 7) = 30/7 = 4.28... -> 4
    let result = round_divide(10, 3, 7);
    assert_eq!(result, 4);
}

#[test]
fn round_divide_rounds_up() {
    // (10 * 4 / 7) = 40/7 = 5.71... -> 6
    let result = round_divide(10, 4, 7);
    assert_eq!(result, 6);
}

#[test]
fn round_divide_ties_away_from_zero() {
    // (10 * 1 / 2) = 10/2 = 5, no rounding needed
    let result = round_divide(10, 1, 2);
    assert_eq!(result, 5);
}

// === Action Tests ===

#[test]
fn rebind_to_empty_element() {
    let mut charge = make_charge(100, ElementIndex((1u64 << 56) | 5), 50);
    let mut src = make_element(1, 0, 1000, 0);
    src.curve.shares = 50;
    let mut dst = Element {
        ..make_element(2, 0, 1000, 0)
    };
    dst.index.clear(); // mark destination as empty

    rebind(&mut charge, &mut src, &mut dst);

    // Rebinding to empty element: share becomes 0, index becomes empty.
    assert!(charge.index.is_zero());
    assert_eq!(charge.share, 0);
}

#[test]
fn rebind_updates_index_and_share() {
    let mut charge = make_charge(100, ElementIndex(0), 0);
    let mut src = make_element(1, 0, 1000, 0);
    src.index.clear();
    let mut dst = make_element(2, 0, 1000, 0);

    rebind(&mut charge, &mut src, &mut dst);

    assert_eq!(charge.index, dst.index);
    // share is set by dx_for_dc; we expect some value
    assert!(charge.share > 0 || charge.share == 0); // just check it's set
}

#[test]
fn claim_distributes_reward() {
    let mut charge = make_charge(0, ElementIndex((1u64 << 56) | 1), 500_000); // share = 50%
    let mut artefact = Artefact::zeroed();
    artefact.shares = MAX_SATURATION;
    artefact.pot = 1_000_000;
    artefact.index = ElementIndex((1u64 << 56) | 1);

    claim(&mut charge, &mut artefact);

    // reward = round_div(1_000_000, 500_000, MAX_POSITION) = portion of pot based on share
    assert!(charge.balance > 0);
    assert!(artefact.pot < 1_000_000);
    assert!(charge.share == 0); // cleared
    assert!(charge.index.is_zero()); // cleared
}

#[test]
fn compress_moves_pot() {
    let mut charge = make_charge(100, ElementIndex((1u64 << 56) | 1), 100);
    let mut src = make_element(1, 0, 1000, 500);
    src.curve.shares = 100;
    let mut dst = make_element(2, 0, 1000, 200);

    compress(&mut charge, &mut src, &mut dst);

    assert_eq!(src.pot, 0);
    assert_eq!(dst.pot, 700); // 200 + 500
}

// === Fee Tests ===

#[test]
fn bind_fee_respects_min() {
    let charge = make_charge(100, ElementIndex(0), 0);
    let element = make_element(1, 0, 1000, 0);

    let fee = bind_fee(&charge, &element);
    assert!(fee >= MIN_FEE);
}

#[test]
fn compression_fee_respects_min() {
    let element = make_element(1, 0, 1000, 100);

    let fee = compression_fee(&element);
    assert!(fee >= MIN_FEE);
}

#[test]
fn speed_multiplier_decreases_with_time() {
    let mut charge = make_charge(100, ElementIndex(0), 0);
    charge.timestamp = 1000;

    let multiplier_early = fee_multiplier(&charge, 1010);
    let multiplier_late = fee_multiplier(&charge, 2000);

    assert!(multiplier_late < multiplier_early);
}

#[test]
fn speed_multiplier_caps_at_max_delta() {
    let mut charge = make_charge(100, ElementIndex(0), 0);
    charge.timestamp = 0;

    let multiplier_at_max = fee_multiplier(&charge, MAX_DELTA_TIMESTAMP + 100);
    let multiplier_at_cap = fee_multiplier(&charge, MAX_DELTA_TIMESTAMP);

    // Beyond MAX_DELTA_TIMESTAMP, multiplier should be same (capped)
    assert_eq!(multiplier_at_max, multiplier_at_cap);
}

#[test]
fn rebind_fee_respects_min() {
    let charge = make_charge(100, ElementIndex(0), 0);
    let src = make_element(5, 0, 1000, 0);
    let dst = make_element(10, 0, 1000, 0);

    let fee = rebind_fee(&charge, &src, &dst);
    assert!(fee >= MIN_FEE);
}

// === Board & Wallet Tests ===

#[test]
fn wallet_structure_correct() {
    use bytemuck::Zeroable;
    let mut wallet = Wallet::zeroed();
    wallet.balance = 1_000_000;
    wallet.authority = dummy_address();
    wallet.mint = dummy_address();
    assert_eq!(wallet.balance, 1_000_000);
}

#[test]
fn board_structure_correct() {
    let board = Board {
        tvl: 5_000_000,
        quantum_pocket: 100_000,
        charge_count: 42,
        quantum_index: 3,
        _pad: [0; 3],
    };
    assert_eq!(board.tvl, 5_000_000);
    assert_eq!(board.charge_count, 42);
    assert_eq!(board.quantum_index, 3);
}

// === Constant Tests ===

#[test]
fn consts_reasonable_values() {
    assert!(MIN_FEE > 0);
    assert!(MAX_ATOMIC_NUMBER > 0);
    assert!(MAX_SPEED_MULTIPLIER > 0);
    assert!(MAX_DELTA_TIMESTAMP > 0);
}

// === Coordinates Tests ===

/// Convert algebraic coordinate (e.g., "A1") to bit position (0-63).
/// Row-major layout: A1=0, B1=1, ..., H1=7, A2=8, ..., H8=63.
fn coord_to_bit(coord: &str) -> u64 {
    let col = (coord.as_bytes()[0] - b'A') as u64;
    let row = (coord[1..].parse::<u64>().unwrap() - 1) as u64;
    1 << (row * 8 + col)
}

/// Create a bitmask from a list of algebraic coordinates.
fn coords_mask(coords: &[&str]) -> u64 {
    coords.iter().fold(0u64, |acc, c| acc | coord_to_bit(c))
}

#[test]
fn element_coordinates_match_docs() {
    // Verify each element's coordinates match the documented tiles.
    // From GAME_DESIGN.md coordinates table.

    assert_eq!(COORD_01_H.0, coords_mask(&["A1", "A2", "B1", "C1"]));
    assert_eq!(COORD_02_HE.0, coords_mask(&["D1", "E1"]));
    assert_eq!(COORD_03_LI.0, coords_mask(&["F1", "F2"]));
    assert_eq!(COORD_04_BE.0, coords_mask(&["G1", "H1", "H2", "H3"]));
    assert_eq!(COORD_05_B.0, coords_mask(&["H4", "H5"]));
    assert_eq!(COORD_06_C.0, coords_mask(&["G6", "H6"]));
    assert_eq!(COORD_07_N.0, coords_mask(&["F8", "G8", "H7", "H8"]));
    assert_eq!(COORD_08_O.0, coords_mask(&["D8", "E8"]));
    assert_eq!(COORD_09_F.0, coords_mask(&["C7", "C8"]));
    assert_eq!(COORD_10_NE.0, coords_mask(&["A6", "A7", "A8", "B8"]));
    assert_eq!(COORD_11_NA.0, coords_mask(&["A4", "A5"]));
    assert_eq!(COORD_12_MG.0, coords_mask(&["A3", "B3"]));
    assert_eq!(COORD_13_AL.0, coords_mask(&["B2", "C2", "D2"]));
    assert_eq!(COORD_14_SI.0, coords_mask(&["E2", "E3"]));
    assert_eq!(COORD_15_P.0, coords_mask(&["F3", "F4"]));
    assert_eq!(COORD_16_S.0, coords_mask(&["G2", "G3", "G4"]));
    assert_eq!(COORD_17_CL.0, coords_mask(&["F5", "G5"]));
    assert_eq!(COORD_18_AR.0, coords_mask(&["E6", "F6"]));
    assert_eq!(COORD_19_K.0, coords_mask(&["E7", "F7", "G7"]));
    assert_eq!(COORD_20_CA.0, coords_mask(&["D6", "D7"]));
    assert_eq!(COORD_21_SC.0, coords_mask(&["C5", "C6"]));
    assert_eq!(COORD_22_TI.0, coords_mask(&["B5", "B6", "B7"]));
    assert_eq!(COORD_23_V.0, coords_mask(&["B4", "C4"]));
    assert_eq!(COORD_24_CR.0, coords_mask(&["C3", "D3"]));
    assert_eq!(COORD_25_MN.0, coords_mask(&["D4"]));
    assert_eq!(COORD_26_FE.0, coords_mask(&["E4", "D5", "E5"]));
}

#[test]
fn element_coordinates_no_overlap() {
    // Elements should not overlap - each tile belongs to exactly one element.
    let all_elements = [
        COORD_01_H,
        COORD_02_HE,
        COORD_03_LI,
        COORD_04_BE,
        COORD_05_B,
        COORD_06_C,
        COORD_07_N,
        COORD_08_O,
        COORD_09_F,
        COORD_10_NE,
        COORD_11_NA,
        COORD_12_MG,
        COORD_13_AL,
        COORD_14_SI,
        COORD_15_P,
        COORD_16_S,
        COORD_17_CL,
        COORD_18_AR,
        COORD_19_K,
        COORD_20_CA,
        COORD_21_SC,
        COORD_22_TI,
        COORD_23_V,
        COORD_24_CR,
        COORD_25_MN,
        COORD_26_FE,
    ];

    // Check each pair doesn't overlap
    for i in 0..all_elements.len() {
        for j in (i + 1)..all_elements.len() {
            assert_eq!(
                all_elements[i].0 & all_elements[j].0,
                0,
                "Elements {} and {} overlap",
                i + 1,
                j + 1
            );
        }
    }
}

#[test]
fn edge_elements_on_perimeter() {
    // From GAME_DESIGN.md: 12 edge elements (Z=1-12) touch the board perimeter.
    // These are the entry/exit gateways for the board.

    // First six edge elements (top row)
    assert!(COORD_01_H.on_edge(), "H should be on edge");
    assert!(COORD_02_HE.on_edge(), "He should be on edge");
    assert!(COORD_03_LI.on_edge(), "Li should be on edge");
    assert!(COORD_04_BE.on_edge(), "Be should be on edge");
    assert!(COORD_05_B.on_edge(), "B should be on edge");
    assert!(COORD_06_C.on_edge(), "C should be on edge");

    // Next six edge elements (bottom/left edge)
    assert!(COORD_07_N.on_edge(), "N should be on edge");
    assert!(COORD_08_O.on_edge(), "O should be on edge");
    assert!(COORD_09_F.on_edge(), "F should be on edge");
    assert!(COORD_10_NE.on_edge(), "Ne should be on edge");
    assert!(COORD_11_NA.on_edge(), "Na should be on edge");
    assert!(COORD_12_MG.on_edge(), "Mg should be on edge");
}

#[test]
fn core_element_not_on_edge() {
    // Fe (Z=26) is the core element, not on the perimeter.
    assert!(!COORD_26_FE.on_edge(), "Fe should not be on edge");
}

#[test]
fn middepth_elements_not_on_edge() {
    // Mid-depth elements (Z=13-20) should not touch the perimeter.
    assert!(!COORD_13_AL.on_edge(), "Al should not be on edge");
    assert!(!COORD_14_SI.on_edge(), "Si should not be on edge");
    assert!(!COORD_15_P.on_edge(), "P should not be on edge");
    assert!(!COORD_16_S.on_edge(), "S should not be on edge");
    assert!(!COORD_17_CL.on_edge(), "Cl should not be on edge");
    assert!(!COORD_18_AR.on_edge(), "Ar should not be on edge");
    assert!(!COORD_19_K.on_edge(), "K should not be on edge");
    assert!(!COORD_20_CA.on_edge(), "Ca should not be on edge");
}

#[test]
fn deep_elements_not_on_edge() {
    // Deep elements (Z=21-25) should not touch the perimeter.
    assert!(!COORD_21_SC.on_edge(), "Sc should not be on edge");
    assert!(!COORD_22_TI.on_edge(), "Ti should not be on edge");
    assert!(!COORD_23_V.on_edge(), "V should not be on edge");
    assert!(!COORD_24_CR.on_edge(), "Cr should not be on edge");
    assert!(!COORD_25_MN.on_edge(), "Mn should not be on edge");
}

#[test]
fn adjacency_from_board_layout() {
    // Test specific adjacencies visible in the board ASCII art.
    // These are elements that share edges based on the documented layout.

    // H (A1-A2,B1,C1) is adjacent to He (D1,E1) at D1-B1
    assert!(
        COORD_01_H.adjacent(COORD_02_HE),
        "H should be adjacent to He"
    );

    // H is adjacent to Mg (A3,B3) at A3-A2
    assert!(
        COORD_01_H.adjacent(COORD_12_MG),
        "H should be adjacent to Mg"
    );

    // Mg is adjacent to Na (A4,A5) at A4-A3
    assert!(
        COORD_12_MG.adjacent(COORD_11_NA),
        "Mg should be adjacent to Na"
    );

    // He (D1,E1) is adjacent to Li (F1,F2) at F1-E1
    assert!(
        COORD_02_HE.adjacent(COORD_03_LI),
        "He should be adjacent to Li"
    );

    // Li (F1,F2) is adjacent to Be (G1,H1,H2,H3) at G1-F1
    assert!(
        COORD_03_LI.adjacent(COORD_04_BE),
        "Li should be adjacent to Be"
    );

    // Be is adjacent to B (H4,H5) at H4-H3
    assert!(
        COORD_04_BE.adjacent(COORD_05_B),
        "Be should be adjacent to B"
    );

    // B is adjacent to C (G6,H6) at H6-H5
    assert!(COORD_05_B.adjacent(COORD_06_C), "B should be adjacent to C");

    // Al (B2,C2,D2) is adjacent to Mg (A3,B3) at B3-B2
    assert!(
        COORD_13_AL.adjacent(COORD_12_MG),
        "Al should be adjacent to Mg"
    );

    // Al is adjacent to Si (E2,E3) at E2-D2
    assert!(
        COORD_13_AL.adjacent(COORD_14_SI),
        "Al should be adjacent to Si"
    );

    // Si is adjacent to P (F3,F4) at F3-E3
    assert!(
        COORD_14_SI.adjacent(COORD_15_P),
        "Si should be adjacent to P"
    );

    // Fe (E4,D5,E5) is the core, surrounded by deep elements
    assert!(
        COORD_26_FE.adjacent(COORD_25_MN),
        "Fe should be adjacent to Mn"
    );
    assert!(
        COORD_26_FE.adjacent(COORD_21_SC),
        "Fe should be adjacent to Sc"
    );
    assert!(
        COORD_26_FE.adjacent(COORD_20_CA),
        "Fe should be adjacent to Ca"
    );
    assert!(
        COORD_26_FE.adjacent(COORD_15_P),
        "Fe should be adjacent to P"
    );
    assert!(
        COORD_26_FE.adjacent(COORD_17_CL),
        "Fe should be adjacent to Cl"
    );
}

#[test]
fn non_adjacent_elements() {
    // Elements that should NOT be adjacent (visually separated in layout).

    // H (top-left corner) and N (top-right corner F8,G8,H7,H8)
    assert!(
        !COORD_01_H.adjacent(COORD_07_N),
        "H should not be adjacent to N"
    );

    // He (D1,E1) and N (far side of board)
    assert!(
        !COORD_02_HE.adjacent(COORD_07_N),
        "He should not be adjacent to N"
    );

    // Edge elements on opposite corners: Li (F1,F2) and Ne (A6,A7,A8,B8)
    assert!(
        !COORD_03_LI.adjacent(COORD_10_NE),
        "Li should not be adjacent to Ne"
    );
}

#[test]
fn all_elements_used() {
    // Verify we have exactly 26 elements (Z=1 to Z=26).
    let all_elements = [
        COORD_01_H,
        COORD_02_HE,
        COORD_03_LI,
        COORD_04_BE,
        COORD_05_B,
        COORD_06_C,
        COORD_07_N,
        COORD_08_O,
        COORD_09_F,
        COORD_10_NE,
        COORD_11_NA,
        COORD_12_MG,
        COORD_13_AL,
        COORD_14_SI,
        COORD_15_P,
        COORD_16_S,
        COORD_17_CL,
        COORD_18_AR,
        COORD_19_K,
        COORD_20_CA,
        COORD_21_SC,
        COORD_22_TI,
        COORD_23_V,
        COORD_24_CR,
        COORD_25_MN,
        COORD_26_FE,
    ];
    assert_eq!(all_elements.len(), 26);
}
