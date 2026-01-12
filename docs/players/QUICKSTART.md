# Quick Start

Get playing TOKAMAK64 in 5 minutes.

## Prerequisites

- A Solana wallet with SOL for transaction fees
- USDT or USDC tokens

## Step 1: Enter the Economy

Convert stablecoins to Gluon (the in-game currency):

```
Infuse: USDC → Gluon (1:1 conversion)
```

This creates a **Wallet** that holds Gluon. The Wallet is the player's resource pool—Gluon here is liquid and can be withdrawn anytime.

## Step 2: Create a Charge

Allocate Gluon from the Wallet to create a **Charge**:

```
Charge: Wallet → Charge account
```

A Charge is the entity that enters the board. Players can create multiple Charges to pursue different positions simultaneously.

## Step 3: Enter the Board

Place the Charge on an edge Element using **Bind**:

```
Bind: Charge → Element (H, He, Li, Be, B, or C only)
```

The Charge is now **bound** and participating in the game. It contributes to the Element's saturation and will receive rewards if the Element resets.

## Step 4: Play

From here, the core loop is:

| Action | What It Does |
|--------|--------------|
| **Rebind** | Move to an adjacent Element |
| **Wait** | Accumulate share value as saturation builds (costs nothing) |
| **Overload** | Trigger a reset when saturation exceeds threshold |
| **Claim** | Collect rewards after someone else triggers a reset |

## Step 5: Exit

When ready to leave:

1. **Unbind** — Remove Charge from board (must be at edge Element)
2. **Discharge** — Merge Charge balance back to Wallet
3. **Extract** — Convert Gluon back to stablecoins

## The Goal

Position Charges to be present when Elements reset. Resets distribute the Element's accumulated pot to all bound Charges proportionally.

**The core question**: *Where and when will the next profitable reset occur?*

- Arrive early → larger share of the pot
- Arrive late → can trigger the reset at a favorable moment
- Move deeper → bigger pots, but harder to escape

## Quick Reference

| Term | Meaning |
|------|---------|
| **Gluon** | In-game currency (1:1 with USDC) |
| **Charge** | Entity that enters the board |
| **Element** | Region on the board (26 total, named H through Fe) |
| **Saturation** | How "full" an Element is (resets at 100%) |
| **Pot** | Value accumulated in an Element (distributed at reset) |
| **Reset** | Element clears; pot distributed to bound Charges |

## Next Steps

- **[Concepts](CONCEPTS.md)** — Understand the mental model
- **[Operations](OPERATIONS.md)** — Detailed guide to each action
- **[Strategy](STRATEGY.md)** — How mechanics interact
- **[Reference](REFERENCE.md)** — Fees, formulas, board map
- **[Glossary](../GLOSSARY.md)** — All terms defined
