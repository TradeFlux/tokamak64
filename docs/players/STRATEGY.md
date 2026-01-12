# Strategy Guide

How game mechanics create strategic choices.

## Strategic Implications

These are mathematical consequences of how the game works.

### Early Binding Advantage

The sigmoid curve determines commitment share based on:

1. Gluon balance at binding
2. Current saturation level
3. Element's capacity (curve size)

**Binding earlier gives more shares per Gluon:**

| Saturation | Relative Efficiency vs 90% |
|------------|----------------------------|
| 10% | 12.0× |
| 30% | 4.3× |
| 50% | 2.0× |
| 70% | 1.3× |
| 90% | 1.0× (baseline) |

Large Gluon balance can trigger reset from any saturation level. The curve doesn't have arbitrary cutoffs—more balance means more share, regardless of when binding occurs.

**See:** [REFERENCE.md](REFERENCE.md#commitment-share) for exact table

---

### Speed Tax

Movement fees multiply based on time since last action:

| Timing | Multiplier |
|--------|-----------|
| Immediate (0s) | 128× |
| Rushed (13s) | 72× |
| Moderate (26s) | 32× |
| Patient (51s+) | 1× |

Waiting 51 seconds between moves reduces fees by up to 99.2%.

**See:** [REFERENCE.md](REFERENCE.md#speed-multiplier)

---

### Fee Direction

Fees route to Element pots based on movement direction:

- **Inward** (higher Z): Destination pot receives fee, uses destination saturation
- **Outward** (lower Z): Source pot receives fee, uses source saturation

Deeper elements typically have lower saturation due to larger curves. Moving through deep elements is often cheaper.

---

### Phantom Saturation

Fees reduce balance, but exit removes less saturation than entry added.

Example:
- Enter with 1,000 Gluon
- Pay 50 Gluon in fees
- Exit with 950 Gluon
- Exit removes less saturation than entry added

The gap becomes "phantom saturation"—saturation that belongs to no Charge.

**Effect:** When Element resets, pot splits among fewer claimants. High phantom means better reward per share for remaining Charges.

**Detect:** Compare saturation to total shares. Large gap indicates high phantom.

---

### Depth Tradeoff

Deeper elements have larger curves but reduced positioning flexibility:

| Depth | Pot Capacity | Positioning |
|-------|-------------|-------------|
| Edge (Z=1-12) | Small | Direct Bind/Unbind (board entry/exit) |
| Inner (Z=13-20) | Medium | Moderate flexibility |
| Deep (Z=21-25) | Large | Limited (fewer neighbors) |
| Core (Z=26) | Largest | Isolated (all 7 neighbors are deep) |

Repositioning cost depends on `distance² × path saturation`. Moving Fe → Mn (distance 1) is cheap regardless of depth.

---

### Reset Mechanics

When saturation crosses 100%, any bound Charge can call Overload:

1. **Trigger:** First to call Overload receives proportional reward + first position in new cycle
2. **Others:** Ejected for free (no unbind fee), receive proportional reward via Claim

**Critical:** Once saturation >= 100%, anyone can trigger. If binding pushes past 100% without triggering, another Charge can capture the trigger reward.

**Choice:** Bind to trigger reset, or bind for share + free ejection. Both valid—depends on pot size and calculation.

---

## Common Mistakes

| Mistake | Why It Fails |
|---------|--------------|
| Rushing moves | Speed tax up to 128× increases cost |
| Fighting gravity | Outward through saturated elements is expensive; route through quiet spots |
| Crossing 100% without triggering | Anyone can call Overload and capture the reward |
| Over-committing deep | Positioning limited if reset doesn't come; calculate whether pot justifies |
| Racing to fill vacuum | Everyone sees it; phantom remains; speed tax applies |

---

## Information

All state is visible: pots, saturation, positions, fees.

All outcomes are calculable: exact share for any bind, exact fee for any move, exact reward at reset.

No randomness. No hidden information.

---

## Summary

TOKAMAK64 rewards:
- **Patience** — Speed tax decreases with waiting
- **Positioning** — Presence at resets captures value
- **Calculation** — All outcomes predictable before acting
- **Reading** — All information public; edge is correct interpretation

---

## Next

- **[Reference](REFERENCE.md)** — Exact numbers for calculation
- **[Concepts](CONCEPTS.md)** — How mechanics work
- **[Operations](OPERATIONS.md)** — How to execute actions
