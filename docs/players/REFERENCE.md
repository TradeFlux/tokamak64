# Reference

Fees, formulas, board layout, constants.

## Board Layout

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

## Element Table

| Z | Element | Tiles | Edge? | Adjacent To |
| --- | --- | --- | --- | --- |
| 1 | H | A1, A2, B1, C1 | Yes | He(2), Mg(12), Al(13) |
| 2 | He | D1, E1 | Yes | H(1), Li(3), Al(13), Si(14) |
| 3 | Li | F1, F2 | Yes | He(2), Be(4), Si(14), P(15), S(16) |
| 4 | Be | G1, H1, H2, H3 | Yes | Li(3), B(5), S(16) |
| 5 | B | H4, H5 | Yes | Be(4), C(6), S(16), Cl(17) |
| 6 | C | G6, H6 | Yes | B(5), N(7), Cl(17), Ar(18), K(19) |
| 7 | N | F8, G8, H7, H8 | Yes | C(6), O(8), K(19) |
| 8 | O | D8, E8 | Yes | N(7), F(9), K(19), Ca(20) |
| 9 | F | C7, C8 | Yes | O(8), Ne(10), Ca(20), Sc(21), Ti(22) |
| 10 | Ne | A6, A7, A8, B8 | Yes | F(9), Na(11), Ti(22) |
| 11 | Na | A4, A5 | Yes | Ne(10), Mg(12), Ti(22), V(23) |
| 12 | Mg | A3, B3 | Yes | H(1), Na(11), Al(13), V(23), Cr(24) |
| 13 | Al | B2, C2, D2 | No | H(1), He(2), Mg(12), Si(14), Cr(24) |
| 14 | Si | E2, E3 | No | He(2), Li(3), Al(13), P(15), Cr(24), Fe(26) |
| 15 | P | F3, F4 | No | Li(3), Si(14), S(16), Cl(17), Fe(26) |
| 16 | S | G2, G3, G4 | No | Li(3), Be(4), B(5), P(15), Cl(17) |
| 17 | Cl | F5, G5 | No | B(5), C(6), P(15), S(16), Ar(18), Fe(26) |
| 18 | Ar | E6, F6 | No | C(6), Cl(17), K(19), Ca(20), Fe(26) |
| 19 | K | E7, F7, G7 | No | C(6), N(7), O(8), Ar(18), Ca(20) |
| 20 | Ca | D6, D7 | No | O(8), F(9), Ar(18), K(19), Sc(21), Fe(26) |
| 21 | Sc | C5, C6 | No | F(9), Ca(20), Ti(22), V(23), Fe(26) |
| 22 | Ti | B5, B6, B7 | No | F(9), Ne(10), Na(11), Sc(21), V(23) |
| 23 | V | B4, C4 | No | Na(11), Mg(12), Sc(21), Ti(22), Cr(24), Mn(25) |
| 24 | Cr | C3, D3 | No | Mg(12), Al(13), Si(14), V(23), Mn(25) |
| 25 | Mn | D4 | No | V(23), Cr(24), Fe(26) |
| 26 | Fe | E4, D5, E5 | No | Si(14), P(15), Cl(17), Ar(18), Ca(20), Sc(21), Mn(25) |


## Edge Elements

Entry/exit points: **H, He, Li, Be, B, C, N, O, F, Ne, Na, Mg** (Z=1–12)

Only these touch the board perimeter. Bind and Unbind require edge.

## Fee Formulas

### Base Structure

```
total_fee = base_fee × speed_multiplier
```

### Movement Fee (Rebind)

```
base_fee = balance × distance² × saturation / DENOMINATOR

DENOMINATOR = MAX_ATOMIC_NUMBER² × MAX_SATURATION = 68,048,388,096
```

| Factor | Range | Notes |
|--------|-------|-------|
| balance | Charge's Gluon | Linear |
| saturation | 0–MAX_SATURATION (100,663,296) | Direction-dependent |
| distance² | 1–169 | Atomic number difference, quadratic |
| MAX_ATOMIC_NUMBER | 26 | H through Fe |
| MAX_SATURATION | 100,663,296 | Q8.24 fixed-point |

**Direction:**
- **Inward** (dst.Z > src.Z): destination saturation
- **Outward** (dst.Z < src.Z): source saturation

**Unified Fee Structure:**
All movement fees use the same formula:
- **Bind** (onboard): distance = dst.Z (treating offboard as Z=0)
- **Unbind** (offboard): distance = src.Z (moving to Z=0)
- **Rebind** (board→board): distance = |dst.Z - src.Z|

