# Game Design

This document explains the conceptual design of TOKAMAK64: how mechanics create meaningful tension, and what emerges from simple rules.

## The Board

The game world is a static 8×8 grid (64 tiles).

### Elements

**Elements** are groups of connected tiles named after chemical elements and ordered by **atomic number (Z)**: there are 26 Elements on the board (Z=1 to Z=26), with element 0 serving as an off-board placeholder.

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

- **Edge Elements**: H (Hydrogen), He (Helium), Li (Lithium), Be (Beryllium), B (Boron), C (Carbon) — touch the board perimeter
- **Mid-depth Elements**: N, O, F, Ne, Na, Mg, Al, Si, P, S, Cl, Ar, K, Ca — progressively deeper
- **Deep Elements**: Sc, Ti, V, Cr, Mn — approaching the core
- **Core Element**: **Fe (Iron)** — maximum depth (Z=26), terminal attractor for compressed value

Each Element has fixed depth (Z). Higher Z means:
- Larger commitment curves (absorbs more saturation before reset)
- Bigger typical pots (value concentrates toward center)
- Harder to escape (require multiple Rebind steps back to edge)

**Why this matters**: Depth creates strategic gradient. Outer Elements reset quickly with smaller pots; deep Elements are slower, heavier, more consequential.

### Movement

Two Elements are **adjacent** if they share a full tile edge (not just corners). Movement is direct jump between adjacent Elements via **Rebind**.

**Directional bias**:
- **Inward** (toward higher Z, toward Fe): Cheaper base cost
- **Outward** (toward lower Z, toward H): More expensive

**Why asymmetric costs?** If inward and outward cost the same, the board would churn constantly with no natural direction. Cheap inward movement creates drift toward the center; expensive outward movement makes commitment sticky. This produces accumulation, tension, and meaningful risk.

The theme is inspired by fusion reactors where magnetic fields naturally confine plasma inward—escaping requires fighting containment.

## Charges

You don't exist on the board directly. You interact only through **Charges**—entities you control.

### Binding as Commitment

- A Charge is **bound** when occupying an Element
- A Charge is **unbound** when off the board
- You can control multiple Charges simultaneously—independent positions, independent strategies

When a Charge binds to an Element (via **Bind**), it occupies **all tiles of that Element simultaneously**. Multiple Charges can occupy the same Element without blocking each other.

**Why shared presence?** If Elements had finite capacity, early arrivals would lock out others—punishing timing skill with arbitrary cutoffs. Shared presence means positions aren't scarce; what's scarce is *when* you're present. This shifts competition from "get there first" to "be there at the right moment."

### No Neutral States

There is no spectator mode. Being on the board means being bound to exactly one Element. If you're not bound, you're not participating.

**Why strict binding?** If Charges could "hover" without commitment, saturation would be diluted and timing would lose meaning. Strict binding ensures every Charge in an Element contributes to its saturation. Every arrival matters, every departure matters. No free rides.

## Commitment and Saturation

### Measured Commitment, Not Paid

When a Charge binds to an Element, its **commitment share** is measured instantly based on:
1. **Charge's Gluon balance**
2. **Current saturation** (when you arrive on the sigmoid curve)
3. **Element depth Z** (deeper curves are larger, need more Gluon for equivalent share)

Your Gluon is not locked or transferred. It's an input to measurement, not a payment.

### The Sigmoid Curve: Early Advantage

Commitment follows a **sigmoid (logistic) curve** with inflection point at saturation 3.0 (midpoint of [0, 6] range):
- **Early binding** (low saturation): Large commitment share per Gluon (~20×+ efficiency at sat=0)
- **Mid binding** (near inflection): Diminishing returns (~2× efficiency at sat=3)
- **Late binding** (near threshold): Marginal share gain (~1× efficiency at sat=6), mostly pushes toward reset

**Why sigmoid?** Linear commitment would make late arrivals as influential as early ones—killing patience as a strategy. Exponential would lock out late arrivals entirely—making timing too brittle. Sigmoid balances: early patience is rewarded with strong shares, but late arrivals can still trigger profitable resets (even with weak shares).

