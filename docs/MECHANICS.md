# Mechanics

This document covers the technical mechanics of TOKAMAK64: instructions, fee routing, invariants, and account layouts.

## Instructions

The game provides 13 instructions.

### Account Initialization

| Instruction | Purpose |
|-------------|---------|
| **InitCharge** | Initialize a new Charge account (PDA). Derives from signer + counter. Multiple Charges per player allowed. |
| **InitWallet** | Initialize a new Wallet account (PDA). Derives from signer + mint. Holds Gluon. |

### Wallet & Balance Management

| Instruction | Purpose |
|-------------|---------|
| **Infuse** | Convert stablecoins (USDT/USDC) into Gluon in your Wallet (1:1). Entry point for on-chain value. |
| **Extract** | Convert Wallet Gluon back to stablecoins in your ATA. Exit point for on-chain value. |
| **Charge** | Create a new Charge by allocating Gluon from Wallet to Charge account. |
| **Discharge** | Merge a Charge's remaining Gluon back into your Wallet account. |

### Entry & Exit

| Instruction | Purpose |
|-------------|---------|
| **Inject** | Bind a Charge onto the board into a target **edge Element only**. Charge becomes bound and participates in saturation mechanics. Edge Elements: H, He, Li, Be, C (touch board perimeter). |
| **Eject** | Voluntarily unbind a Charge from its **current edge Element only**. Applies exit cost. Unbound Charges cannot claim future rewards from that Element. |

### Movement & Value Transfer

| Instruction | Purpose |
|-------------|---------|
| **Rebind** | Move a bound Charge from one Element to an adjacent Element. Inward movement (toward higher Z) is cheaper; outward movement (toward lower Z) is more expensive. Incurs movement costs plus speed tax. |
| **Compress** | Move an Element's pot inward to a deeper adjacent Element while rebinding the Charge. Incurs migration fee + merge fee (both added to moved pot). Inward-only. |
| **Vent** | Donate part of a bound Charge's Gluon to its current Element's pot. Charge must be bound to that Element. Does not affect commitment share or saturation. |

### Reset & Rewards

| Instruction | Purpose |
|-------------|---------|
| **Overload** | Trigger an Element reset when saturation exceeds threshold. Typically executed atomically in the same transaction as the Rebind/Inject that pushes saturation over max. Triggering Charge receives its share and re-binds to the reset Element (first-mover advantage). All other Charges ejected for free. |
| **Claim** | Collect proportional reward share from an Element's pot after reset. Requires exact index match (atomic number + generation). Only for Charges that were bound at reset instant. |

## Element Identity

Each Element has a unique **index** encoding identity and versioning:

```
┌─────────────────────────────────────────────────────────┬──────────┐
│              Generation (56 bits)                       │ Atomic # │
│                                                         │ (8 bits) │
└─────────────────────────────────────────────────────────┴──────────┘
```

- **Atomic Number** (8 MSB): 1–26 (26 Elements on board, 0 is off-board placeholder)
- **Generation** (56 LSB): Counter incremented when Element resets

**Element names** (atomic number): H (1), He (2), Li (3), Be (4), C (5), N (6), O (7), F (8), Ne (9), Na (10), Mg (11), Al (12), Si (13), P (14), S (15), Cl (16), Ar (17), K (18), Ca (19), Sc (20), Ti (21), V (22), Cr (23), Mn (24), **Fe (25)**, **Ni (26)**.

### Bound vs. Unbound

| State | Condition | Behavior |
|-------|-----------|----------|
| **Bound** | `index != 0` | On-board, participates in Element mechanics, can claim if Element resets |
| **Unbound** | `index == 0` | Off-board, cannot participate or claim |

Generation enables detecting stale Charge references after Element reset. A Charge can only claim from an artefact if its index exactly matches (same atomic number AND generation).

## Fee Routing

### Rebind (Movement)

Fee destination depends on direction:

| Direction | Condition | Fee Recipient | Rationale |
|-----------|-----------|---------------|-----------|
| **Inward** | `src.index < dst.index` | Destination Element | Funds deeper Elements, accelerating value accumulation toward center |
| **Outward** | `src.index > dst.index` | Source Element | Taxes departing Charges, incentivizing sustained commitment |

**Why directional routing?** Creates inward value flow toward Fe. All friction ultimately concentrates toward center.

### Compress

Always moves inward. Both fees go to the pot being moved (which then merges with destination):
- **Migration fee** — depth cost
- **Merge fee** — consolidation cost

The resulting pot is always larger than the sum of source + destination pots.

### Speed Tax

Movement costs scale with time since last action:

| Parameter | Value |
|-----------|-------|
| `MAX_SPEED_MULTIPLIER` | 127 |
| `MAX_DELTA_TIMESTAMP` | 1024 slots |
| Full decay time | ~7 minutes (at ~400ms/slot) |

The multiplier decreases linearly from maximum to 1 as time elapses. Acting immediately after a previous action is prohibitively expensive.

**Why speed tax?** Prevents automation/reflex advantage.

## Overload Mechanics

Overload typically occurs atomically in the same transaction as the triggering action:
1. **Rebind or Inject** — Pushes saturation over threshold
2. **Overload** — Executed immediately in same transaction

When Overload executes:

1. **Snapshot** — Element pot and index copied to Artefact
2. **Claim (triggering Charge)** — Triggering Charge receives reward based on its share
3. **Reset** — Element generation increments, curve/pot/saturation cleared
4. **Bonus** — Triggering Charge immediately re-binds to the reset Element (first-mover advantage: early share in fresh cycle)
5. **Ejection** — All other bound Charges become unbound (free exit)

**Key invariant:** Only the triggering Charge stays bound; all others are ejected for free (no exit costs during reset).

