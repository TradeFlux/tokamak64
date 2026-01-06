use crate::types::{Coordinates, ElementIndex, Gluon, Q1616, Q1648};

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "zerocopy", derive(bytemuck::AnyBitPattern))]
pub struct Curve {
    pub capacity: Gluon,
    pub s: Q1648,
    pub x: Q1616,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "zerocopy", derive(bytemuck::AnyBitPattern))]
pub struct Element {
    pub index: ElementIndex,
    /// Ordinary (movable/compressable) rewards pot volume (in GLUON)
    pub pot: Gluon,
    /// Bitmasked indices of tiles (out of 64) which are part of this element
    pub coordinates: Coordinates,
    pub curve: Curve,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "zerocopy", derive(bytemuck::AnyBitPattern))]
pub struct Board {
    /// Total value locked on board (in GLUON), this doesn't include (q)pots
    pub tvl: u64,
    /// Total volume of GLUON accumulated in quantum pocket
    pub quantum_pocket: Gluon,
    /// Total charge count, bound on board
    pub charges: u32,
    /// Index of the quantum pot that has been unlocked so far (corresponds to element)
    pub quantum_index: u8,
}

/// An element has been overloaded and is ready to be claimed by shareholders
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "zerocopy", derive(bytemuck::AnyBitPattern))]
pub struct Tombstone {
    pub index: ElementIndex,
    pub pot: Gluon,
    pub charges: u32,
}