### Saturation

Each Element tracks **saturation**—the real-time sum of commitment shares of all currently bound Charges.

- Saturation rises when a Charge binds (entry via Bind or Rebind)
- Saturation falls when a Charge unbinds (exit via Unbind or Rebind)
- Saturation is fully reversible—it can oscillate

When saturation crosses the Element's fixed threshold (1.0 normalized), the Element **resets**. In practice, the triggering player typically executes Overload atomically in the same transaction as the Rebind/Bind that pushes saturation over the threshold.

**Why reversible?** If saturation only increased, Elements would inevitably reset regardless of strategy—removing timing skill. Reversibility creates genuine tension: crowds can form and disperse, Charges can bluff by entering then leaving, saturation can rise and fall based on coordinated or chaotic behavior.

## Value Flow

### The Pot

Each Element maintains a single visible **pot**—accumulated shared value.

Pots grow from:
- **Movement fees** (routed to deeper Elements via directional bias)
- **Compression fees** (added to the pot being moved)
- **Voluntary contributions** (**Vent**: Charges irreversibly donate Gluon)
- **Quantum Pocket fractions** (global yield injection, see Roadmap)

**Why one shared pot?** If each Charge had a private stack, cooperation and conflict would be decoupled—you'd ignore others. A shared pot ensures everyone cares about the same object, creating aligned interests and competition simultaneously.

### Distribution at Reset

When an Element resets:
1. **Snapshot**: Pot value and bound Charges recorded
2. **Distribution**: Pot divided proportionally to each bound Charge's fixed commitment share
3. **Unbinding**: All Charges except the trigger are unbound (free repositioning)
4. **Trigger bonus**: Triggering Charge remains bound to the reset Element (first-mover advantage in new cycle)
5. **Clearing**: Pot and saturation reset to zero

**Critical rule**: Only Charges bound at the exact reset instant receive rewards. Earlier presence doesn't matter. Later arrival doesn't matter. Only being there when the threshold is crossed matters.

**Why instant-only entitlement?** This creates the core tension: repositioning between building hotspots requires guessing *when* reset occurs. Too early and you get weak shares; too late and you miss it entirely. No credit for past contributions prevents rent-seeking.

### Speed Tax

Moving quickly costs more than waiting. A **speed tax multiplier** applies based on time since your last action:
- Immediate second action: prohibitively expensive
- Waiting ~7 minutes (1024 slots): multiplier fully decays to 1

**Why speed tax?** Without it, reflexes would dominate—automation would win, deliberate play would lose. Speed tax eliminates speed as an advantage. The game rewards timing and reading incentives, not clicking fast.

## Compression

Compression allows relocating pots to Elements with higher Z.

### How Compression Works

When a Charge rebinds to an adjacent Element with higher Z via **Compress**, it can opt to **carry the pot**:
- The Charge pays a compression fee (scales with pot size and depth difference)
- Fee is added directly to the pot being moved
- Carried pot merges with destination's existing pot
- Result: strictly larger merged pot at higher Z

**Z increase only**: Compression requires dst.index > src.index. Destination can be sideways (same depth level) or skip depths—as long as Elements are adjacent and Z increases. Cost varies by depth difference, making compression routing strategic.

**Atomic with movement**: Compression completes instantly. Arrival binding may immediately trigger destination reset if saturation crosses threshold.

### Why Compression Exists

Compression solves the "stagnant pot" problem: if an Element builds a large pot but saturation stalls (Charges leave or never arrive), that value is stuck. Compression lets players:
- Rescue value from dead regions
- Push value into active hotspots
- Deliberately escalate stakes by creating massive deep pots

**But compression is escalation, not safety**: Each compression grows the pot (via fee), attracts more attention, increases competition. Compressing hoping for stability is a mistake.

### Iron as Terminal Sink

Fe (Iron) is at maximum depth (Z=26). Compressed pots cannot leave Iron via further compression—they're trapped until Iron resets.