**Note:** While Overload can be called separately, it's typically bundled with the triggering Rebind/Inject for atomicity.

## Claim Sequence

Claims happen in two phases:

### During Overload (Internal)

For the triggering Charge only:
1. Overload processor calls `claim()` internally
2. Reward computed and added to Charge balance
3. Charge share cleared (`share = 0`)
4. Charge re-binds to reset Element

### Separate Claim Transaction (External)

For all other Charges that were bound:
1. Charge submits Claim instruction
2. Requires exact index match (atomic # + generation)
3. Reward distributed proportionally
4. Charge becomes unbound

**Why index must match:**
- Validates Charge was actually bound when Element reset
- Different generation = different reset cycle, ineligible
- Prevents claiming from wrong Elements or double-claiming

## System Invariants

These properties **never change**. If something seems to contradict one, the interpretation is wrong.

### Cost and Value Invariants

| Invariant | Description |
|-----------|-------------|
| **Waiting and binding are free** | Being bound costs nothing. No rent, decay, or passive drain. |
| **Costs only on voluntary actions** | If you never act, you never pay. Resets do not charge fees. |
| **Costs are never burned** | All costs become shared value in Element pots. |
| **Compression always increases pot** | Fee is added to moved pot; merged pot is strictly larger. |

### Pot and Entitlement Invariants

| Invariant | Description |
|-----------|-------------|
| **Pots never move alone** | A pot changes location only when a Charge carries it inward via Compress. |
| **Compression is inward-only** | Always toward higher Z. Never outward, never sideways, never skipping depth. |
| **Entitlement at reset only** | Only Charges bound at exact reset instant receive rewards. No reservations. |

### Saturation Invariants

| Invariant | Description |
|-----------|-------------|
| **Binding is binary** | A Charge is either fully bound to one Element, or not on the board. No partial binding. |
| **Saturation is live** | Reflects only currently bound Charges. Entry increases, exit decreases. No memory. |
| **Reset triggered by entry** | A reset is always triggered by an entry that crosses threshold. Exit can delay but never trigger. |
| **Resets are free exits** | When reset happens, Charges are ejected without fees. Rewards distributed. |

### Transparency Invariants

| Invariant | Description |
|-----------|-------------|
| **No hidden state** | Pot sizes, saturation levels, depth, board structure—all visible. |
| **No special players** | Every rule applies identically to everyone. No admin advantages. |

### Non-Rules (Things That Do NOT Exist)

- No interest or inflation timer
- No decay or passive value loss
- No capture or forced stakes
- No grief protection
- No "fairness correction"

If something feels unfair, another player paid more to shape it.

## Account Layouts

### InitWallet / InitCharge
```
[0] signer    (signer)    - Authority
[1] wallet    (writable)  - Wallet PDA
[2] charge    (writable)  - Charge PDA (InitCharge only)
[2] mint      (readonly)  - Token mint (InitWallet only)
```

### Infuse / Extract
```
[0] authority (signer)    - Wallet authority
[1] wallet    (writable)  - Player wallet
[2] src/vault (writable)  - Source (Infuse) or vault (Extract)
[3] mint      (readonly)  - Token mint
[4] vault/dst (writable)  - Vault (Infuse) or destination (Extract)
[5] authority (readonly)  - Vault authority PDA (Extract only)
```

### Charge / Discharge
```
[0] signer    (signer)    - Wallet authority
[1] charge    (writable)  - Charge account
[2] wallet    (writable)  - Player wallet
```

### Inject / Eject
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] element   (writable)  - Edge element (dst for Inject, src for Eject)
[3] board     (writable)  - Global board state
```

### Rebind / Compress
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] src       (writable)  - Source element
[3] dst       (writable)  - Destination element (adjacent)
```

### Claim
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] artefact  (writable)  - Reset element snapshot
```

### Overload
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] target    (writable)  - Element to reset
[3] artefact  (writable)  - Artefact to create
[4] board     (writable)  - Global board state
```

### Vent
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] target    (writable)  - Element to receive donation
```

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `MAX_ATOMIC_NUMBER` | 26 | 26 Elements on board (1-indexed, 0 is off-board) |
| `MAX_SATURATION` | Q8.24 max | Curve position range [0, 12] |
| `MIN_FEE` | 100,000 | Minimum fee in Gluon (dust prevention) |
| `DECIMALS` | 6 | Gluon precision (matches stablecoins) |
| `MAX_SPEED_MULTIPLIER` | 127 | Maximum speed tax multiplier |
| `MAX_DELTA_TIMESTAMP` | 1024 | Slots for full speed tax decay (~7 min) |

## Fixed-Point Formats

| Type | Format | Usage |
|------|--------|-------|
| `Q824` | Q8.24 (u32) | Saturation [0, 12], commitment share |
| `Q1648` | Q16.48 (u64) | Pressure integral, path-independent history |

Conversion: `q824 = actual_value * 2^24`, `q1648 = actual_value * 2^48`

## Math Engine Reference

**Commitment share on bind**: `sigmoid(current_saturation) × GLUON_balance × depth_scaling(Z)`
- Early → strong influence
- Late → marginal
- Deeper Z → larger curve (more Gluon required for equivalent share)

**Saturation**: Instant sum of current fixed shares.

**Reset threshold**: Fixed at 1.0 normalized.

**Reset resolution**: Payout pot by shares → clear pot/saturation → free ejection (except trigger remains).

**Costs**: Voluntary actions only; speed tax multiplier + directional bias (inward cheaper than outward); fees route to deeper pot.

**Compression**: Inward adjacent; carry full pot + merge + add fee to pot.

**Contributions**: Direct irreversible pot add; no saturation/influence effect.
