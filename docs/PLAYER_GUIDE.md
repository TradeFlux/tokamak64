# Player Guide

This guide walks you through playing TOKAMAK64: operations, emerging archetypes, and profit-chasing strategies.

## Prerequisites

- A Solana wallet
- USDT or USDC tokens
- An Associated Token Account (ATA) for your stablecoin

## Account Types

| Account | Purpose |
|---------|---------|
| **Wallet** | Holds Gluon in liquid form. Your resource pool. Created on first Infuse. |
| **Charge** | A distinct entity that enters the board. You can have multiple Charges. |
| **ATA** | Your Solana token account holding USDT/USDC. Entry/exit point for real value. |

## Getting Started

### 1. Initialize Your Wallet

Your first **Infuse** creates your wallet automatically. This converts stablecoins to Gluon:

```
Your ATA (USDC) → Infuse → Your Wallet (Gluon)
```

The conversion is 1:1. 100 USDC becomes 100 Gluon.

### 2. Create Charges

Use **Charge** to allocate Gluon from your wallet to new Charge accounts:

```
Your Wallet (Gluon) → Charge → Charge Account (Gluon)
```

You can create multiple Charges—each is an independent entity that can be positioned differently on the board.

### 3. Enter the Board

Use **Inject** to place a Charge on the board:

```
Charge (unbound) → Inject → Charge (bound to edge Element)
```

**Important:** You can only inject into edge Elements—H, He, Li, Be, B, C (those touching the board perimeter).

## Playing the Game

Once on the board, you have several options.

### Move Between Elements

**Rebind** moves your Charge to an adjacent Element:

- **Inward** (toward higher Z, toward Fe): Cheaper base cost
- **Outward** (toward lower Z, toward H): More expensive
- Elements must share a full edge (not just corners)
- Moving quickly costs more (speed tax)

**Why the asymmetry?** Creates natural value flow toward the center. Commitment becomes sticky; escaping costs energy.

### Wait for Saturation Buildup

Your Charge contributes to saturation simply by being bound. As more Charges gather and fees are paid:

- The Element's pot grows
- Saturation increases toward threshold
- Eventually, the Element will reset via Overload

**Key insight**: Waiting costs nothing. Early binding grants large commitment shares (strong yields when reset occurs).

### Trigger Overload

When your Rebind or Inject pushes saturation over the threshold, you typically execute **Overload** in the same transaction (two instructions atomically):

- You receive your reward share immediately
- You re-bind to the reset Element (first-mover advantage in fresh cycle)
- All other Charges are ejected for free (repositioning opportunity)

**In practice**: You submit a transaction with both instructions—the Rebind/Inject that triggers the threshold, immediately followed by Overload.

### Donate to the Pot

Use **Vent** to transfer Gluon from your Charge to the Element's pot:

- Reduces your individual capacity
- Accelerates reaching saturation threshold
- Functions as a public signal: "This Element is worth resetting."

**Why signal?** Attracts attention—sometimes allies, sometimes predators, often both.

### Move Pots Inward

Use **Compress** to carry pots to Elements with higher Z:

- Carry current Element's pot to an adjacent Element where dst.index > src.index
- Can be sideways (same depth) or skip depths, as long as Elements are adjacent
- Pays migration + merge fees (both added to the pot)
- Creates larger, more valuable pots at higher Z
- **Strategic routing**: Costs vary by depth difference and board topology—choose compression paths carefully

## Exiting the Board

### Voluntary Exit

Use **Eject** to leave the board:

```
Charge (bound to edge Element) → Eject → Charge (unbound)
```

- Must be in an edge Element (H, He, Li, Be, B, C)
- Pay exit costs
- Keep remaining Gluon
- **Cannot claim rewards from future resets of that Element**

### Exit via Reset

If the Element you're in resets:

- You're ejected automatically for free
- You receive your proportional share of the pot
- No exit cost

This is often better than voluntary exit—but you can't control when it happens.

### Collecting Rewards

