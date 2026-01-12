# Strategy Guide

How mechanics interact. What strategies emerge.

## The Core Question

Every decision comes down to:

> **Where and when will the next profitable reset happen?**

No safe spots. Value comes from being in the right place at the right time.

---

## Three Tradeoffs

### 1. Entry Timing

When to bind relative to saturation:

| Timing | Share | Risk |
|--------|-------|------|
| **Early** (low saturation) | Large | Pot might stagnate; opportunity cost |
| **Mid** (building) | Moderate | Balanced |
| **Late** (near threshold) | Small | Pot may not justify marginal share; OR free ejection bonus |

The sigmoid makes this real:
- Early: ~12× more efficient than late (at 10% vs 90% saturation; theoretical max: 20× at 0%)
- Late: marginal share, but two paths: (a) calculate to cross 100% and trigger immediately, OR (b) bind for large pot + free ejection

Everything is visible and calculable. Players know exactly: share size, fee cost, reset outcome. Note: Anyone can call overload once saturation >= 100%, so crossing the threshold without triggering creates risk of someone else capturing the reward.

### 2. Depth Selection

Where to sit on the edge-to-core gradient:

| Depth | Pot Size | Cycle Speed | Exit Cost |
|-------|----------|-------------|-----------|
| **Edge** (H–Mg) | Small | Fast | Low (direct unbind) |
| **Inner** (Al–Ca) | Medium | Moderate | Medium (rebind out) |
| **Deep** (Sc–Mn) | Large | Slow | High (many rebinds) |
| **Core** (Fe) | Largest | Slowest | Highest (7 hops to edge) |

Deeper = bigger potential payout, but more fees to leave. Fe has 7 neighbors and high adjacency—Charges can always rebind outward, it just costs.

### 3. Activity Level

How often to move:

| Activity | Fee Cost | Exposure |
|----------|----------|----------|
| **Patient** (few moves) | Minimal | Concentrated |
| **Active** (repositioning) | Speed tax adds up | Diversified |

Speed tax punishes rapid movement (up to 128×). Patience pays—but also means eggs in fewer baskets.

---

## Mechanic Interactions

### Fee Asymmetry = Gravity

Inward fees use destination saturation. Outward fees use source saturation. Deeper elements typically have larger curves with lower saturation.

**Strategy implications:**
- Moving through deep/quiet elements is cheaper
- Escaping from crowded elements is expensive
- Routes through low-saturation elements minimize fees

Value flows toward the center through low-saturation paths. Don't fight it—use the gradient.

### Speed Tax = Timing Over Speed

128× multiplier on immediate actions. Bots gain nothing from speed. Human reaction time doesn't matter. The ~51 second decay window serves as a strategic planning horizon.

Plan ahead. The patient path through low-saturation Elements beats a rushed direct jump.

### Public State = Information Game

Everything is visible and deterministic: pots, saturation, positions, fees.

**Players can calculate exactly:**
- How much saturation a bind will add (based on balance and current saturation)
- Whether the bind will cross 100% and trigger reset
- What the share will be at any saturation level
- Exact fees for any move (before committing)

No randomness, no hidden information. Watch saturation to see:
- How close an Element is to reset
- Where players are concentrating
- Which pots are growing

Read the board. Calculate precisely. Act with certainty.

### Compression Reshapes Value

Compression moves pots inward without moving Charges.

Use it to:
- Rescue value from dead Elements
- Concentrate value for bigger resets
- Make specific Elements attractive

Compression is a coordination problem—the initiator pays, everyone benefits. Only worth it if positioned in the destination.

### Vent = Signal

Venting adds to pot without changing saturation. Pure signal: "this Element is worth attention."

Attracts both allies (who want the reset) and predators (who want to trigger early).

Use sparingly. Everyone sees it.

### Phantom Saturation

Fees create saturation that belongs to no one.

Example: Charge enters with 1000 Gluon, pays 50 in fees, exits with 950. The exit removes less saturation than entry added. That 50 Gluon gap is phantom.

Why it matters: Element resets at 100%. If 30% is phantom, only 70% is real Charges—pot splits among fewer claimants (~43% bonus per share).

Watch the saturation-to-shares gap. When saturation exceeds shares, phantom has built up. 20%+ gap = heavy churn.

| Saturation | Shares | Meaning |
|------------|--------|---------|
| 70% | 68% | Clean, minimal phantom |
| 70% | 50% | Hot, 20% phantom |

Use churn offensively to force early resets on competitors. Or defensively to deny predators. The churner pays fees but shapes the board.

Don't race to fill a "vacuum" after a large exit—phantom remains, room is smaller than it looks, and speed tax punishes the rush.

---

## Strategies

### The Wait

**Setup**: Bind early at low saturation.

**Why it works**:
- Sigmoid gives big early shares
- Waiting is free
- No passive costs

**Risk**: Pot stagnates. Others avoid the Element. Opportunity cost.

