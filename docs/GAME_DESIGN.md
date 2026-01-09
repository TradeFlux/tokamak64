# Game Design

This document explains the conceptual design of TOKAMAK64: how mechanics create meaningful tension, and what emerges from simple rules.

## The Board

The game world is a static 8×8 grid (64 tiles).

### Elements

**Elements** are groups of connected tiles named after chemical elements and ordered by **atomic number (Z)**: there are 26 Elements on the board (Z=1 to Z=26), with element 0 serving as an off-board placeholder.

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

- **Edge Elements**: H (Hydrogen), He (Helium), Li (Lithium), Be (Beryllium), B (Boron), C (Carbon) — touch the board perimeter
- **Mid-depth Elements**: N, O, F, Ne, Na, Mg, Al, Si, P, S, Cl, Ar, K, Ca — progressively deeper
- **Deep Elements**: Sc, Ti, V, Cr, Mn — approaching the core
- **Core Element**: **Fe (Iron)** — maximum depth (Z=26), terminal attractor for compressed value

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

