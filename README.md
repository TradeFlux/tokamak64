# TOKAMAK64

A strategy game where players compete to position themselves in the right locations at the right time to capture value from element resets.

## The Core Loop

TOKAMAK64 is played on a fixed 8×8 board divided into 26 **Elements** (regions) named after chemical elements—Hydrogen at the outer edge, progressively heavier elements inward, Iron at the center.

You control **Charges** that bind to Elements. Multiple Charges can occupy the same Element, building **saturation**. When saturation exceeds the threshold, the Element **resets**—distributing its accumulated pot to bound Charges.

Movement fees depend on saturation: inward fees use destination saturation, outward fees use source saturation. Since deeper Elements have larger curves (lower typical saturation), inward moves tend to be cheaper—creating natural value flow toward the center. All costs feed back into the system as shared value.

**The fundamental question**: *"Where and when will the next profitable reset occur?"*

## The Game in 30 Seconds

- Fixed 8×8 board divided into 26 **Elements** (H, He, Li... Fe)
- You control **Charges** that occupy entire Elements
- Multiple Charges per Element accumulate **saturation**
- When saturation exceeds threshold, Element **resets**: pot distributed, Charges unbound
- Inward moves typically cheaper (fee asymmetry from curve capacity)
- All costs feed back as shared value

**Core tension**: Constantly reposition between building hotspots, balancing early arrival (strong shares) against timing risk.

## Quick Start

1. Fund Solana wallet with USDT/USDC
2. **Infuse** — convert stablecoins to Gluon (in-game currency, 1:1)
3. **Charge** — allocate Gluon to create a Charge
4. **Bind** — place Charge on board (edge Elements only: H, He, Li, Be, B, C)
5. **Rebind** — move between adjacent Elements
6. **Overload** — trigger Element reset (typically bundled atomically with Rebind/Bind that pushes saturation over threshold)
7. **Claim** — collect reward share after reset
8. **Discharge** — merge Charge back to wallet
9. **Extract** — convert Gluon back to stablecoins

## Key Concepts

| Concept | Description |
|---------|-------------|
| **Element** | A region (group of tiles). Named after chemical elements (H at edge, Fe at center). |
| **Charge** | Your on-board entity. Bound when on board, unbound when off. Multiple per player. |
| **Gluon** | In-game currency. Always yours—never locked or staked. |
| **Saturation** | Sum of commitment shares. Rises on entry, falls on exit. |
| **Reset** | When saturation exceeds threshold: pot distributed, Charges unbound (free exit). |
| **Depth (Z)** | Atomic number (1–26). Higher Z = deeper, bigger pots, harder to escape. |
| **Commitment Share** | Measured when binding. Sigmoid curve: early binding grants larger shares. |
| **Compression** | Carry pot to adjacent Element with higher Z. Can be sideways or skip depths. Fee added to pot. Cost varies by depth difference. |

## Why Play?

**Profit-chasing via positioning**: Early arrival grants large commitment shares; late arrival gets diminishing returns but can trigger resets. Compression escalates stakes by moving value deeper. The core skill is reading saturation buildup and timing repositioning.

**No dominant strategy**: Entry timing, depth, and activity level are independent tradeoffs. The game cycles forever—equilibrium emerges from player decisions, not prescribed roles.

## Board Layout

8×8 grid (64 tiles) with 26 Elements:

```
    1    2    3    4    5    6    7    8
  ┌──────────────┬─────────┬────┬─────────┐
A │  H1          │  He2    │ Li3│      Be4│
  │    ┌─────────┴────┬────┤    ├────┐    │
B │    │       Al13   │Si14│    │    │    │
  ├────┴────┬─────────┤    ├────┤    │    │
C │  Mg12   │  Cr24   │    │ P15│ S16│    │
  ├────┬────┴────┬────┼────┤    │    ├────┤
D │    │  V23    │Mn25│    │    │    │ B5 │
  │    ├────┬────┼────┘    ├────┴────┤    │
E │Na11│    │    │     Fe26│     Cl17│    │
  ├────│    │    ├────┬────┴────┬────┴────┤
F │    │Ti22│Sc21│    │     Ar18│      C6 │
  │    │    ├────┤    ├─────────┴────┬────┤
G │    │    │    │Ca20│        K19   │    │
  │    └────┤    ├────┴────┬─────────┘    │
H │  Ne10   │ F9 │      O8 │           N7 │
  └─────────┴────┴─────────┴──────────────┘
```

## Documentation

- **[Game Design](docs/GAME_DESIGN.md)** — Board topology, commitment mechanics, value flow, design motivation
- **[Mechanics](docs/MECHANICS.md)** — All 13 instructions, fee routing, invariants, technical details
- **[Player Guide](docs/PLAYER_GUIDE.md)** — Operations, fee strategy, strategic tradeoffs

## Building

```bash
cargo build --workspace           # Build all crates
cargo test --workspace            # Run tests
cargo build-sbf -p program        # Solana BPF target
```

See [backend/README.md](backend/README.md) for implementation details.

## Architecture

```
backend/
├── curve/     # Sigmoid commitment curve (LUT)
├── nucleus/   # Core types and logic (blockchain-agnostic)
└── program/   # Solana on-chain program
```

**Program ID:** `DuJrE9ZB4TqcMByw9g4CiDQdNQosPQCQw2ECWGfLiyi`

## Why On-Chain?

- **Complete transparency** — All state visible: saturation, pots, positions
- **No hidden information** — Every threshold, every share is public
- **Fairness** — Rules cannot be changed; enforcement is deterministic
- **Permanence** — All actions immutably recorded
