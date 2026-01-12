# Core Concepts

The mental model: what things are, how they connect, why they matter.

## The Board

The game takes place on a fixed 8×8 grid divided into 26 **Elements**. Each Element is a contiguous region of tiles named after chemical elements, ordered by atomic number (Z):

- **Z=1 (Hydrogen)** at the outer edge
- **Z=26 (Iron)** at the center

```
    1    2    3    4    5    6    7    8
  ┌──────────────┬─────────┬────┬─────────┐
A │  H1          │  He2    │ Li3│      Be4│
  │    ┌─────────┴────┬────┤    ├────┐    │
B │    │       Al13   │Si14│    │    │    │
  ├────┴────┬─────────┤    ├────┤    │    │
C │  Mg12   │  Cr24   │    │ P15│ S16│    │
  ├────┬────┴────┬────┼────┤    │    ├────┤
D │    │  V23    │Mn25│    │    │    │ B5 │
  │    ├────┬────┼────┘    ├────┴────┤    │
E │Na11│    │    │     Fe26│     Cl17│    │
  ├────│    │    ├────┬────┴────┬────┴────┤
F │    │Ti22│Sc21│    │     Ar18│      C6 │
  │    │    ├────┤    ├─────────┴────┬────┤
G │    │    │    │Ca20│        K19   │    │
  │    └────┤    ├────┴────┬─────────┘    │
H │  Ne10   │ F9 │      O8 │           N7 │
  └─────────┴────┴─────────┴──────────────┘
```

### Depth

An Element's atomic number (Z) represents its **depth**:

| Depth | Elements | Characteristics |
|-------|----------|-----------------|
| Edge (Z=1-12) | H through Mg | Direct board access via Bind/Unbind |
| Inner (Z=13-20) | Al, Si, P, S, Cl, Ar, K, Ca | Larger pots, slower cycles |
| Deep (Z=21-25) | Sc, Ti, V, Cr, Mn | Big pots, high fees to leave |
| Core (Z=26) | Fe (Iron) | Deepest element, 7 neighbors |

Deeper Elements have **larger curves**—they absorb more value before resetting. Movement fees use the saturation of the destination curve (inward) or source curve (outward). Deeper elements typically have lower saturation, so moving through them is often cheaper.

### Adjacency

Two Elements are **adjacent** if they share a full tile edge (not just corners). Movement between Elements requires adjacency.

## Charges

Players do not exist on the board directly. They interact through **Charges**—entities they control.

### States

A Charge is always in one of two states:

| State | Meaning |
|-------|---------|
| **Bound** | On the board, occupying an Element |
| **Unbound** | Off the board, in the player's control |

A bound Charge occupies an entire Element (all its tiles at once). Multiple Charges can share the same Element—space isn't scarce, but **timing** is.

### What a Charge Holds

- **Balance**: Gluon amount (determines fee costs and commitment measurement)
- **Share**: Commitment share in the current Element (determines reward at reset)
- **Index**: Which Element and generation the Charge is bound to

## Value Flow

### The Pot

Each Element has a **pot**—accumulated value from:

1. **Movement fees** — paid when Charges enter, exit, or pass through
2. **Compression fees** — paid when pots merge inward
3. **Donations** — voluntary Vent contributions

Everyone sees the same pot. No hidden state.

### Saturation

**Saturation** is the sum of commitment shares of all bound Charges.

- Entry adds saturation
- Exit removes saturation
- It's **live**—only current Charges count, no memory of who left

When saturation crosses 100%, the Element **resets**.

### The Reset Cycle

When an Element resets:

1. **Distribution** — Pot splits proportionally by share
2. **Ejection** — All Charges except the trigger unbind (free)
3. **First-mover** — Trigger stays bound, first in the new cycle
4. **Clear** — Pot and saturation go to zero
5. **Generation** — Element's generation counter ticks up

Then the cycle starts over. Forever.

## Commitment

### The Sigmoid Curve

When a Charge binds, its **commitment share** depends on:

