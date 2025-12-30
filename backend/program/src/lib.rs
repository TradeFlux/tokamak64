struct Board {
    /// Total value locked on board (in GLUON), this doesn't include (q)pots
    tvl: u64,
    /// Total player count, bound on board
    tpc: u32,
    /// Total volume of GLUON accumulated in quantum pocket
    qpocket: u64,
}

struct Element {
    /// Atomic number of the element
    z: u8,
    /// Generation - number of resets the element underwent
    generation: u32,
    /// Number of wallets bound to the element at the moment
    bound: u32,
    /// Amount of GLUON bound to the element at the momment
    saturation: u64,
    /// Total capacity (in GLUON) that the element can absorb before reset
    capacity: u64,
    /// Ordinary (movable/compressable) rewards pot volume (in GLUON)
    pot: u64,
    /// Quantum (uncompressable) rewards pot volume (in GLUON)
    qpot: u64,
    /// Bitmasked indices of tiles (out of 64) which are part of this element
    coordinates: u64,
}

struct Player {
    /// Atomic number of the element, the player is bound to
    z: u8,
    /// Total volume of GLUON, the player has
    balance: u64,
    /// Total share of the rewards the player is currently entitled to (determined by the curve)
    entitlement: u32,
    /// Timestamp (in some discrete blockchain measure) when the player performed the last action
    ts: u64,
}
