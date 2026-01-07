type Gluon = u64;
type Q1616 = u32;
type Q1648 = u64;

use bytemuck::AnyBitPattern;

#[repr(transparent)]
#[derive(AnyBitPattern, Clone, Copy)]
pub struct ElementIndex(u64);
#[repr(transparent)]
#[derive(AnyBitPattern, Clone, Copy)]
pub struct Coordinates(u64);

impl Coordinates {
    // File masks for an 8x8 bitboard where bit 0 is the leftmost of the bottom row
    // (row-major indexing: 0..7 first row, 8..15 next, etc.)
    const NFILE_A: u64 = !0x0101_0101_0101_0101;
    const NFILE_H: u64 = !0x8080_8080_8080_8080;

    /// Returns true if `self` shares at least one edge (N/S/E/W) with `other`.
    /// Groups are guaranteed disjoint (no overlapping bits), but that’s not required for correctness.
    #[inline(always)]
    pub fn adjacent(self, other: Coordinates) -> bool {
        // Neighbors of `self` in 4 directions (no wrap across left/right edges).
        let neighbors = ((self.0 & Self::NFILE_H) << 1) // east
            | ((self.0 & Self::NFILE_A) >> 1) // west
            | (self.0 << 8) // north
            | (self.0 >> 8); // south

        (neighbors & other.0) != 0
    }
}

#[repr(C)]
#[derive(AnyBitPattern, Clone, Copy)]
pub struct Curve {
    capacity: Gluon,
    s: Q1648,
    x: Q1616,
}

#[repr(C)]
#[derive(AnyBitPattern, Clone, Copy)]
struct Board {
    /// Total value locked on board (in GLUON), including pots
    tvl: u64,
    /// Total volume of GLUON accumulated in quantum pocket
    quantum_pocket: Gluon,
    /// Total charge count, bound on board
    charges: u32,
    /// Index of the quantum pot that has been unlocked so far (corresponds to element)
    quantum_index: u8,
}

#[repr(C)]
#[derive(AnyBitPattern, Clone, Copy)]
struct Element {
    index: ElementIndex,
    /// Ordinary (movable/compressable) rewards pot volume (in GLUON)
    pot: Gluon,
    /// Bitmasked indices of tiles (out of 64) which are part of this element
    coordinates: Coordinates,
    curve: Curve,
}

#[repr(C)]
#[derive(AnyBitPattern, Clone, Copy)]
struct Tombstone {
    index: ElementIndex,
    pot: u64,
    charges: u32,
}

#[repr(C)]
#[derive(AnyBitPattern, Clone, Copy)]
struct Charge {
    index: ElementIndex,
    /// Total volume of GLUON, the player has
    balance: Gluon,
    /// Timestamp (in some discrete blockchain measure) when the player performed the last action
    timestamps: u64,
    /// Total share of the rewards the player is currently entitled to (determined by the curve)
    share: Q1616,
    /// Pubkey of the ownning wallet
    authority: [u8; 32],
}

impl ElementIndex {
    const GEN_BITS: u32 = u64::BITS - u8::BITS;
    const GEN_MASK: u64 = u64::MAX >> u8::BITS;

    pub fn atomic_number(self) -> u64 {
        self.0 >> Self::GEN_BITS
    }

    pub fn generation(self) -> u64 {
        self.0 & Self::GEN_MASK
    }

    pub fn increment_generation(&mut self) {
        let generation = (self.0 + 1) & Self::GEN_MASK;
        self.0 = (self.0 & !Self::GEN_MASK) | generation;
    }
}
