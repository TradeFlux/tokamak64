# TOKAMAK64

A strategy game where players compete to position themselves in the right locations at the right time to capture value from element resets.

## The Core Loop

TOKAMAK64 is played on a fixed 8×8 board divided into 26 **Elements** (regions) named after chemical elements—Hydrogen at the outer edge, progressively heavier elements inward, Iron at the center.

You control **Charges** that bind to Elements. Multiple Charges can occupy the same Element, building **saturation**. When saturation exceeds the threshold, the Element **resets**—distributing its accumulated pot to bound Charges.

Movement has directional bias: moving inward (toward Iron) is cheaper; moving outward costs more. This creates natural value flow toward the center, inspired by how fusion reactors have inward magnetic confinement. All costs feed back into the system as shared value — nothing is destroyed.

**The fundamental question**: *"Where and when will the next profitable reset occur?"*

## The Game in 30 Seconds

- Fixed 8×8 board divided into 26 **Elements** (H, He, Li... Fe)
- You control **Charges** that occupy entire Elements
- Multiple Charges per Element accumulate **saturation**
- When saturation exceeds threshold, Element **resets**: pot distributed, Charges ejected
- Moving inward is cheap; moving outward is expensive
- All costs feed back as shared value

**Core tension**: Constantly reposition between building hotspots, balancing early arrival (strong shares) against timing risk.

## Quick Start

1. Fund Solana wallet with USDT/USDC
2. **Infuse** — convert stablecoins to Gluon (in-game currency, 1:1)
3. **Charge** — allocate Gluon to create a Charge
4. **Inject** — place Charge on board (edge Elements only: H, He, Li, Be, B, C)
5. **Rebind** — move between adjacent Elements
6. **Overload** — trigger Element reset (typically bundled atomically with Rebind/Inject that pushes saturation over threshold)
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
| **Reset** | When saturation exceeds threshold: pot distributed, Charges ejected (free exit). |
| **Depth (Z)** | Atomic number (1–26). Higher Z = deeper, bigger pots, harder to escape. |
| **Commitment Share** | Measured when binding. Sigmoid curve: early binding grants larger shares. |
| **Compression** | Carry pot to adjacent Element with higher Z. Can be sideways or skip depths. Fee added to pot. Cost varies by depth difference. |

## Why Play?

**Profit-chasing via positioning**: Early arrival grants large commitment shares; late arrival gets diminishing returns but can trigger resets. Compression escalates stakes by moving value to higher Z Elements (strategic routing based on cost/topology). Reading saturation buildup and timing repositioning is the core skill.

**Emergent roles without coordination**:
- **Sentinels** wait patiently through buildup
- **Catalysts** trigger profitable resets with precision
- **Compressors** redirect value flows to reshape hotspots
- **Arsonists** escalate chaos with aggressive contributions
- **Scavengers** harvest shallow quick cycles
- **Divers** time deep Iron resets for massive yields

The game cycles forever—no shutdown, only evolving states shaped by player timing.

## Documentation

- **[Game Design](docs/GAME_DESIGN.md)** — Board topology, commitment mechanics, value flow, design motivation
- **[Mechanics](docs/MECHANICS.md)** — All 13 instructions, fee routing, invariants, technical details
- **[Player Guide](docs/PLAYER_GUIDE.md)** — Mental models, archetypes, profit-chasing strategies

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
