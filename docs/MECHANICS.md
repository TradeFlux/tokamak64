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
| **Bind** | Bind a Charge onto the board into a target **edge Element only**. Charge becomes bound and participates in saturation mechanics. Edge Elements: H, He, Li, Be, B, C (touch board perimeter). |
| **Unbind** | Voluntarily unbind a Charge from its **current edge Element only**. Applies exit cost. Unbound Charges cannot claim future rewards from that Element. |

### Movement & Value Transfer

| Instruction | Purpose |
|-------------|---------|
| **Rebind** | Move a bound Charge to an adjacent Element. Fee uses destination saturation (inward) or source saturation (outward). Incurs movement costs plus speed tax. |
| **Compress** | Move an Element's pot to an adjacent Element with higher Z while rebinding the Charge. Can be sideways (same depth) or skip depths, as long as dst.index > src.index and Elements are adjacent. Incurs compression fee (added to moved pot). Cost scales with pot size and depth difference—strategic routing. |
| **Vent** | Donate part of a bound Charge's Gluon to its current Element's pot. Charge must be bound to that Element. Does not affect commitment share or saturation. |

### Reset & Rewards

| Instruction | Purpose |
|-------------|---------|
| **Overload** | Trigger an Element reset when saturation exceeds threshold. Typically executed atomically in the same transaction as the Rebind/Bind that pushes saturation over max. Triggering Charge receives its share and re-binds to the reset Element (first-mover advantage). All other Charges unbound for free. |
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

**Element names** (atomic number): H (1), He (2), Li (3), Be (4), B (5), C (6), N (7), O (8), F (9), Ne (10), Na (11), Mg (12), Al (13), Si (14), P (15), S (16), Cl (17), Ar (18), K (19), Ca (20), Sc (21), Ti (22), V (23), Cr (24), Mn (25), **Fe (26)**.

### Bound vs. Unbound

| State | Condition | Behavior |
|-------|-----------|----------|
| **Bound** | `index != 0` | On-board, participates in Element mechanics, can claim if Element resets |
| **Unbound** | `index == 0` | Off-board, cannot participate or claim |

Generation enables detecting stale Charge references after Element reset. A Charge can only claim from an artefact if its index exactly matches (same atomic number AND generation).

## Fee Routing

### Fee Calculation

All movement fees (Rebind, Bind, Unbind) scale based on:
- **Distance²** (quadratic) — Atomic number difference; long jumps are exponentially expensive
- **Saturation** (linear) — Element fullness (0-100%); crowded Elements cost more
- **Speed tax** (up to 128×) — Time since last action; immediate moves are prohibitively expensive
- **Balance** (linear) — Charge's Gluon amount

The total fee is: `base_fee × speed_multiplier`

Fees are proportional to balance, so larger Charges pay proportionally more for the same move.

### Rebind (Movement)

Fee destination depends on direction:

| Direction | Condition | Fee Recipient | Rationale |
|-----------|-----------|---------------|-----------|
| **Inward** | `src.index < dst.index` | Destination Element | Funds deeper Elements, accelerating value accumulation toward center |
| **Outward** | `src.index > dst.index` | Source Element | Taxes departing Charges, incentivizing sustained commitment |

### Compress

Moves to any adjacent Element where dst.index > src.index (can be sideways at same depth or skip depths). Compression incurs two fees (both added to destination pot):
- **Rebind fee** — standard movement fee (distance, saturation, speed tax)
- **Compression fee** — consolidation tax: up to 5% of source pot, scaled by saturation

The compression fee scales from 0% (empty element) to 5% (fully saturated). Both fees are paid by the Charge and added to the destination pot. The source pot then merges with destination, making the resulting pot strictly larger than the sum of original source + destination pots.

**Strategic routing**: Since costs vary by depth difference and board topology, compression direction is strategic. Element 1 might compress to adjacent Element 2 (sideways) or Element 7 (skipping depths), depending on board layout and cost optimization.

### Speed Tax

Movement costs scale with time since last action:

The speed multiplier decays **quadratically** from 128× (immediate action) to 1× (full decay after 1024 slots, ~51 seconds on L2).

## Overload Mechanics

Overload typically occurs atomically in the same transaction as the triggering action:
1. **Rebind or Bind** — Pushes saturation over threshold
2. **Overload** — Executed immediately in same transaction

When Overload executes:

1. **Snapshot** — Element pot and index copied to Artefact
2. **Claim (triggering Charge)** — Triggering Charge receives reward based on its share
3. **Reset** — Element generation increments, curve/pot/saturation cleared
4. **Bonus** — Triggering Charge immediately re-binds to the reset Element (first-mover advantage: early share in fresh cycle)
5. **Unbinding** — All other bound Charges become unbound (free exit)

**Key invariant:** Only the triggering Charge stays bound; all others are unbound for free (no exit costs during reset).

**Note:** While Overload can be called separately, it's typically bundled with the triggering Rebind/Bind for atomicity.

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

Index match validates the Charge was bound at that specific reset cycle.

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
| **Compression requires Z increase** | Always dst.index > src.index (toward higher Z). Can be sideways or skip depths as long as Elements are adjacent and Z increases. |
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

### Non-Rules

No interest/inflation, no decay, no forced stakes, no grief protection, no fairness correction.

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

### Bind / Unbind
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] element   (writable)  - Edge element (dst for Bind, src for Unbind)
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

## Game Constants

| Constant | Value | Description |
|----------|-------|-------------|
| **Elements** | 26 | Total Elements on board (H to Fe) |
| **Saturation range** | 0-100% | Empty to reset threshold |
| **Speed tax** | 1× to 128× | Multiplier based on slots since last action |
| **Speed decay** | 1024 slots (~51s) | Full decay on L2 (50ms slots) |
| **Min fee** | 0.1 GLUON | Minimum fee to prevent dust |

## Math Engine Reference

**Commitment share on bind**: `sigmoid(current_saturation) × GLUON_balance × depth_scaling(Z)`
- Early → strong influence
- Late → marginal
- Deeper Z → larger curve (more Gluon required for equivalent share)

**Saturation**: Instant sum of current fixed shares.

**Reset threshold**: Fixed at 1.0 normalized.

**Reset resolution**: Payout pot by shares → clear pot/saturation → free unbinding (except trigger remains).

**Costs**: Voluntary actions only; speed tax multiplier; inward fees use destination saturation, outward fees use source saturation.

**Compression**: To adjacent Element with higher Z (can be sideways/skip depths); pay rebind fee + compression fee (0-5% of pot, scaled by saturation), then merge source pot into destination. Total cost scales with distance, pot size, and saturation.

**Contributions**: Direct irreversible pot add; no saturation/influence effect.
