use bytemuck::AnyBitPattern;

pub type Gluon = u64;
pub type Q1616 = i32;
pub type Q1648 = i64;

#[repr(transparent)]
#[derive(AnyBitPattern, Clone, Copy)]
pub struct ElementIndex(u64);
#[repr(transparent)]
#[derive(AnyBitPattern, Clone, Copy)]
pub struct Coordinates(u64);

pub type AddressBytes = [u8; 32];

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
