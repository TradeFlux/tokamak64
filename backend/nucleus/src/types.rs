use bytemuck::{Pod, Zeroable};

/// Gluon: atomic unit of value circulating in the game.
pub type Gluon = u64;

/// Q8.24 fixed-point: 8 integer bits, 24 fractional. Range [0, 12].
/// Used for curve position and pressure-driven share calculations.
pub type Q824 = u32;

/// Q16.48 fixed-point: 16 integer bits, 48 fractional.
/// Used for curve state to accumulate precise pressure changes over time.
pub type Q1648 = u64;

/// AddressBytes: 32-byte public key (Solana address).
pub type AddressBytes = [u8; 32];

/// ElementIndex encodes atomic number (high 8 bits) and generation (low 56 bits).
/// Atomic number identifies the element's static position; generation increments after reset.
/// Layout: `[gen_56bits | atomic_8bits]`
#[repr(transparent)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ElementIndex(pub u64);

impl ElementIndex {
    const GEN_BITS: u32 = u64::BITS - u8::BITS;
    const GEN_MASK: u64 = u64::MAX >> u8::BITS;

    /// Atomic number of this element (0..255).
    #[inline]
    pub fn atomic_number(self) -> u64 {
        self.0 >> Self::GEN_BITS
    }

    /// Generation counter (increments on element reset).
    #[inline]
    pub fn generation(self) -> u64 {
        self.0 & Self::GEN_MASK
    }

    /// Increment generation (wraps within 56 bits).
    #[inline]
    pub fn advance_generation(&mut self) {
        let generation = (self.0 + 1) & Self::GEN_MASK;
        self.0 = (self.0 & !Self::GEN_MASK) | generation;
    }

    /// Reset to zero (used when player exits).
    #[inline]
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    /// True if zero.
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

/// Coordinates: 64-bit bitboard representing spatial extent on 8×8 board.
/// Each bit = one square. Multiple bits = multi-square element.
#[repr(transparent)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct Coordinates(pub u64);

impl Coordinates {
    // File and rank masks for 8×8 row-major layout.
    const FILE_A: u64 = 0x0101_0101_0101_0101; // left edge
    const FILE_H: u64 = 0x8080_8080_8080_8080; // right edge
    const RANK_1: u64 = 0x0000_0000_0000_00FF; // bottom edge
    const RANK_8: u64 = 0xFF00_0000_0000_0000; // top edge
    const NFILE_A: u64 = !Self::FILE_A;
    const NFILE_H: u64 = !Self::FILE_H;
    const PERIMETER: u64 = Self::FILE_A | Self::FILE_H | Self::RANK_1 | Self::RANK_8;

    /// True if self shares an orthogonal edge (N/S/E/W, not diagonal) with other.
    #[inline(always)]
    pub fn adjacent(self, other: Coordinates) -> bool {
        let neighbors = ((self.0 & Self::NFILE_H) << 1) // east
            | ((self.0 & Self::NFILE_A) >> 1) // west
            | (self.0 << 8) // north
            | (self.0 >> 8); // south
        (neighbors & other.0) != 0
    }

    /// True if this element touches the board perimeter.
    #[inline(always)]
    pub fn is_peripheral(self) -> bool {
        (self.0 & Self::PERIMETER) != 0
    }
}
