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

### Phantom Shares and Overheating

Every fee paid leaves a permanent mark on the Element. This creates one of the game's most subtle and powerful dynamics.

**How It Works**:

When a Charge binds, it pushes saturation up proportional to its Gluon balance. When it unbinds, saturation drops by the same proportion. In theory, entry and exit cancel out.

**But fees change everything.**

Each action costs fees, reducing the Charge's balance. When the Charge finally exits, it removes less saturation than it added—because it has less Gluon.

**Example**: A Charge enters with 1000 Gluon, pays 50 in fees during its stay, then exits with 950. The difference—50 Gluon worth of saturation—remains behind as **phantom saturation** that doesn't belong to any Charge.

**The Accumulation**:

Every fee paid deposits phantom saturation. A single Charge churning through an Element three times:

| Round | Entry Balance | Fee (5%) | Exit Balance | Phantom Added |
|-------|---------------|----------|--------------|---------------|
| 1 | 1000 | 50 | 950 | ~1.5% sat |
| 2 | 950 | 47 | 903 | ~1.4% sat |
| 3 | 903 | 45 | 858 | ~1.3% sat |

**Total phantom from one player: ~4% saturation**

Multiple players churning can push phantom saturation to 20-30% of the reset threshold.

**Why This Matters**:

An Element resets when saturation exceeds 100%. But if 30% of that saturation is phantom:
- Only 70% represents actual committed Charges
- The Element resets at **70% effective commitment**
- Incumbents split the pot among fewer actual shares → **bonus per share**

| Saturation | Phantom | Real Shares | Effect |
|------------|---------|-------------|--------|
| 100% | 0% | 100% | Normal reset |
| 100% | 30% | 70% | Early reset, ~43% bonus per share |

**Offensive Use**: Churn through an Element to inflate phantom saturation. The attacker pays fees (which grow the pot) but accelerates the reset timeline. Competitors committed to the Element get forced into an early payout—before the pot fully develops.

**Defensive Use**: If a predator is approaching threshold, churn can trigger a premature reset. Scorched earth—take a smaller payout now rather than let them snipe it.

**The Vacuum Dilemma**:

When a large holder exits after heavy fee payments:
- Saturation drops (they removed their remaining balance)
- But phantom saturation remains elevated
- The "vacuum" is smaller than it appears
- Late entrants racing to fill it compete for diminishing room
- Speed tax punishes the rush
- The Element is closer to reset than raw saturation suggests

**Reading the Heat**:

The pot is **not** a reliable phantom indicator—compression moves pots independently of saturation. An Element can export its pot (compression out) while retaining phantom saturation, or import pots (compression in) without gaining phantom.

The true indicator is the **saturation-to-shares gap**. The game tracks two separate values for each Element:
- **Saturation**: How full the Element is (includes phantom from fee-reduced exits)
- **Shares**: The actual committed stakes of bound Charges

When these diverge, phantom has accumulated:

| Indicator | Meaning |
|-----------|---------|
| Saturation ≈ Shares | Clean Element, minimal phantom |
| Saturation > Shares | Phantom accumulated, running hot |
| Gap of 20%+ | Heavy churn history, approaching overheat |

**Strategic implication**: Compare saturation to shares, not saturation to pot. An Element at 70% saturation with only 50% shares is already 70% toward reset but will distribute rewards among only 50% worth of claimants — an extra bonus per share. Elements that run hot die young.

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

### The Overheat

**Setup**: Deliberately churn through an Element to accelerate its reset via phantom share accumulation.

**Mechanics exploited**:
- Early entry buys cheap shares
- Exit returns only marginal saturation
- The gap becomes phantom residue
- Residue pushes Element toward premature reset

**Execution**:
1. Bind early with significant capital when saturation is low
2. Wait for others to follow (saturation climbs)
3. Exit—leave phantom shares behind
4. Optionally re-enter at the new (lower) saturation for another round
5. Repeat until Element overheats

**Offensive variant**: Force a reset in an Element where a competitor is deeply committed. They get their payout early—before the pot fully develops.

**Defensive variant**: Trigger a premature reset to deny an approaching predator. Better a small payout now than getting sniped at threshold.

**Risk**: Each churn cycle costs fees. The attacker must calculate whether the board-shaping effect is worth the burn.

**Counter**: Patient holders who recognize the churn pattern can exit before the overheat, denying the attacker their intended disruption.

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

### Churn Detection

High traffic through an Element signals potential overheating. The key indicator is the **saturation-to-shares gap**:

| Signal | Meaning |
|--------|---------|
| saturation ≈ shares | Healthy Element, low churn |
| saturation > shares by 10%+ | Moderate phantom accumulation |
| saturation > shares by 30%+ | Heavily churned, approaching overheat |
| TVL low but saturation stable | Exits removed Gluon but phantom remains |

**Note**: The pot is unreliable—compression moves pots between Elements independently of phantom saturation. An Element can have a huge pot (from compression in) with no phantom, or no pot (compressed out) with heavy phantom.

**Warning sign**: An Element at 70% saturation with only 50% shares is closer to reset than one at 90% saturation with 88% shares. The gap reveals the churn history that saturation alone hides.

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
| Ignoring churn history | A churned 60% is hotter than a quiet 80% |
| Filling the vacuum blindly | Everyone sees it; speed tax punishes the race |

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
