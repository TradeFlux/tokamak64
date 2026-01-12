# Strategy Guide

This document explores how TOKAMAK64 mechanics interact and what strategies emerge from their combination.

## The Fundamental Question

Every decision reduces to:

> **Where and when will the next profitable reset occur?**

This is not a stay/leave decision—it's profit-chasing through constant repositioning. The game has no safe harbor; value is captured by being present at the right place at the right time.

---

## The Three Tradeoffs

Strategy in TOKAMAK64 involves three independent dimensions. Optimizing one does not determine the others.

### 1. Entry Timing

When to bind to an Element relative to its saturation:

| Timing | Share | Risk |
|--------|-------|------|
| **Early** (low saturation) | Large | Pot may stagnate; opportunity cost of waiting |
| **Mid** (building saturation) | Moderate | Balanced risk/reward |
| **Late** (near threshold) | Small | Can trigger reset; depends on accurate prediction |

The sigmoid curve creates this tradeoff mathematically:
- Early binding: ~20× share efficiency per Gluon
- Late binding: Marginal share, but timing advantage

**Key insight**: The trigger gets first position in the new cycle—partial compensation for late entry.

### 2. Depth Selection

Where to position along the edge-to-core gradient:

| Depth | Pot Size | Cycle Speed | Escape Cost |
|-------|----------|-------------|-------------|
| **Shallow** (H–C) | Small | Fast | Low (edge unbind) |
| **Mid** (Al–Ca) | Medium | Moderate | Medium (several rebinds) |
| **Deep** (Sc–Mn) | Large | Slow | High (many rebinds) |
| **Core** (Fe) | Largest | Slowest | Maximum (no escape except reset) |

Deeper positions offer higher potential payoff but require more commitment. A Charge in Fe cannot leave except via reset—it's fully committed to that Element's cycle.

### 3. Activity Level

How frequently to reposition:

| Activity | Fee Cost | Coverage |
|----------|----------|----------|
| **Patient** (few moves) | Minimal | Concentrated exposure |
| **Active** (frequent repositioning) | Cumulative speed tax | Diversified exposure |

The speed tax punishes rapid movement (up to 128× multiplier). Patience is rewarded—but patience also means concentrated risk in fewer positions.

---

## Mechanic Interactions

### Fee Asymmetry Creates Gravity

Inward movement fees use destination saturation; outward fees use source saturation. Since deeper Elements have larger curves and typically lower saturation:

- **Inward moves are usually cheaper**
- **Outward moves are usually more expensive**

This creates "gravitational pull" toward the center. Value naturally flows inward as fees accumulate in deeper pots.

**Strategic implication**: Don't fight gravity. Use it. Enter cheap, extract value at reset, re-enter cheap.

### Speed Tax Rewards Timing Over Reflexes

The 128× speed multiplier on immediate actions means:

- Bots gain no advantage from speed
- Human reaction time is irrelevant
- The ~51 second decay window is the strategic unit

**Strategic implication**: Plan moves in advance. The patient path through multiple low-saturation Elements beats a rushed direct jump.

### Saturation Creates Information

All state is public. Watching saturation levels reveals:

- How close an Element is to reset
- Where other players are concentrating
- Which pots are growing fastest

**Strategic implication**: Read the board. Saturation patterns telegraph intentions.

### Compression Reshapes the Board

Compression moves pots inward without moving Charges. This allows:

- Rescuing value from stagnant Elements
- Concentrating value for larger resets
- Manipulating which Elements become attractive

**Strategic implication**: Compression is a coordination problem—the compressor pays costs but benefits all participants in the destination. It can be worth it to create a more valuable reset.

### Vent as Costly Signal

Venting adds to pot without affecting saturation. It's pure signal:

- "This Element is worth attention"
- Attracts both allies (who want the reset) and predators (who want to trigger it)

**Strategic implication**: Vent sparingly. The signal is visible to everyone.

---

## Emergent Strategies

### The Waiting Game

**Setup**: Bind early to a promising Element at low saturation.

**Mechanics exploited**:
- Sigmoid gives large early shares
- Waiting is free (no passive costs)
- Fees only on action

**Risk**: The pot may never grow. Other players may avoid the Element. Opportunity cost of sitting idle.

**Counter**: Watch for compression incoming. If value is moving toward the Element, the wait pays off.

### The Trigger Snipe

**Setup**: Wait for an Element to approach threshold, then enter just before reset.

**Mechanics exploited**:
- Trigger gets immediate payout
- Trigger re-binds first in new cycle
- Late entry still gets proportional share

**Risk**: Someone else triggers first. Timing is visible to everyone.

**Counter**: Bundle Rebind + Overload atomically so no one can front-run.