After an Element resets (and you didn't trigger it), use **Claim**:

```
Charge (with matching index) → Claim → Charge (balance increased, unbound)
```

Your Charge must have been bound to that specific Element during that specific generation.

### Returning Gluon to Wallet

Use **Discharge** to merge a Charge back into your wallet:

```
Charge (unbound) → Discharge → Wallet (increased balance)
```

### Withdrawing Real Value

Use **Extract** to convert Gluon back to stablecoins:

```
Wallet (Gluon) → Extract → Your ATA (USDC)
```

## Core Tension: Where Will the Next Reset Occur?

The fundamental question driving all strategy is:

> **"Where and when will the next profitable reset occur?"**

This is **not** a stay/leave decision. It's **profit-chasing via constant repositioning**:

### Early vs. Late Positioning

- **Early arrival** (low saturation): Large commitment share via sigmoid curve—strong yields when reset finally occurs
- **Late arrival** (near threshold): Diminishing share returns, but can trigger at a favorable moment

You constantly balance:
- Arriving early enough for good shares
- Not arriving so early that you're trapped with mediocre pots
- Guessing when reset is imminent
- Reading whether to compress pots to reshape the board

### Multiple Charges = Multiple Bets

Having multiple Charges lets you:
- Diversify across Elements (hedge positioning risk)
- Test hypotheses without full commitment
- Play different archetypes simultaneously

## Understanding Fee Strategy

Movement fees determine positioning costs. The formula:

```
fee = balance × (distance² × saturation × speed_tax) / (26² × 6.0)
```

The **multiplier is computed first**, then applied to your balance. This keeps fees proportional.

### Concrete Examples (100 GLUON Charge)

**Note**: Saturation ranges from 0.0 to 6.0. Reset occurs at 6.0 (100% full). Percentages show how full the element is.

**Adjacent moves with timing:**
```
Element 13→14 at sat=0.3 (5% full):
  Patient (7 min):  0.10 GLUON
  Rushed (0s):      0.95 GLUON (9×)
```

**Edge to inner (Element 1→13, distance=12):**
```
At sat=0.6 (10% full), patient:   2.13 GLUON
At sat=1.8 (30% full), patient:   6.39 GLUON (3× more)
At sat=3.6 (60% full), patient:  12.78 GLUON (6× more)
At sat=5.4 (90% full), patient:  19.17 GLUON (9× more)
```

**Patient path to Fe (Element 26):**
```
Incremental movement as saturation builds:
  1→13 (d=12, sat=0.9, 15% full):  3.20 GLUON
  13→24 (d=11, sat=1.5, 25% full): 4.47 GLUON
  24→25 (d=1, sat=2.4, 40% full):  0.10 GLUON
  25→26 (d=1, sat=3.0, 50% full):  0.10 GLUON
  ────────────────────────────────────────
  TOTAL (patient):                 7.87 GLUON (7.9%)

Rushed movement (immediate actions):
  TOTAL:                         998.82 GLUON (999%)
  → Rushing costs 127× more!
```

### Strategic Tradeoffs

| Factor | Effect | Implication |
|--------|--------|-------------|
| **Distance²** | Quadratic scaling | d=10 costs 100× more than d=1; incremental movement is strategic |
| **Saturation** | Linear scaling (0-6.0) | Early arrival (<30% full) = minimal fees + strong shares; critical advantage |
| **Speed tax** | Up to 128× multiplier | Patience saves massive costs; automation loses advantage |
| **Balance** | Linear scaling | Splitting Charges doesn't reduce total fees; weakens shares proportionally |

**Key insights**:
- **Saturation range**: 0.0 to 6.0 (reset at 100% full)
- **Strategic window**: Arrive early (<30% full) for low fees + strong shares
- **Distance dominates**: A rushed long jump can cost 10× your balance
- **Patient incremental movement** through low-saturation elements is optimal

## Player Archetypes

The game has no explicit roles, but recurring patterns emerge from profit-chasing incentives:

### Sentinels (Patient Holders)

**Strategy**: Early binders who wait through long buildup periods.

- Inject into low-saturation Elements
- Accept weak initial pots in exchange for strong shares
- Wait for others to add value (fees, compressions)
- Reap large shares when reset finally occurs

**Risk**: Element stalls (nobody else arrives); opportunity cost of waiting.

### Catalysts (Precision Triggers)

**Strategy**: Late arrivals who trigger profitable resets with surgical timing.

- Monitor building saturation across multiple Elements
- Enter precisely when reset is imminent and pot is large
- Accept weak shares; profit comes from triggering large pots
- Receive first-mover advantage by re-binding to reset Element

**Risk**: Misjudge timing (arrive too early or too late); speed tax if repositioning rapidly.

### Shepherds / Compressors (Flow Redirectors)

**Strategy**: Actively reshape value flows via compression.

- Identify stagnant pots in shallow Elements
- Compress value inward to create massive deep pots
- Follow the value deeper, positioning for eventual reset
- Sometimes deliberately escalate to trigger resets

**Risk**: Compression costs scale with pot size; may escalate beyond control; attracts competition.

### Arsonists (Aggressive Escalators)

**Strategy**: Deliberately destabilize via compression and contributions (Vent).

- Use Vent to rapidly grow pots
- Compress aggressively to push value toward center
- Trigger chaos and volatility
- Profit from resulting resets or manipulate timing

**Risk**: Creates unpredictable dynamics; may benefit competitors; burns resources without guaranteed returns.

### Scavengers (Shallow Farmers)

**Strategy**: Harvest quick, small cycles in outer Elements (H, He, Li, Be, B, C).

- Focus exclusively on edge and shallow-mid Elements
- Capitalize on fast saturation buildup (smaller curves)
- Accept lower yields for higher turnover
- Minimize depth risk (easy to eject if needed)

**Risk**: Low absolute profits; vulnerable to compression (value moved away before reset).

### Divers (Deep Specialists)

**Strategy**: Position for massive Iron (Fe, Z=26) and deep Element resets.

- Move deep into the board via multiple Rebinds
- Tolerate high costs and entrapment risk
- Wait for enormous pots to accumulate
- Profit from deep resets and (future) Quantum Pocket triggers

**Risk**: Trapped deep with no cheap escape; opportunity cost of waiting; compression may trigger unexpected resets.

### Opportunists (Reactive Movers)

**Strategy**: No fixed plan—react to sudden spikes in saturation or pot size.

- Monitor all Elements constantly
- Rapidly reposition when opportunities emerge
- Accept speed tax when necessary
- Flexible, chaotic, high-frequency

**Risk**: Speed tax makes rapid repositioning expensive; lack of early shares reduces yields.

## Reading the Board

All state is public:

- Every Element's pot size
- Every Element's saturation level
- Every Charge's position
- Proximity to saturation thresholds

Use this information to time your positioning.

**No hidden state, only interpretation skill.**

## What Skilled Players Look Like

A skilled player:

- Waits more than they act (patience beats speed)
- Understands when pots are dangerous (vulnerable to compression, near reset)
- Uses compression sparingly (escalation, not safety)
- Contributes (Vent) only with intent (signals, not random seeding)
- Respects directional bias (inward cheap, outward expensive)
- Positions based on "where/when will next reset occur?" not "should I stay or leave?"

They lose value sometimes. Everyone does.

What separates them is **they know why they lost it**.

## Temporary Alliances

No chat needed. Aligned presence creates temporary cooperation:

- Multiple Charges in same Element share interest in reset timing
- Compressors moving value inward create opportunities for Catalysts
- Sentinels seed value that Shepherds can redirect

These alliances dissolve immediately after reset—everyone repositions.

## Common Mistakes

### Treating It Like a Hold/Exit Game

**Wrong mindset**: "Should I stay or leave?"
**Right mindset**: "Where is the next profitable reset?"

This isn't about commitment duration. It's about positioning timing.

### Compressing for Safety

Compression grows pots (via fee) and attracts attention. It's **escalation**, not **protection**.

Compress to reshape the board or create opportunities—never to "secure" value.

### Ignoring Speed Tax

Acting twice quickly is prohibitively expensive.

Each action asks: **"Is this worth doing now, or should I wait?"**

Patience is a first-class strategy, not a fallback.

### Confusing Contributions with Investment

Vent doesn't buy shares or protection. It broadcasts intent.

Contribute to signal or seed—never expecting guaranteed returns.

## Summary

The game is entirely transparent. Your edge comes from:

1. **Reading incentives**: Where is saturation building? Where are pots growing? Where will next reset occur?
2. **Timing positioning**: Balance early shares vs. reset imminence
3. **Managing multiple Charges**: Diversify, hedge, test strategies
4. **Respecting directional bias**: Inward cheap (natural flow toward center), outward expensive

Remember:

> **TOKAMAK64 is a game where value is created by friction, moved by risk (compression), and claimed by presence (being bound at reset instant).**

The game cycles forever. No shutdown, no victory screen—only positioning, timing, and profit-chasing.

Everything else is implementation detail.
