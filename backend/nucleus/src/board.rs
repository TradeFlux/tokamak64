use bytemuck::{Pod, Zeroable};

use crate::types::{Coordinates, ElementIndex, Gluon, Q1648, Q824};

/// Curve encodes the state of an element's bonding curve.
///
/// The curve determines:
/// - how much the next entry costs (capacity and slope),
/// - how much each player's share is worth (position),
/// - and how the element behaves under pressure.
///
/// Field order optimized for packing:
/// - Gluon (u64, 8 bytes, align 8)
/// - Q1648 (i64, 8 bytes, align 8)
/// - Q824 (i32, 4 bytes, align 4)
/// - i32 padding (4 bytes) to reach 24-byte alignment
///
/// Ref: TOKAMAK64 Part 4 (Curves) and Part 5 (Pressure & Sharing)
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Curve {
    /// Total Gluon capacity of the curve. Maximum TVL that can bind here.
    pub capacity: Gluon,
    /// Accumulated state change (Q1648). Tracks integral of pressure over time.
    pub state: Q1648,
    pub volume: Gluon,
    /// Current position on curve (Q824). Higher position means higher entry cost next.
    pub position: Q824,
    _padding: u32,
}

/// Element represents a single group on the board.
///
/// An element is a contiguous region of the 8×8 board where players can gather.
/// When multiple players gather in the same element, they create pressure that
/// can eventually cause the element to reset (overload).
///
/// Ref: TOKAMAK64 Part 0.2 (Squares Are Grouped) and Part 5 (Pressure & Sharing)
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Element {
    /// Shared pot of Gluon in this element (rewards, cost contributions).
    pub pot: Gluon,
    /// Which element this is (atomic number + generation). Static index until reset.
    pub index: ElementIndex,
    /// Bonding curve state for this element.
    pub curve: Curve,
    /// Bitboard of which squares this element occupies on the 8×8 board.
    pub coordinates: Coordinates,
}

/// Board tracks global state across all elements.
///
/// The board is a singleton account that records:
/// - total value locked (for reference),
/// - quantum pocket accumulation (for rare unlocks),
/// - charge count (for pressure calculations).
///
/// Field order optimized for packing:
/// - u64 (8 bytes): tvl
/// - u64 (8 bytes): quantum_pocket
/// - u32 (4 bytes): charges
/// - u8 (1 byte): quantum_index
/// - [u8; 3] padding (3 bytes) to reach alignment
///
/// Ref: TOKAMAK64 Part 6 (Overload & Reset) and Part 9+ (Quantum Mechanics)
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Board {
    /// Total Gluon currently bound to charges (TVL). For reference.
    pub tvl: Gluon,
    /// Accumulated Gluon in the quantum pocket (unlocked at special depths).
    pub quantum_pocket: Gluon,
    /// Total count of active charges. Used to calculate global pressure.
    pub charges: u32,
    /// Quantum unlock index. How many elements have been fully reset.
    pub quantum_index: u8,
    _padding: [u8; 3],
}

/// Tombstone marks an element that has overloaded and is being wound down.
///
/// When an element overloads, it transitions to a Tombstone state. Players
/// can no longer enter but can still claim their share of the final pot.
///
/// Field order optimized for packing:
/// - u64 (8 bytes): pot
/// - u64 (8 bytes): index
/// - u32 (4 bytes): shares
/// - u32 padding (4 bytes) for alignment
///
/// Ref: TOKAMAK64 Part 6 (Overload & Reset)
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Tombstone {
    /// Final pot value to be distributed to shareholders.
    pub pot: Gluon,
    /// Which element this was (for reference).
    pub index: ElementIndex,
}