There's no special "entry/exit" fee—movement cost depends only on atomic number distance and curve saturation.

### Speed Multiplier

```
multiplier = 128 × (1 - decay)²

decay = min(slots_since_last_action / 1024, 1)
```

| Slots | Time (~L2) | Multiplier |
|-------|------------|------------|
| 0 | 0s | 128× |
| 256 | ~13s | 72× |
| 512 | ~26s | 32× |
| 768 | ~38s | 8× |
| 1024+ | ~51s+ | 1× |

Full decay: 1024 slots ≈ 51 seconds (at 50ms/slot)

### Compression Fee

```
compression_fee = source_pot × compression_rate × saturation

compression_rate = 0.05 (5% max)
```

Scales 0% (empty) to 5% (fully saturated).

### Minimum Fee

```
MIN_FEE = 0.1 Gluon
```

## Fee Examples

**Inputs:** 10,000 Gluon charge, patient timing (51s decay = 1× multiplier)

### Adjacent Moves (distance = 1)

| Saturation | Fee |
|:----------:|:---:|
| 5% | 0.74 |
| 25% | 3.70 |
| 50% | 7.40 |
| 75% | 11.09 |
| 95% | 14.05 |

### Key Move Examples (25% saturation)

| Move | Distance | Fee | Note |
|:----:|:--------:|:---:|------|
| H→He | 1 | 3.70 | Adjacent edge |
| H→Mg | 11 | 447.49 | Edge-to-edge |
| H→Al | 12 | 532.54 | Edge-to-inner |
| **F→Ti** | **13** | **625.00** | Max distance |

### Speed Tax (H→He, 50% saturation)

| Timing | Multiplier | Fee |
|--------|:----------:|:---:|
| Patient (51s) | 1× | 7.40 |
| Moderate (26s) | 32× | 236.69 |
| Rushed (13s) | 72× | 532.54 |
| Immediate (0s) | 128× | 946.75 |

### Max Distance (F→Ti, patient)

| Saturation | Fee |
|:----------:|:---:|
| 25% | 625.00 |
| 50% | 1,250.00 |
| 95% | 2,375.00 |

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| Elements | 26 | H through Fe |
| Edge Elements | 12 | H through Mg |
| Max distance | 13 | F→Ti (atomic number difference) |
| Fee denominator | 68,048,388,096 | MAX_ATOMIC_NUMBER² × MAX_SATURATION |
| Saturation threshold | 100% | Reset trigger |
| Speed tax max | 128× | Immediate action |
| Speed decay | 1024 slots | Full decay |
| Decay time (L2) | ~51s | At 50ms/slot |
| Min fee | 0.1 Gluon | Floor |
| Compression rate | 5% max | Of source pot |
| Gluon:USDC | 1:1 | Conversion |

## Commitment Share

```
share = dx_for_dc(saturation, pressure, balance, capacity)
```

The share is the x-increment (dx) earned for a given balance deposit, calculated via the sigmoid curve integral (softplus).

**Efficiency** (dx per balance) follows `1/sigmoid(x)`:
- Higher at low saturation (early = more efficient)
- Lower at high saturation (late = less efficient)
- Inflection point at 50% saturation

| Saturation | x value | sigmoid(x) | Relative Efficiency |
|------------|---------|------------|---------------------|
| 10% | -2.4 | 0.083 | 12.0× |
| 30% | -1.2 | 0.231 | 4.3× |
| 50% | 0.0 | 0.500 | 2.0× (baseline) |
| 70% | +1.2 | 0.769 | 1.3× |
| 90% | +2.4 | 0.917 | 1.1× |

**Early vs late**: Binding at 10% saturation gives ~12× more shares per Gluon than binding at 90%.

## Quick Card

### Entry
```
USDC → Infuse → Wallet → Charge → Bind → Board
```

### Exit
```
Board → Unbind → Discharge → Wallet → Extract → USDC
```

### Fee Direction
- Inward → destination pot
- Outward → source pot

### Key Timings
- Speed decay: ~51s to 1×
- Optimal spacing: 51+ seconds

### Key Rates
- Min fee: 0.1 Gluon
- Compression: 5% max
- Speed tax: 128× max

---

## See Also

- **[Concepts](CONCEPTS.md)** — mental model
- **[Operations](OPERATIONS.md)** — actions
- **[Strategy](STRATEGY.md)** — using these numbers
- **[Glossary](../GLOSSARY.md)** — terms