1. **Gluon balance** — bigger Charge, bigger share
2. **Current saturation** — the position on the curve
3. **Element depth** — deeper = larger curve

The curve is a **sigmoid**:

```
Share
  │
  │                    ┌─────── Late: marginal share
  │               ╱────┘
  │          ╱────
  │     ╱────
  │ ────           ← Inflection point (50% saturation)
  │
  └────────────────────────── Saturation
       Early: large share
```

- **Early** (low saturation): More efficient (~12× at 10% vs 90% saturation; theoretical max: 20× at 0%)
- **Mid** (near inflection): Diminishing returns
- **Late** (near threshold): Marginal share; either (a) calculate to cross 100% and trigger immediately, OR (b) accept small share + free ejection

Early birds get more shares. Late arrivals choose: trigger reset for first position (must do so immediately), or ride for free ejection and profit from large pot. Strategic note: Anyone can call overload once saturation >= 100%, so crossing without triggering risks losing the reward.

### Measured, Not Staked

Gluon isn't locked. A Charge's balance determines its share, but the Gluon remains available.

- Bigger Charges get bigger shares
- Charges can rebind elsewhere with full balance
- No staking, no lockups

## Fees

Every action costs fees. Fees are **never burned**—they go into pots and become future rewards.

### What Affects Fees

| Factor | Effect | Implication |
|--------|--------|-------------|
| **Distance²** | Quadratic scaling | Long jumps are exponentially expensive |
| **Saturation** | Linear scaling | Crowded Elements cost more to enter/exit |
| **Speed tax** | Up to 128× multiplier | Rapid actions are punished |
| **Balance** | Linear scaling | Larger Charges pay proportionally more |

### Movement: All One Formula

**Bind, Unbind, and Rebind all use the same fee calculation:**

```
fee = balance × distance² × saturation / DENOMINATOR

distance = atomic number difference:
- Bind (onboard): Z=0 → destination.Z
- Unbind (offboard): source.Z → Z=0
- Rebind (board→board): |destination.Z - source.Z|
```

No special "entry" or "exit" fees—cost depends only on distance moved and curve saturation. Moving from edge (H, Z=1) to He (Z=2) costs the same whether you're binding on or rebinding between them.

### Fee Direction

Movement fees route to Element pots based on direction:

| Direction | Fee Based On | Pot Receives |
|-----------|--------------|--------------|
| **Inward** (toward Fe) | Destination saturation | Destination pot |
| **Outward** (toward edge) | Source saturation | Source pot |

Deeper elements usually have lower saturation (larger curves). Value flows toward the center through low-saturation paths.

### Speed Tax

A multiplier (1× to 128×) based on time since the Charge's last action:

- **Immediate**: 128× multiplier
- **After ~51 seconds**: 1× (full decay)

Speed is not an advantage. Timing beats reflexes.

## The Closed Loop

Value circulates:

```
Actions cost Gluon → Costs fill pots → Pots attract Charges →
Charges build saturation → Saturation triggers reset →
Reset distributes pot → Charges reposition → (repeat)
```

Nothing burns. Every fee becomes someone else's reward.

## Invariants

Rules that never change:

| Rule | What it means |
|------|---------------|
| Waiting is free | No rent, no decay, no passive drain |
| Pay only on action | Do nothing, pay nothing |
| No burning | All fees become rewards |
| Binary binding | Bound to one Element or off the board |
| Live saturation | Only current Charges count |
| Entry triggers reset | Never exit |
| Free ejection | Reset kicks everyone out for free; saves unbind fees |
| **Full transparency** | All state visible: pots, saturation, positions |
| **Deterministic** | Calculate exactly: fees, shares, threshold crosses |

Players can predict every outcome before committing. No randomness, no guessing.

## Next Steps

- **[Operations](OPERATIONS.md)** — How to perform each action
- **[Strategy](STRATEGY.md)** — How mechanics interact
- **[Reference](REFERENCE.md)** — Fees, formulas, board map
