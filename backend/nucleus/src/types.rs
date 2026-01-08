use bytemuck::{Pod, Zeroable};

/// Gluon is the unit of value that circulates through the game.
/// Every quantity tracked in TOKAMAK64 is measured in Gluon.
pub type Gluon = u64;

/// Q824 is a fixed-point number with 24 fractional bits (unsigned).
/// Used for curve position and precise share calculations in pressure mechanics.
/// Range: [0, 12] in actual values, represented as u32 in Q8.24.
pub type Q824 = u32;

/// Q1648 is a fixed-point number with 48 fractional bits (unsigned).
/// Used for curve state (s parameter) to accumulate precise changes.
/// Encoded as u64 in Q16.48.
pub type Q1648 = u64;

/// Z represents unsigned integer quantities (deprecated alias).
pub type Z = u64;

/// AddressBytes is a 32-byte public key (Solana address).
pub type AddressBytes = [u8; 32];

/// ElementIndex encodes both the element's atomic number and its generation.
///
/// Layout (u64):
/// - High 8 bits: atomic number (0..255) — the static identity of the element
/// - Low 56 bits: generation counter — increments each time the element resets
///
/// Ref: TOKAMAK64 Part 6 (Element Overload & Reset)
#[repr(transparent)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ElementIndex(u64);

impl ElementIndex {
    const GEN_BITS: u32 = u64::BITS - u8::BITS;
    const GEN_MASK: u64 = u64::MAX >> u8::BITS;

    /// Returns the atomic number of this element (0..255).
    #[inline]
    pub fn atomic_number(self) -> u64 {
        self.0 >> Self::GEN_BITS
    }

    /// Returns the generation counter of this element.
    #[inline]
    pub fn generation(self) -> u64 {
        self.0 & Self::GEN_MASK
    }

    /// Increments the generation counter (wraps within 56 bits).
    #[inline]
    pub fn nextgen(&mut self) {
        let generation = (self.0 + 1) & Self::GEN_MASK;
        self.0 = (self.0 & !Self::GEN_MASK) | generation;
    }

    /// Clears the element index to zero (used on player exit).
    #[inline]
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    #[inline]
    pub fn zero(&self) -> bool {
        self.0 == 0
    }
}

/// Coordinates encode the spatial extent of an element as a bitboard.
///
/// A bitmask over a 64-square grid (8×8 board), where each bit represents
/// one square. Multiple bits indicate a multi-square element.
///
/// Ref: TOKAMAK64 Part 0.2 (Squares Are Grouped)
#[repr(transparent)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct Coordinates(u64);

impl Coordinates {
    // File masks for edge detection in 8×8 bitboard layout (row-major).
    // File A (left edge): bits 0, 8, 16, 24, 32, 40, 48, 56
    // File H (right edge): bits 7, 15, 23, 31, 39, 47, 55, 63
    const FILE_A: u64 = 0x0101_0101_0101_0101;
    const FILE_H: u64 = 0x8080_8080_8080_8080;
    const RANK_1: u64 = 0x0000_0000_0000_00FF;
    const RANK_8: u64 = 0xFF00_0000_0000_0000;
    const NFILE_A: u64 = !Self::FILE_A;
    const NFILE_H: u64 = !Self::FILE_H;
    const PERIMETER: u64 = Self::FILE_A | Self::FILE_H | Self::RANK_1 | Self::RANK_8;

    /// Returns true if self shares an edge (N/S/E/W) with other.
    ///
    /// Elements are considered neighbors if they share at least one full edge
    /// (not just a corner). This determines valid movement paths on the board.
    ///
    /// Ref: TOKAMAK64 Part 0.4 (Movement)
    #[inline(always)]
    pub fn adjacent(self, other: Coordinates) -> bool {
        // Compute all edge-adjacent squares to self (no diagonal/corner).
        let neighbors = ((self.0 & Self::NFILE_H) << 1) // east
            | ((self.0 & Self::NFILE_A) >> 1) // west
            | (self.0 << 8) // north
            | (self.0 >> 8); // south
        (neighbors & other.0) != 0
    }

    #[inline(always)]
    pub fn peripheral(&self) -> bool {
        (self.0 & Self::PERIMETER) != 0
    }
}
