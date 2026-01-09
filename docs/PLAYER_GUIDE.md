# Player Guide

This guide covers TOKAMAK64 operations and strategy.

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

Use **Bind** to place a Charge on the board:

```
Charge (unbound) → Bind → Charge (bound to edge Element)
```

**Important:** You can only bind into edge Elements—H, He, Li, Be, B, C (those touching the board perimeter).

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

When your Rebind or Bind pushes saturation over the threshold, you typically execute **Overload** in the same transaction (two instructions atomically):

- You receive your reward share immediately
- You re-bind to the reset Element (first-mover advantage in fresh cycle)
- All other Charges are unbound for free (repositioning opportunity)

**In practice**: You submit a transaction with both instructions—the Rebind/Bind that triggers the threshold, immediately followed by Overload.

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
- Pays movement fee + compression fee (up to 5% of pot, based on saturation)
- Creates larger, more valuable pots at higher Z
- **Strategic routing**: Costs vary by distance and pot size—choose compression paths carefully

## Exiting the Board

### Voluntary Exit

Use **Unbind** to leave the board:

```
Charge (bound to edge Element) → Unbind → Charge (unbound)
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

Multiple Charges let you diversify across Elements (hedge positioning risk) and test strategies without full commitment.

## Understanding Fee Strategy

Movement fees determine positioning costs and scale based on:
- **Distance²** (quadratic) - How far you're moving
- **Saturation** (linear) - How full the element is (0-100%)
- **Speed tax** (up to 128×) - How recently you last acted
- **Balance** (linear) - Your Charge's GLUON

Fees are proportional to your balance, so a 100 GLUON Charge pays 10× more than a 10 GLUON Charge for the same move.

### Concrete Examples (100 GLUON Charge)

**Note**: Saturation builds from 0% (empty) to 100% (reset threshold). The UI shows saturation as a percentage.

**Adjacent moves with timing:**
```
Element 13→14 at 5% saturation:
  Patient (~51s):  0.10 GLUON
  Rushed (0s):     0.95 GLUON (9×)
```

**Edge to inner (Element 1→13, distance=12):**
```
At 10% saturation:   2.13 GLUON
At 30% saturation:   6.39 GLUON (3× more)
At 60% saturation:  12.78 GLUON (6× more)
At 90% saturation:  19.17 GLUON (9× more)
```

**Patient path to Fe (Element 26):**
```
Incremental movement as saturation builds:
  1→13 (dist=12, 15% saturation):  3.20 GLUON
  13→24 (dist=11, 25% saturation): 4.47 GLUON
  24→25 (dist=1, 40% saturation):  0.10 GLUON
  25→26 (dist=1, 50% saturation):  0.10 GLUON
  ────────────────────────────────────────
  TOTAL (patient):                 7.87 GLUON (7.9% of balance)

Rushed movement (immediate actions):
  TOTAL:                  998.82 GLUON (999% of balance!)
  → Rushing costs 127× more!
```

### Strategic Tradeoffs

| Factor | Effect | Implication |
|--------|--------|-------------|
| **Distance²** | Quadratic scaling | Moving 10 elements costs 100× more than 1; incremental movement is strategic |
| **Saturation** | Linear scaling (0-100%) | Early arrival (<30% saturation) = minimal fees + strong shares; critical advantage |
| **Speed tax** | Up to 128× multiplier | Patience saves massive costs; automation loses advantage |
| **Balance** | Linear scaling | Splitting Charges doesn't reduce total fees; weakens shares proportionally |

**Key insights**:
- **Strategic window**: Arrive early (<30% saturation) for low fees + strong shares
- **Distance dominates**: Long jumps are exponentially expensive
- **Patient incremental movement** through low-saturation elements is optimal
- **Timing matters**: Wait ~51 seconds (1024 slots) between moves to avoid speed tax

### Outward Strikes from Depth

Outward movement fees are based on the **source** Element's saturation, not the destination. This creates opportunity:

- Position a Charge in a quiet deep Element (low saturation, cheap entry)
- Wait for an adjacent shallower Element to saturate and build a large pot
- Strike outward at minimal cost—fees scale with your quiet source, not the crowded target

**Example**: Boron (Z=5) at 95% saturation costs 1.41 GLUON to enter from Beryllium. But entering from Carbon (Z=6, 2% saturation) costs 0.10 GLUON—14× cheaper.

**Implication**: High-saturation pots are always vulnerable from deeper positions. Control deeper staging grounds to strike shallower pots at will.



## Strategic Tradeoffs

Three independent dimensions define positioning strategy:

### Entry Timing

The sigmoid curve creates a tradeoff between share size and certainty:

| Timing | Share | Tradeoff |
|--------|-------|----------|
| Early (low saturation) | Large | High opportunity cost if pot stalls or gets compressed away |
| Late (near threshold) | Small | Lower risk, but depends on accurate threshold prediction |

Trigger advantage partially compensates for late entry—the triggering Charge re-binds first in the fresh cycle.

### Depth

Deeper Elements (higher Z) have larger curves and accumulate bigger pots:

| Depth | Cycle Speed | Stakes | Exit Cost |
|-------|-------------|--------|-----------|
| Shallow (H–C) | Fast | Lower | Cheap (edge unbind) |
| Deep (toward Fe) | Slow | Higher | Expensive (multiple outward rebinds) |

Compression only flows inward, so deep positions are vulnerable to unexpected pot arrivals that trigger resets.

### Activity

Speed tax creates a cost curve for repositioning frequency:

| Activity | Cost | Coverage |
|----------|------|----------|
| Patient (few moves) | Minimal fees | Concentrated exposure |
| Active (frequent repositioning) | Cumulative speed tax | Distributed exposure |

Patience dominates when reset timing is predictable. Active repositioning pays off only when multiple Elements approach threshold simultaneously.

## Equilibrium Considerations

- **No dominant strategy**: Early entry beats late entry on shares, but late entry beats early on certainty. Neither dominates.
- **Compression as coordination problem**: Compressing value creates larger pots but benefits all participants in the destination Element, not just the compressor.
- **Information is symmetric**: All state is public. Edge comes from interpreting incentives, not from information asymmetry.
- **Vent as costly signal**: Contributions grow pots without affecting shares—pure signaling that invites both cooperation and predation.

## Summary

Your edge comes from reading incentives, timing positioning, managing multiple Charges, and understanding fee asymmetry (inward typically cheaper due to curve capacity differences).

Value is created by friction, moved by compression, claimed by presence at reset. The game cycles forever.
