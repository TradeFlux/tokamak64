# Operations Guide

Every action in TOKAMAK64, organized by purpose.

## Currency Operations

Moving value between stablecoins and Gluon.

### Infuse

Convert stablecoins to Gluon.

```
Player ATA (USDC/USDT) → Infuse → Player Wallet (Gluon)
```

| Aspect | Detail |
|--------|--------|
| Rate | 1:1 (100 USDC = 100 Gluon) |
| First infuse | Creates a player Wallet |
| Cost | Network tx fee only |

Gluon in the Wallet is liquid—withdraw anytime via Extract.

### Extract

Convert Gluon back to stablecoins.

```
Player Wallet (Gluon) → Extract → Player ATA (USDC/USDT)
```

| Aspect | Detail |
|--------|--------|
| Rate | 1:1 |
| Requirement | Wallet balance |
| Cost | Network tx fee only |

Exit from the game economy. Gluon in Charges must Discharge to Wallet first.

---

## Charge Management

Creating and closing Charges.

### Charge

Create a new Charge from Wallet balance.

```
Wallet (Gluon) → Charge → New Charge Account (Gluon)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Wallet balance |
| Result | New unbound Charge |
| Multiple | Create as desired |

Each Charge moves independently.

### Discharge

Merge a Charge back into Wallet.

```
Charge (unbound) → Discharge → Wallet (balance increased)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Charge must be **unbound** |
| Result | Charge closed, balance returns to Wallet |

Bound Charges must Unbind first.

---

## Board Entry and Exit

Getting on and off the board.

### Bind

Place an unbound Charge on the board.

```
Charge (unbound) → Bind → Charge (bound to Element)
```

| Aspect | Detail |
|--------|--------|
| Restriction | **Edge Elements only** (H through Mg) |
| Cost | Movement fee (distance from Z=0 to destination) |
| Effect | Saturation increases |
| Share | Measured at binding (see: Commitment Share) |

Earlier binding = larger share (sigmoid curve).

### Unbind

Remove a Charge from the board.

```
Charge (bound) → Unbind → Charge (unbound)
```

| Aspect | Detail |
|--------|--------|
| Restriction | **Edge Elements only** |
| Cost | Movement fee (distance from source to Z=0) |
| Effect | Saturation decreases |
| Consequence | No rewards from future resets |

From deep Elements, Rebind outward step by step until reaching an edge.

---

## Movement

Repositioning on the board.

### Rebind

Move a Charge to an adjacent Element.

```
Charge (at Element A) → Rebind → Charge (at Element B)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Elements share a full edge |
| Cost | Movement fee (distance, saturation, speed tax, balance) |
| Fee routing | Inward → destination pot; Outward → source pot |
| Effect | Unbinds from source, binds to destination |

Primary positioning action. See [Strategy](STRATEGY.md).

#### Direction Matters

| Direction | Fee Basis | Notes |
|-----------|-----------|-------|
| **Inward** (higher Z) | Destination saturation | Deeper elements typically have lower saturation |
| **Outward** (lower Z) | Source saturation | Cost depends on the current element's saturation |

Fee magnitude depends on the saturation of the curve used, not direction alone.

### Compress

Move an Element's pot to an adjacent deeper Element.

```
Source Element (pot) → Compress → Destination Element (pot increased)
```

| Aspect | Detail |
|--------|--------|
| Requirement | dst.index > src.index (deeper or equal) |
| Requirement | Adjacent Elements |
| Cost | Movement fee + Compression fee (up to 5%) |
| Effect | Source pot empties into destination |

Relocates value:
- Source pot empties
- Destination pot gains source + compression fee
- Net: destination grows by more than source had

Rescues stagnant value. Concentrates pots for bigger resets.

---

## Value Operations

Affecting Element pots.

### Vent

Donate Gluon from Charge to Element pot.

```
Charge (balance) → Vent → Element (pot increased)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Charge bound to the Element |
| Effect | Charge balance down, pot up |
| Saturation | Unchanged |

Signal move: "this Element is worth resetting." Attracts attention—both allies and predators.

---

## Reset Operations

Triggering and claiming resets.

### Overload

Trigger an Element reset.

```
Element (saturated) → Overload → Element (reset) + Artefact (created)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Saturation > 100% |
| Who can trigger | Anyone (once saturation >= 100%), but typically the binder who crossed threshold calls immediately |
| Trigger reward | First position in new cycle |
| Others | Unbound for free (ejected without fee) |

In practice, submit one transaction with:
1. Rebind or Bind that pushes saturation over 100%
2. Overload instruction

Atomic execution—no front-running. However, if a binder pushes past 100% without calling overload, anyone else can call it and capture the trigger reward (rare but possible).

### Claim

Collect reward from a past reset.

```
Artefact (pot snapshot) → Claim → Charge (balance increased)
```

| Aspect | Detail |
|--------|--------|
| Requirement | Charge was bound at reset moment |
| Requirement | Charge index matches Artefact |
| Effect | Proportional share added to balance |
| After | Charge becomes unbound |

For Charges that didn't trigger. Trigger gets share automatically during Overload.

---

## Summary

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
Bind → Rebind (position) → Wait → Overload/Claim → Rebind → ...
```

### Fee Summary

| Operation | Fee Type |
|-----------|----------|
| Infuse | None (network tx only) |
| Extract | None (network tx only) |
| Charge | None (wallet→charge transfer) |
| Discharge | None (charge→wallet merge) |
| Bind | Movement fee (distance: Z=0 → destination) |
| Unbind | Movement fee (distance: source → Z=0) |
| Rebind | Movement fee (distance: source → destination) |
| Compress | Movement + Compression fee (up to 5% of source pot) |
| Vent | None (donation added to pot) |
| Overload | None |
| Claim | None |

See [REFERENCE.md](REFERENCE.md) for formulas and examples.

---

## Common Patterns

### Safe Positioning

1. Bind to edge (low saturation preferred)
2. Wait for saturation to build
3. Trigger Overload or get ejected
4. Claim if needed, reposition

### Deep Raid

1. Bind to edge, Rebind inward incrementally
2. Wait between moves (avoid speed tax)
3. Position in quiet deep Element
4. Strike outward when adjacent saturates

### Value Concentration

1. Find stagnant pots
2. Compress inward toward active Elements
3. Follow compressed value to claim at reset

---

## Next

- **[Strategy](STRATEGY.md)** — how operations combine
- **[Reference](REFERENCE.md)** — exact fees and formulas
