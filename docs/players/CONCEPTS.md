# Core Concepts

This document explains the mental model of TOKAMAK64—what things are, how they relate, and why they matter.

## The Board

The game takes place on a fixed 8×8 grid divided into 26 **Elements**. Each Element is a contiguous region of tiles named after chemical elements, ordered by atomic number (Z):

- **Z=1 (Hydrogen)** at the outer edge
- **Z=26 (Iron)** at the center

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

### Depth

An Element's atomic number (Z) represents its **depth**:

| Depth | Elements | Characteristics |
|-------|----------|-----------------|
| Edge (Z=1-6) | H, He, Li, Be, B, C | Fast cycles, small pots, easy exit |
| Mid (Z=7-12) | N, O, F, Ne, Na, Mg | Moderate stakes |
| Inner (Z=13-20) | Al, Si, P, S, Cl, Ar, K, Ca | Larger pots, slower cycles |
| Deep (Z=21-25) | Sc, Ti, V, Cr, Mn | High stakes, difficult escape |
| Core (Z=26) | Fe (Iron) | Maximum depth, terminal value sink |

Deeper Elements have **larger curves**—they absorb more value before resetting. This creates a natural gradient where value tends to flow inward.

### Adjacency

Two Elements are **adjacent** if they share a full tile edge (not just corners). Movement between Elements requires adjacency.

## Charges

Players do not exist on the board directly. They interact through **Charges**—entities they control.

### States

A Charge is always in one of two states:

| State | Meaning |
|-------|---------|
| **Bound** | On the board, occupying an Element |
| **Unbound** | Off the board, in the player's control |

A bound Charge occupies an entire Element (all its tiles simultaneously). Multiple Charges from different players can occupy the same Element—positions are not scarce, but **timing** is.

### What a Charge Holds

- **Balance**: Gluon amount (determines fee costs and commitment measurement)
- **Share**: Commitment share in the current Element (determines reward at reset)
- **Index**: Which Element and generation the Charge is bound to

## Value Flow

### The Pot

Each Element maintains a single **pot**—accumulated value that grows from:

1. **Movement fees** — Paid when Charges enter, exit, or move through
2. **Compression fees** — Paid when pots are merged inward
3. **Donations** — Voluntary contributions via Vent

The pot is visible to everyone. All players watching an Element see the same number.

### Saturation

Each Element tracks **saturation**—the sum of commitment shares of all currently bound Charges.

- Entry increases saturation
- Exit decreases saturation
- Saturation is **live**: it reflects only currently bound Charges, with no memory of past presence

When saturation exceeds the threshold (100%), the Element **resets**.

### The Reset Cycle

When an Element resets:

1. **Distribution**: Pot divided proportionally by commitment share
2. **Ejection**: All Charges except the trigger are unbound (free exit)
3. **First-mover**: Triggering Charge remains bound with first position in new cycle
4. **Clear**: Pot and saturation reset to zero
5. **Generation**: Element's generation counter increments

The cycle then begins again. The game runs forever.

## Commitment

### The Sigmoid Curve

When a Charge binds to an Element, its **commitment share** is measured based on:

1. **Charge's Gluon balance**
2. **Current saturation** (position on the curve)
3. **Element depth** (deeper = larger curve)

The measurement follows a **sigmoid curve**:

```
Share
  │
  │                    ┌─────── Late: marginal share
  │               ╱────┘
  │          ╱────
  │     ╱────
  │ ────           ← Inflection point (50% saturation)
  │
  └────────────────────────── Saturation
       Early: large share
```

- **Early arrival** (low saturation): Large share per Gluon (~20× efficiency)
- **Mid arrival** (near inflection): Diminishing returns
- **Late arrival** (near threshold): Marginal share, but can trigger reset

This rewards patience without excluding late participants.

### Commitment Is Measured, Not Paid

Gluon is **not locked or staked**. The balance is an input to measurement—the Charge keeps its Gluon. This means:

- Larger Charges get larger shares (proportional)
- The same Charge can rebind elsewhere with its full balance
- No "staking" or "locking" mechanics

## Fees

All voluntary actions cost fees. Fees are **never burned**—they flow into Element pots, becoming future rewards.

### What Affects Fees

| Factor | Effect | Implication |
|--------|--------|-------------|
| **Distance²** | Quadratic scaling | Long jumps are exponentially expensive |
| **Saturation** | Linear scaling | Crowded Elements cost more to enter/exit |
| **Speed tax** | Up to 128× multiplier | Rapid actions are punished |
| **Balance** | Linear scaling | Larger Charges pay proportionally more |

### Fee Direction

Movement fees route to Element pots based on direction:

| Direction | Fee Based On | Pot Receives |
|-----------|--------------|--------------|
| **Inward** (toward Fe) | Destination saturation | Destination pot |
| **Outward** (toward edge) | Source saturation | Source pot |

Since deeper Elements typically have lower saturation (larger curves), **inward moves tend to be cheaper**. This creates natural value flow toward the center.

### Speed Tax

A multiplier (1× to 128×) applies based on time since the Charge's last action:

- **Immediate action**: 128× fee multiplier
- **After ~51 seconds**: 1× fee multiplier (full decay)

This eliminates speed as an advantage. The game rewards **timing**, not reflexes.

## The Closed Loop

All value circulates:

```
Actions cost Gluon
       ↓
Costs become pots
       ↓
Pots attract Charges
       ↓
Charges create saturation
       ↓
Saturation triggers reset
       ↓
Reset distributes pot
       ↓
Charges reposition
       ↓
(cycle repeats)
```

Nothing is burned. Every fee paid creates opportunity for future participants.

## Key Invariants

These rules **never change**:

| Invariant | Meaning |
|-----------|---------|
| Waiting is free | Being bound costs nothing—no rent, decay, or passive drain |
| Costs only on action | Players who never act never pay |
| No burning | All costs become shared value |
| Binary binding | A Charge is fully bound to one Element, or not on board |
| Live saturation | Only currently bound Charges count |
| Reset by entry | Resets are triggered by entry crossing threshold, never by exit |
| Free reset exit | Ejected Charges pay no fees |
| Full transparency | All state is visible—pots, saturation, positions |

## Next Steps

- **[Operations](OPERATIONS.md)** — How to perform each action
- **[Strategy](STRATEGY.md)** — How mechanics interact
- **[Reference](REFERENCE.md)** — Fees, formulas, board map
