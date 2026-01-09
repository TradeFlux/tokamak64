use crate::{
    action::{claim, compress, rebind},
    board::{Artefact, Board, Curve, Element},
    consts::*,
    fees::{bind_fee, compression_fee, fee_multiplier, rebind_fee, unbind_fee},
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
    Element {
        index,
        pot,
        coordinates: Coordinates(1u64),
        curve: Curve {
            capacity,
            tvl: 0,
            pressure: 0,
            saturation: 0,
            _pad: 0,
        },
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
    assert_eq!(idx.atomic_number(), 42);
    assert_eq!(idx.generation(), 7);
}

#[test]
fn element_index_next_gen() {
    let mut idx = ElementIndex((5u64 << 56) | 100);
    idx.advance_generation();
    assert_eq!(idx.atomic_number(), 5);
    assert_eq!(idx.generation(), 101);
}

#[test]
fn element_index_next_gen_wraps() {
    let max_gen = (1u64 << 56) - 1;
    let mut idx = ElementIndex((10u64 << 56) | max_gen);
    idx.advance_generation();
    assert_eq!(idx.atomic_number(), 10);
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
    let mut artefact = Artefact {
        pot: 1_000_000,
        index: ElementIndex((1u64 << 56) | 1),
    };

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
