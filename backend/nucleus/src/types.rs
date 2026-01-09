//! Fundamental types: fixed-point arithmetic, element identifiers, and board coordinates.

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

/// Sole in-game currency. Accumulates in wallets (liquid), charges (allocated), and element pots (shared).
pub type Gluon = u64;

/// Fixed-point (8 integer, 24 fractional bits): element saturation and player share in pots.
/// Range [0, 6]; conversion: `q824 = actual_value * 2^24`.
pub type Q824 = u32;

/// Fixed-point (16 integer, 48 fractional bits): pressure integral for path-independent history tracking.
/// Conversion: `q1648 = actual_value * 2^48`.
pub type Q1648 = u64;

/// 32-byte Solana public key: identifies authorities (signers) and mint accounts.
pub type AddressBytes = [u8; 32];

/// Encodes element atomic number (8 bits, high) and generation (56 bits, low).
/// Detects stale charge references when element resets increment generation.
/// Layout: `[generation_56bits | atomic_8bits]`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
pub struct ElementIndex(pub u64);

impl ElementIndex {
    const GEN_BITS: u32 = u64::BITS - u8::BITS;
    const GEN_MASK: u64 = u64::MAX >> u8::BITS;

    /// Extract atomic number (element position, 0..255).
    #[inline]
    pub fn atomic(self) -> u64 {
        self.0 >> Self::GEN_BITS
    }

    /// Extract generation counter.
    #[inline]
    pub fn generation(self) -> u64 {
        self.0 & Self::GEN_MASK
    }

    /// Increment generation to invalidate stale references after element reset.
    #[inline]
    pub fn advance_generation(&mut self) {
        let generation = (self.0 + 1) & Self::GEN_MASK;
        self.0 = (self.0 & !Self::GEN_MASK) | generation;
    }

    /// Mark charge as unbound (off-board).
    #[inline]
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    /// True if unbound (zero).
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn tiles(&self) -> u64 {
        self.0.count_ones() as u64
    }
}

/// Bitboard of element's squares on 8×8 board (row-major: rank 8 at top, file A at left).
/// Each bit represents one square. Used for adjacency checks and perimeter detection.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
pub struct Coordinates(pub u64);

impl Coordinates {
    // File and rank masks for 8×8 row-major layout.
    const FILE_A: u64 = 0x0101_0101_0101_0101; // left edge (file A)
    const FILE_H: u64 = 0x8080_8080_8080_8080; // right edge (file H)
    const RANK_1: u64 = 0x0000_0000_0000_00FF; // bottom edge (rank 1)
    const RANK_8: u64 = 0xFF00_0000_0000_0000; // top edge (rank 8)
    const NFILE_A: u64 = !Self::FILE_A;
    const NFILE_H: u64 = !Self::FILE_H;
    const PERIMETER: u64 = Self::FILE_A | Self::FILE_H | Self::RANK_1 | Self::RANK_8;

    /// True if shares orthogonal edge with another element (for movement validation).
    #[inline(always)]
    pub fn adjacent(self, other: Coordinates) -> bool {
        let neighbors = ((self.0 & Self::NFILE_H) << 1) // east neighbors
            | ((self.0 & Self::NFILE_A) >> 1) // west neighbors
            | (self.0 << 8) // north neighbors
            | (self.0 >> 8); // south neighbors
        (neighbors & other.0) != 0
    }

    /// True if touches board perimeter (entry/exit gateways).
    #[inline(always)]
    pub fn on_edge(self) -> bool {
        (self.0 & Self::PERIMETER) != 0
    }
}