### The Deep Raid

**Setup**: Position in a quiet deep Element (low saturation), wait for adjacent shallower Elements to saturate.

**Mechanics exploited**:
- Outward fees use source saturation (your quiet Element = cheap exit)
- Can strike into saturated pots at minimal cost
- Deep position provides cover while waiting

**Example**:
```
Carbon (Z=6) at 95% saturation: 1.41 Gluon to enter from Beryllium
Carbon (Z=6) at 95% saturation: 0.10 Gluon to enter from Nitrogen (Z=7, 2% saturation)
```

The Nitrogen position allows striking into Carbon at 14× lower cost.

**Risk**: The deep Element itself may reset while waiting. Deeper means slower cycles, but not immune.

### The Compression Pipeline

**Setup**: Compress value from outer Elements inward to create a large reset event.

**Mechanics exploited**:
- Compression fee adds to pot (self-reinforcing)
- Larger pots attract more participants
- Deep pots are harder to escape, creating commitment

**Risk**: Other players ride the compression without paying costs. Coordination problem.

**Counter**: Only compress when already well-positioned in the destination Element.

### The Edge Rotation

**Setup**: Stay in edge Elements, cycling through quick resets.

**Mechanics exploited**:
- Edge Elements reset fast (small curves)
- Exit is always available (no escape cost)
- Many small wins compound

**Risk**: Small pots mean small absolute gains. Depth-seekers capture more per cycle.

**Trade**: Volume vs magnitude. Edge rotation is lower risk, lower reward.

---

## Multiple Charges

Players can control multiple Charges. This enables:

### Diversification

Spread across different Elements to hedge timing risk. If one position stagnates, others may pay off.

### Staging

Position Charges at different depths:
- Shallow Charge for quick cycles
- Deep Charge for large payoffs
- Mid Charge for opportunistic strikes

### Scouting

Use a small Charge to test an Element's dynamics before committing larger Charges.

**Cost consideration**: Fees scale with balance. A 10 Gluon Charge pays 1/10th the fee of a 100 Gluon Charge for the same move. But it also gets 1/10th the share.

---

## Board Reading

### Saturation Patterns

| Pattern | Meaning | Action |
|---------|---------|--------|
| Single Element rising fast | Convergence point | Consider positioning |
| Multiple Elements rising | Distributed activity | Harder to predict winner |
| Deep Element rising | High-stakes event coming | Plan escape or commitment |
| Stagnant pot with no saturation | Dead value | Compression target |

### Flow Detection

Watch where fees are routing:
- Inward fees go to destination → deep pots grow
- Outward fees go to source → retreating players fund the Element they leave

Heavy outward traffic means players are escaping—but their fees fund whoever stays.

### Compression Watching

When someone compresses, the destination becomes more attractive:
- Larger pot draws attention
- Saturation hasn't changed (just pot)
- First movers after compression get large shares on the new, bigger pot

---

## The Meta-Game

### No Dominant Strategy

The tradeoffs are real:
- Early beats late on shares; late beats early on timing
- Deep beats shallow on magnitude; shallow beats deep on frequency
- Patient beats active on costs; active beats patient on coverage

Equilibrium emerges from player choices, not prescribed roles.

### Information Symmetry

All state is public. No hidden information. Edge comes from:
- Reading incentives correctly
- Predicting behavior of other players
- Timing actions relative to the ~51s decay window

### The Infinite Game

The board resets element by element, forever. There's no end state. The question is not "who wins" but "how much value can be captured over time."

Long-term success requires:
- Sustainable fee management
- Avoiding value destruction (paying more in fees than captured in resets)
- Reading the meta-game (where is activity trending?)

---

## Common Mistakes

| Mistake | Why It Fails |
|---------|--------------|
| Rushing movement | Speed tax up to 128× destroys value |
| Fighting gravity | Outward movement is expensive; use resets to escape |
| Ignoring saturation | Entry timing determines share—late entry gets marginal shares |
| Over-committing to depth | Fe is a trap if the reset doesn't come |
| Under-utilizing multiple Charges | Diversification reduces risk |
| Compression without positioning | Benefits everyone; position first |

---

## Summary

TOKAMAK64 rewards:
- **Patience**: Speed tax punishes haste; waiting is free
- **Positioning**: Be present at resets; absence captures nothing
- **Reading**: All information is public; advantage comes from interpretation
- **Timing**: Early for shares, late for triggers, always relative to saturation

The game cycles forever. Master the cycle.

---

## Next Steps

- **[Reference](REFERENCE.md)** — Exact numbers for planning
- **[Concepts](CONCEPTS.md)** — Review the mental model
- **[Operations](OPERATIONS.md)** — Detailed action guide
