# Game Design

This document explains the conceptual design of TOKAMAK64: how mechanics create meaningful tension, and what emerges from simple rules.

## The Board

The game world is a static 8×8 grid (64 tiles).

### Elements

**Elements** are groups of connected tiles named after chemical elements and ordered by **atomic number (Z)**: there are 26 Elements on the board (Z=1 to Z=26), with element 0 serving as an off-board placeholder.

### Board Layout

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

#### Coordinates

| Z  | Element | Tiles                           |
|----|---------|---------------------------------|
| 1  | H       | A1, A2, B1, C1                  |
| 2  | He      | D1, E1                          |
| 3  | Li      | F1, F2                          |
| 4  | Be      | G1, H1, H2, H3                  |
| 5  | B       | H4, H5                          |
| 6  | C       | G6, H6                          |
| 7  | N       | F8, G8, H7, H8                  |
| 8  | O       | D8, E8                          |
| 9  | F       | C7, C8                          |
| 10 | Ne      | A6, A7, A8, B8                  |
| 11 | Na      | A4, A5                          |
| 12 | Mg      | A3, B3                          |
| 13 | Al      | B2, C2, D2                      |
| 14 | Si      | E2, E3                          |
| 15 | P       | F3, F4                          |
| 16 | S       | G2, G3, G4                      |
| 17 | Cl      | F5, G5                          |
| 18 | Ar      | E6, F6                          |
| 19 | K       | E7, F7, G7                      |
| 20 | Ca      | D6, D7                          |
| 21 | Sc      | C5, C6                          |
| 22 | Ti      | B5, B6, B7                      |
| 23 | V       | B4, C4                          |
| 24 | Cr      | C3, D3                          |
| 25 | Mn      | D4                              |
| 26 | Fe      | E4, D5, E5                                |

- **Edge Elements**: H, He, Li, Be, B, C, N, O, F, Ne, Na, Mg (Z=1-12) — touch the board perimeter
- **Mid-depth Elements**: Al, Si, P, S, Cl, Ar, K, Ca (Z=13-20) — progressively deeper
- **Deep Elements**: Sc, Ti, V, Cr, Mn (Z=21-25) — approaching the core
- **Core Element**: **Fe (Iron)** (Z=26) — maximum depth, terminal attractor for compressed value

Each Element has fixed depth (Z). Higher Z means:
- Larger commitment curves (absorbs more saturation before reset)
- Bigger typical pots (value concentrates toward center)
- Harder to escape (require multiple Rebind steps back to edge)

**Why this matters**: Depth creates strategic gradient. Outer Elements reset quickly with smaller pots; deep Elements are slower, heavier, more consequential.

### Movement

Two Elements are **adjacent** if they share a full tile edge (not just corners). Movement is direct jump via **Rebind**.

**Directional fee calculation**: Inward movement fees use destination saturation (before binding); outward fees use source saturation (before unbinding). Since deeper Elements have larger curves and thus typically lower saturation, inward moves tend to be cheaper. This is emergent, not hardcoded—outward can be cheaper when saturation is asymmetric.

## Charges

You don't exist on the board directly. You interact only through **Charges**—entities you control.

### Binding

- **Bound**: Charge occupies an Element (all its tiles simultaneously)
- **Unbound**: Charge is off-board
- Multiple Charges can occupy the same Element—positions aren't scarce, *timing* is

No spectator mode. Being on the board means being bound to exactly one Element. Every arrival and departure affects saturation.

## Commitment and Saturation

### Measured Commitment, Not Paid

When a Charge binds to an Element, its **commitment share** is measured instantly based on:
1. **Charge's Gluon balance**
2. **Current saturation** (when you arrive on the sigmoid curve)
3. **Element depth Z** (deeper curves are larger, need more Gluon for equivalent share)

Your Gluon is not locked or transferred. It's an input to measurement, not a payment.

### The Sigmoid Curve

Commitment follows a **sigmoid curve**:
- **Early** (low saturation): Large share per Gluon (~20× efficiency)
- **Mid** (near inflection): Diminishing returns (~2×)
- **Late** (near threshold): Marginal share, mostly pushes toward reset

Sigmoid rewards patience without locking out late triggers.

### Saturation

Each Element tracks **saturation**—the sum of commitment shares of currently bound Charges. Entry increases it, exit decreases it. Fully reversible—crowds can form and disperse.

When saturation crosses the threshold (1.0 normalized), the Element **resets**. Typically the triggering Rebind/Bind and Overload are bundled atomically.

## Value Flow

### The Pot

Each Element maintains a single visible **pot**—shared value that grows from movement fees, compression fees, and voluntary contributions (Vent). One shared pot ensures everyone cares about the same object.

### Distribution at Reset

When an Element resets:
1. Pot divided proportionally by commitment share
2. All Charges except trigger are unbound (free repositioning)
3. Trigger remains bound (first-mover advantage in new cycle)
4. Pot and saturation cleared

**Critical**: Only Charges bound at the exact reset instant receive rewards. No credit for past presence.

### Speed Tax

A multiplier (up to 128×) applies based on time since last action. Decays to 1× after 1024 slots (~51 seconds on L2). Eliminates speed as an advantage—the game rewards timing, not reflexes.

## Compression

Compression relocates pots to adjacent Elements with higher Z:
- Fee (scales with pot size and depth difference) is added to the pot being moved
- Merged pot at destination is strictly larger
- Requires dst.index > src.index (can be sideways or skip depths if adjacent)

Compression solves stagnant pots—rescues value from dead regions, pushes value into hotspots, escalates stakes. But compression grows pots and attracts competition—it's escalation, not safety.

**Iron (Fe, Z=26)** is the terminal sink. Compressed pots cannot leave; Iron resets are the biggest events.

## Voluntary Contributions (Vent)

A bound Charge can irreversibly add Gluon to its Element's pot. The pot increases but commitment share and saturation don't change. Contributions are **signals**, not investments—they attract attention (allies and predators alike).

## The Closed Loop

Actions cost Gluon → costs become pots → pots attract attention → attention creates saturation → saturation triggers reset → reset redistributes and ejects → Charges reposition.

All costs become shared value (never burned). Every action creates opportunity for future participants.

## Core Tension

The question is not "should I stay or leave?" but **"where and when will the next profitable reset occur?"**

- Early arrival → large shares
- Late arrival → can trigger at favorable moment
- Compression → escalates stakes

Gameplay is positioning/timing with multiple valid strategies.

## Roadmap: Quantum Pocket

*Not yet implemented.*

When Fe resets, external yield injects into the **Quantum Pocket**—a global pool. Yield unlocks sequentially from edge inward (H, He, Li...), distributed at each depth's reset. Keeps edge Elements relevant in late-game.

## Design Principles

1. **Transparency** — All state visible; strategy is reading incentives
2. **Waiting is free** — Costs only on voluntary actions
3. **Closed economics** — All friction becomes shared value
4. **Directional asymmetry** — Fee calculation creates typical inward bias; value drifts center
5. **Sigmoid patience** — Early commitment rewarded, late triggers possible
6. **Clean resets** — Free repositioning; game cycles forever
7. **Emergent complexity** — Simple rules produce diverse strategies

