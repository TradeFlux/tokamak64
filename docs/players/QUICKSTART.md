# Quick Start

Playing TOKAMAK64 in 5 minutes.

## Prerequisites

- Solana wallet with SOL for tx fees
- USDT or USDC

## Step 1: Enter the Economy

Convert stablecoins to Gluon:

```
Infuse: USDC → Gluon (1:1)
```

Creates a **Wallet** holding Gluon. Liquid—withdraw anytime.

## Step 2: Create a Charge

Allocate Gluon from Wallet to a **Charge**:

```
Charge: Wallet → Charge account
```

Charges enter the board. You can have multiple.

## Step 3: Enter the Board

Place Charge on an edge Element via **Bind**:

```
Bind: Charge → Element (H through Mg only)
```

Now **bound** and playing. Contributes to saturation, earns rewards at reset.

## Step 4: Play

Core loop:

| Action | What it does |
|--------|--------------|
| **Rebind** | Move to adjacent Element |
| **Wait** | Build share value (free) |
| **Overload** | Trigger reset when saturation > 100% |
| **Claim** | Collect rewards after someone else triggers |

## Step 5: Exit

When done:

1. **Unbind** — leave board (must be at edge)
2. **Discharge** — merge Charge back to Wallet
3. **Extract** — convert Gluon to stablecoins

## The Goal

Be present when Elements reset. Resets distribute the pot proportionally.

Consider: *Where and when will the next profitable reset happen?*

- Arrive early → bigger share
- Arrive late → either trigger reset, or get small share + free ejection
- Go deeper → bigger pots, higher fees to leave

## Quick Reference

| Term | Meaning |
|------|---------|
| **Gluon** | In-game currency (1:1 with USDC) |
| **Charge** | Entity that enters the board |
| **Element** | Board region (26 total, H through Fe) |
| **Saturation** | How "full" an Element is (resets at 100%) |
| **Pot** | Value accumulated (distributed at reset) |
| **Reset** | Element clears; pot splits among bound Charges |

## Next

- **[Concepts](CONCEPTS.md)** — mental model
- **[Operations](OPERATIONS.md)** — all actions detailed
- **[Strategy](STRATEGY.md)** — how mechanics interact
- **[Reference](REFERENCE.md)** — fees, formulas, board map
- **[Glossary](../GLOSSARY.md)** — all terms
