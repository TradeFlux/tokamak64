//! Game parameters and constants: element limits, fee baselines, and curve saturation bounds.

use crate::types::Coordinates;

/// Maximum atomic number (0-indexed, so 27 distinct elements).
pub const MAX_ATOMIC_NUMBER: u64 = 26;

pub const SUM_ATOMIC_NUMBERS: u64 = MAX_ATOMIC_NUMBER * (MAX_ATOMIC_NUMBER + 1) / 2;

/// Maximum curve position (Q8.24, range [0, 6]).
pub const MAX_SATURATION: u32 = curve::consts::LUT_X_MAX;

/// Minimum fee (in Gluon) to prevent dust.
pub const MIN_FEE: u64 = 100_000;

/// Token decimals (Gluon precision, same as popular stable coins).
pub const DECIMALS: u8 = 6;

/// Maximum speed multiplier (applied based on action timing).
pub const MAX_SPEED_MULTIPLIER: u64 = 127;

/// Maximum elapsed time window for speed tax (slots).
pub const MAX_DELTA_TIMESTAMP: u64 = 1024;

/// Element coordinate bitmasks (8×8 board, row-major).
/// Each bit represents one tile: A1=bit0, B1=bit1, ..., H8=bit63.
pub const COORD_01_H: Coordinates = Coordinates(0x0000000000000107); // A1, A2, B1, C1
pub const COORD_02_HE: Coordinates = Coordinates(0x0000000000000018); // D1, E1
pub const COORD_03_LI: Coordinates = Coordinates(0x0000000000002020); // F1, F2
pub const COORD_04_BE: Coordinates = Coordinates(0x00000000008080C0); // G1, H1, H2, H3
pub const COORD_05_B: Coordinates = Coordinates(0x0000008080000000); // H4, H5
pub const COORD_06_C: Coordinates = Coordinates(0x0000C00000000000); // G6, H6
pub const COORD_07_N: Coordinates = Coordinates(0xE080000000000000); // F8, G8, H7, H8
pub const COORD_08_O: Coordinates = Coordinates(0x1800000000000000); // D8, E8
pub const COORD_09_F: Coordinates = Coordinates(0x0404000000000000); // C7, C8
pub const COORD_10_NE: Coordinates = Coordinates(0x0301010000000000); // A6, A7, A8, B8
pub const COORD_11_NA: Coordinates = Coordinates(0x0000000101000000); // A4, A5
pub const COORD_12_MG: Coordinates = Coordinates(0x0000000000030000); // A3, B3
pub const COORD_13_AL: Coordinates = Coordinates(0x0000000000000E00); // B2, C2, D2
pub const COORD_14_SI: Coordinates = Coordinates(0x0000000000101000); // E2, E3
pub const COORD_15_P: Coordinates = Coordinates(0x0000000020200000); // F3, F4
pub const COORD_16_S: Coordinates = Coordinates(0x0000000040404000); // G2, G3, G4
pub const COORD_17_CL: Coordinates = Coordinates(0x0000006000000000); // F5, G5
pub const COORD_18_AR: Coordinates = Coordinates(0x0000300000000000); // E6, F6
pub const COORD_19_K: Coordinates = Coordinates(0x0070000000000000); // E7, F7, G7
pub const COORD_20_CA: Coordinates = Coordinates(0x0008080000000000); // D6, D7
pub const COORD_21_SC: Coordinates = Coordinates(0x0000040400000000); // C5, C6
pub const COORD_22_TI: Coordinates = Coordinates(0x0002020200000000); // B5, B6, B7
pub const COORD_23_V: Coordinates = Coordinates(0x0000000006000000); // B4, C4
pub const COORD_24_CR: Coordinates = Coordinates(0x00000000000C0000); // C3, D3
pub const COORD_25_MN: Coordinates = Coordinates(0x0000000008000000); // D4
pub const COORD_26_FE: Coordinates = Coordinates(0x0000001810000000); // E4, D5, E5