**Why terminal?** This creates a natural attractor. All inward compression flows ultimately concentrate at Fe. Iron resets are the biggest, most consequential events—triggering global Quantum Pocket yield injection (see Roadmap).

## Voluntary Contributions

A bound Charge can irreversibly add any amount of its Gluon directly to its current Element's pot via **Vent**.

**Effects**:
- Pot increases
- Charge's commitment share does not change
- Saturation does not change

**Why contribute?** Contributions are **signals**, not investments. They broadcast: "This Element is worth sustaining/resetting." This attracts attention—sometimes allies who want the same reset, sometimes predators who compress the pot away or trigger at an unfavorable moment.

## The Closed Loop

The system requires no external input to sustain itself:

```
Charges take actions (Bind/Rebind/Compress/Unbind/Vent)
       ↓
Actions cost Gluon
       ↓
Costs become pots (routed deeper via directional fees)
       ↓
Pots attract attention
       ↓
Attention creates saturation
       ↓
Saturation triggers reset
       ↓
Reset redistributes value and ejects Charges
       ↓
Charges reposition elsewhere
```

**Why closed loop?** If costs were burned (destroyed), the game would deflate—value would drain over time. If costs stayed with the payer, friction would discourage action. By redirecting costs into pots, every action creates opportunity for future participants. This sustains perpetual cycles.

## Core Tension: Where Will the Next Reset Occur?

The fundamental question is not "should I stay or leave?" It's:

> **"Where and when will the next profitable reset occur?"**

Gameplay is profit-chasing via repositioning:
- **Early arrival** grants large commitment shares (strong yields)
- **Late arrival** gets diminishing returns but can trigger at a favorable moment
- **Compression** escalates stakes by pushing value deeper
- **Reading building saturation** and timing your repositioning is the core skill

Players constantly balance:
- Arriving early enough for good shares
- Not arriving so early that you're trapped with mediocre pots
- Guessing when reset is imminent
- Deciding whether to compress pots to reshape the board

This is **not** a stay/leave decision. It's a positioning/timing game with multiple valid strategies and no dominant approach.

## Roadmap: Global Cycle and Quantum Pocket

The following mechanics are designed but not yet fully implemented (`qpot` field is placeholder).

### Iron Reset: Global Event

When Fe (Iron) at maximum depth resets, a **global event** triggers:
- Accumulated external yield is injected into the **Quantum Pocket**—a global pool visible but initially unclaimable

### Sequential Unlock

Quantum Pocket yield is released in fixed fractions assigned by depth (shallow small, deep large; sum 100%):
- Unlocks sequentially from outermost Elements (H, He, Li...) inward
- Advance condition: reset at the current frontier depth
- On reset at an unlocked depth: that depth's fraction added to the resetting Element's pot, distributed normally

Stalled progression → larger accumulated yields for future cycles.

**Why sequential unlock?** This keeps edge Elements relevant even in late-game. If yield flooded everywhere instantly, deep play would dominate and outer regions would be ignored. Sequential unlock rewards sustained play across all depths.

## Design Principles

### Why These Rules?

1. **Transparency over mystery** — All state visible; strategy comes from reading incentives, not discovering secrets
2. **Waiting is free** — Patience is a first-class strategy, not a fallback. Costs apply only to voluntary actions.
3. **Closed economics** — Friction creates value; value creates attraction; attraction creates saturation; saturation creates resets. Self-sustaining.
4. **Directional bias** — Cheap inward, expensive outward creates natural flow. Value drifts toward center.
5. **Sigmoid rewards patience** — Early commitment is powerful without locking out late triggers.
6. **Resets are clean** — Free repositioning avoids permanent traps. Game cycles forever.
7. **Emergent complexity** — Simple rules (sigmoid curve + directional costs + shared pots) produce sophisticated decision-making and diverse strategies.

The fusion reactor theme (inward magnetic confinement, element progression H→Fe, density-triggered reactions) makes these principles intuitive but isn't required to understand the mechanics.



