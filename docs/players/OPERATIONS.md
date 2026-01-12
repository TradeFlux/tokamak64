# Operations Guide

This document covers every action available in TOKAMAK64, organized by purpose.

## Currency Operations

These operations move value between the external economy (stablecoins) and the game economy (Gluon).

### Infuse

**Purpose**: Convert stablecoins to Gluon.

```
Player ATA (USDC/USDT) → Infuse → Player Wallet (Gluon)
```

| Aspect | Detail |
|--------|--------|
| Conversion rate | 1:1 (100 USDC = 100 Gluon) |
| First infuse | Creates the player's Wallet automatically |
| Cost | Network transaction fee only |

Gluon in the Wallet is liquid—it can be withdrawn anytime via Extract.

### Extract

**Purpose**: Convert Gluon back to stablecoins.

```
Player Wallet (Gluon) → Extract → Player ATA (USDC/USDT)
```

| Aspect | Detail |
|--------|--------|
| Conversion rate | 1:1 |
| Requirement | Sufficient Wallet balance |
| Cost | Network transaction fee only |

This is the exit from the game economy. Gluon in Charges must first be Discharged to the Wallet.

---

## Charge Management

These operations manage Charges—the entities that enter the board.

### Charge

**Purpose**: Create a new Charge by allocating Gluon from Wallet.

```
Wallet (Gluon) → Charge → New Charge Account (Gluon)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Sufficient Wallet balance |
| Result | New unbound Charge with specified balance |
| Multiple | Players can create many Charges |

Each Charge is independent and can be positioned differently on the board.

### Discharge

**Purpose**: Merge a Charge's balance back into the Wallet.

```
Charge (unbound) → Discharge → Wallet (increased balance)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Charge must be **unbound** (off board) |
| Result | Charge account closed, balance added to Wallet |

A Charge on the board must first Unbind before it can Discharge.

---

## Board Entry and Exit

These operations control when Charges enter and leave the board.

### Bind

**Purpose**: Place an unbound Charge onto the board.

```
Charge (unbound) → Bind → Charge (bound to Element)
```

| Aspect | Detail |
|--------|--------|
| Restriction | **Edge Elements only** (H, He, Li, Be, B, C) |
| Cost | Injection fee (see [Reference](REFERENCE.md)) |
| Effect | Charge becomes bound; saturation increases |
| Commitment | Share measured at binding moment |

The commitment share depends on the Element's current saturation—earlier binding yields larger shares.

### Unbind

**Purpose**: Remove a Charge from the board voluntarily.

```
Charge (bound) → Unbind → Charge (unbound)
```

| Aspect | Detail |
|--------|--------|
| Restriction | **Edge Elements only** (must navigate to edge first) |
| Cost | Ejection fee |
| Effect | Charge becomes unbound; saturation decreases |
| Consequence | Cannot claim rewards from future resets of that Element |

To exit from a deep Element, a player must Rebind outward step by step until reaching an edge Element.

---

## Movement

These operations reposition Charges on the board.

### Rebind

**Purpose**: Move a bound Charge to an adjacent Element.

```
Charge (bound to Element A) → Rebind → Charge (bound to Element B)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Elements must be **adjacent** (share full edge) |
| Cost | Movement fee (distance, saturation, speed tax, balance) |
| Fee routing | Inward → destination pot; Outward → source pot |
| Effect | Unbinds from source, binds to destination |

This is the primary positioning action. See [Strategy](STRATEGY.md) for movement considerations.

#### Direction Matters

| Direction | Fee Basis | Typical Cost |
|-----------|-----------|--------------|
| **Inward** (higher Z) | Destination saturation | Usually cheaper |
| **Outward** (lower Z) | Source saturation | Usually more expensive |

The asymmetry creates a "gravity" toward the center—entering is easier than leaving.

### Compress

**Purpose**: Move an Element's pot to an adjacent deeper Element.

```
Source Element (pot) → Compress → Destination Element (pot increased)
```

| Aspect | Detail |
|--------|--------|
| Requirement | dst.index > src.index (must go deeper or sideways-deeper) |
| Requirement | Elements must be adjacent |
| Cost | Movement fee + Compression fee (up to 5% of pot) |
| Effect | Source pot merges into destination pot |

Compression relocates value:

- **Source pot**: Emptied (merged into destination)
- **Destination pot**: Receives source pot + compression fee
- **Net effect**: Destination pot grows by more than the source pot amount

This rescues value from stagnant Elements and concentrates it for larger resets.

---

## Value Operations

These operations affect the value in Elements.

### Vent

**Purpose**: Donate Gluon from a Charge to the Element's pot.

```
Charge (balance) → Vent → Element (pot increased)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Charge must be bound to the target Element |
| Effect | Charge balance decreases; Element pot increases |
| No saturation change | Does not affect commitment shares |

Vent is a **signal**—it says "this Element is worth resetting" without changing the saturation dynamics. This attracts attention from other players.

---

## Reset Operations

These operations relate to Element resets.

### Overload

**Purpose**: Trigger an Element reset when saturation exceeds threshold.

```
Element (saturated) → Overload → Element (reset) + Artefact (created)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Element saturation > 100% |
| Typical use | Bundled atomically with the Rebind/Bind that crosses threshold |
| Trigger reward | Triggering Charge receives share immediately |
| Trigger advantage | Triggering Charge re-binds first in new cycle |
| Other Charges | Unbound for free (no exit cost) |

In practice, players submit a transaction containing both:
1. The Rebind or Bind that pushes saturation over threshold
2. The Overload instruction

This ensures atomic execution—no one can front-run the reset.

### Claim

**Purpose**: Collect reward share from a past reset.

```
Artefact (pot snapshot) → Claim → Charge (balance increased)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Charge was bound at exact reset moment |
| Requirement | Charge index matches Artefact (same Element + generation) |
| Effect | Proportional share of pot added to Charge balance |
| After claim | Charge becomes unbound |

Claim is used by Charges that were bound during a reset they did not trigger. The triggering Charge receives its share automatically during Overload.

---

## Operation Summary

### Entry Flow

```
USDC → Infuse → Wallet → Charge → Bind → Board
```

### Exit Flow

```
Board → Unbind → Discharge → Wallet → Extract → USDC
```

### Core Loop

```
Bind → Rebind (position) → Wait → Overload/Claim → Rebind (reposition) → ...
```

### Fee Summary

| Operation | Fee Type |
|-----------|----------|
| Infuse | None (tx fee only) |
| Extract | None (tx fee only) |
| Charge | None (tx fee only) |
| Discharge | None (tx fee only) |
| Bind | Injection fee |
| Unbind | Ejection fee |
| Rebind | Movement fee |
| Compress | Movement fee + Compression fee |
| Vent | None (donation is the cost) |
| Overload | None |
| Claim | None |

---

## Common Patterns

### Safe Positioning

1. Bind to edge Element (low saturation preferred)
2. Wait for saturation to build
3. Either trigger Overload or get ejected on someone else's trigger
4. Claim if needed, then reposition

### Deep Raid

1. Bind to edge, Rebind inward incrementally
2. Wait between moves to avoid speed tax
3. Position in quiet deep Element
4. Strike outward when adjacent Element saturates

### Value Concentration

1. Identify stagnant pots
2. Compress them inward toward active Elements
3. Follow the compressed value to claim at reset

---

## Next Steps

- **[Strategy](STRATEGY.md)** — How these operations combine
- **[Reference](REFERENCE.md)** — Exact fee formulas and numbers