**Watch for**: Compression incoming. If value flows your way, patience pays.

### The Snipe

**Setup**: Wait for an Element to near threshold, then enter to trigger.

**Why it works**:
- Trigger gets immediate payout
- Trigger re-binds first in new cycle
- Late entry still gets proportional share

**Risk**: Someone else triggers first. Timing is visible.

**Counter**: Bundle Rebind + Overload atomically. No front-running.

### The Deep Raid

**Setup**: Sit in a quiet deep Element. Wait for adjacent shallower Elements to saturate.

**Why it works**:
- Outward fees use source saturation (quiet spot = cheap exit)
- Strike into saturated pots cheaply
- Deep position provides cover

**Example**:
```
Carbon (Z=6) at 95%: 1.41 Gluon to enter from Beryllium
Carbon (Z=6) at 95%: 0.10 Gluon to enter from Nitrogen (Z=7, 2%)
```

Nitrogen position = 14× cheaper strike into Carbon.

**Risk**: Deep Element resets while you wait. Slower cycles, not immune.

### The Pipeline

**Setup**: Compress value from outer Elements inward to create a big reset.

**Why it works**:
- Compression fee adds to pot (self-reinforcing)
- Bigger pots attract more players
- Deep pots have high exit costs, creating commitment

**Risk**: Others ride your compression without paying. Coordination problem.

**Counter**: Only compress when already positioned in destination.

### The Edge Rotation

**Setup**: Stay at edge Elements, cycle through quick resets.

**Why it works**:
- Edge resets fast (small curves)
- Exit always available
- Many small wins compound

**Risk**: Small pots = small absolute gains. Depth-seekers capture more per cycle.

**Trade**: Volume vs magnitude. Lower risk, lower reward.

---

## Multiple Charges

You can run multiple Charges. Use them for:

### Diversification

Spread across Elements. If one stagnates, others pay.

### Staging
- Edge Charge for quick cycles
- Deep Charge for big payoffs
- Mid Charge for opportunistic strikes

### Charge Size Strategy

Different sizes serve different purposes:

| Small Charge | Medium Charge | Large Charge |
|--------------|---------------|--------------|
| Low commitment | Balanced risk/reward | Maximum share |
| Minimal fees | Moderate fees | Higher fees |
| Quick repositioning | Flexible positioning | Strategic depth |

---

## Board Analysis

### Saturation Patterns

| Pattern | Meaning | Action |
|---------|---------|--------|
| Single Element rising fast | Convergence | Consider positioning |
| Multiple rising | Distributed activity | Harder to predict |
| Deep Element rising | Big event coming | Commit or escape |
| Stagnant pot, no saturation | Dead value | Compression target |

### Flow Detection

Watch where fees route:
- Inward → destination pot grows
- Outward → retreating players fund what they leave

Heavy outward traffic = players escaping. Their fees fund whoever stays.

### Post-Compression

After compression, destination becomes attractive:
- Bigger pot
- Saturation unchanged
- First movers get big shares on the new pot

### Churn Detection

Saturation-to-shares gap. 20%+ means heavy churn—hotter than saturation suggests.

---

## The Meta

### No Dominant Strategy

Tradeoffs are real:
- Early beats late on shares; late beats early on timing
- Deep beats shallow on size; shallow beats deep on frequency
- Patient beats active on costs; active beats patient on coverage

Equilibrium comes from player choices, not preset roles.

### Information Symmetry

All state is public. No hidden info. Edge comes from:
- Reading incentives correctly
- Predicting other players
- Timing around the ~51s decay window

### Infinite Game

No end state. The board resets Element by Element, forever. Question isn't "who wins" but "how much value players can capture over time."

Long-term success:
- Sustainable fee management
- Avoid value destruction (fees > rewards)
- Read where activity is trending

---

## Common Mistakes

| Mistake | Why it fails |
|---------|--------------|
| Rushing | Speed tax up to 128× destroys value |
| Fighting gravity | Outward through saturated elements is expensive; use resets or route through quiet spots |
| Ignoring saturation | Entry timing = share size |
| Crossing 100% without triggering | Anyone can call overload and capture the reward; free lunch rarely left on table |
| Over-committing deep | Big exit costs if reset doesn't come |
| Single Charge only | No diversification |
| Compression without position | The initiator pays, everyone benefits |
| Ignoring churn | Churned 60% is hotter than quiet 80% |
| Racing to fill vacuum | Everyone sees it; speed tax punishes |

---

## Summary

TOKAMAK64 rewards:
- **Patience** — speed tax punishes haste; waiting is free
- **Positioning** — be present at resets; absence gets nothing
- **Reading** — all info is public; edge is interpretation
- **Timing** — early for shares; calculate precisely to cross 100% for reset trigger

The game cycles forever. Master the cycle.

---

## Next

- **[Reference](REFERENCE.md)** — exact numbers
- **[Concepts](CONCEPTS.md)** — mental model
- **[Operations](OPERATIONS.md)** — action details
