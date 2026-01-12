# Reference

Quick lookup for fees, formulas, board layout, and constants.

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
|---|---------|-------|-------|-------------|
| 1 | H | A1, A2, B1, C1 | Yes | Al(13), Mg(12), Na(11) |
| 2 | He | D1, E1 | Yes | Na(11), Ti(22) |
| 3 | Li | F1, F2 | Yes | Ti(22), Sc(21), Ne(10) |
| 4 | Be | G1, H1, H2, H3 | Yes | Ne(10), F(9) |
| 5 | B | H4, H5 | Yes | F(9), O(8) |
| 6 | C | G6, H6 | Yes | O(8), K(19), Ar(18), Cl(17) |
| 7 | N | F8, G8, H7, H8 | Yes | Cl(17), Ar(18) |
| 8 | O | D8, E8 | Yes | B(5), N(7), Cl(17), K(19), Ca(20) |
| 9 | F | C7, C8 | Yes | Be(4), Ne(10), Sc(21), Ca(20) |
| 10 | Ne | A6, A7, A8, B8 | Yes | Li(3), Be(4), F(9), Sc(21), Ti(22) |
| 11 | Na | A4, A5 | Yes | H(1), He(2), Mg(12), Ti(22), V(23) |
| 12 | Mg | A3, B3 | Yes | H(1), Na(11), Al(13), V(23), Cr(24) |
| 13 | Al | B2, C2, D2 | No | H(1), Mg(12), Si(14), Cr(24) |
| 14 | Si | E2, E3 | No | Al(13), P(15), Cr(24), Mn(25) |
| 15 | P | F3, F4 | No | Si(14), S(16), Mn(25), Fe(26) |
| 16 | S | G2, G3, G4 | No | P(15), Cl(17), Fe(26) |
| 17 | Cl | F5, G5 | No | C(6), N(7), O(8), S(16), Ar(18), Fe(26) |
| 18 | Ar | E6, F6 | No | C(6), N(7), Cl(17), K(19), Ca(20), Fe(26) |
| 19 | K | E7, F7, G7 | No | C(6), Ar(18), Ca(20) |
| 20 | Ca | D6, D7 | No | O(8), F(9), Ar(18), K(19), Sc(21), Fe(26) |
| 21 | Sc | C5, C6 | No | Li(3), Ne(10), F(9), Ti(22), Ca(20), Fe(26) |
| 22 | Ti | B5, B6, B7 | No | He(2), Li(3), Ne(10), Na(11), V(23), Sc(21) |
| 23 | V | B4, C4 | No | Na(11), Mg(12), Ti(22), Cr(24), Mn(25) |
| 24 | Cr | C3, D3 | No | Mg(12), Al(13), Si(14), V(23), Mn(25) |
| 25 | Mn | D4 | No | Si(14), P(15), V(23), Cr(24), Fe(26) |
| 26 | Fe | E4, D5, E5 | No | P(15), S(16), Cl(17), Ar(18), Ca(20), Sc(21), Mn(25) |

## Edge Elements

Entry and exit points for the board:

**H (1), He (2), Li (3), Be (4), B (5), C (6), N (7), O (8), F (9), Ne (10), Na (11), Mg (12)**

Only these 12 Elements touch the board perimeter and allow Bind/Unbind.

## Fee Formulas

### Base Fee Structure

```
total_fee = base_fee × speed_multiplier
```

Where:
- `base_fee` depends on operation type
- `speed_multiplier` ranges from 1× to 128×

### Movement Fee (Rebind)

```
base_fee = balance × saturation × distance² × MIN_FEE_RATE
```

| Factor | Range | Notes |
|--------|-------|-------|
| balance | Charge's Gluon | Linear scaling |
| saturation | 0.0 – 1.0 | Based on direction (see below) |
| distance² | 1 – 625 | Quadratic scaling |
| MIN_FEE_RATE | 0.001 | Base rate constant |

**Direction determines saturation source:**
- **Inward** (dst.Z > src.Z): Use destination saturation
- **Outward** (dst.Z < src.Z): Use source saturation

### Speed Multiplier

```
multiplier = 128 × (1 - decay)²

where decay = min(slots_since_last_action / 1024, 1)
```

| Slots Since Action | Time (~L2) | Multiplier |
|-------------------|------------|------------|
| 0 | 0s | 128× |
| 256 | ~13s | 72× |
| 512 | ~26s | 32× |
| 768 | ~38s | 8× |
| 1024+ | ~51s+ | 1× |

**Full decay time**: 1024 slots ≈ 51 seconds (at 50ms/slot on L2)

### Compression Fee

```
compression_fee = source_pot × compression_rate × saturation

where compression_rate = 0.05 (5% max)
```

Compression fee scales from 0% (empty Element) to 5% (fully saturated).

### Minimum Fee

All fees have a floor:

```
MIN_FEE = 0.1 Gluon
```

## Fee Examples

Assuming 100 Gluon Charge, patient movement (~51s between moves):

### Adjacent Moves (distance = 1)

| Saturation | Base Fee | With Min Fee |
|------------|----------|--------------|
| 5% | 0.005 Gluon | 0.10 Gluon |
| 25% | 0.025 Gluon | 0.10 Gluon |
| 50% | 0.050 Gluon | 0.10 Gluon |
| 75% | 0.075 Gluon | 0.10 Gluon |
| 95% | 0.095 Gluon | 0.10 Gluon |

### Long Jumps (patient, 25% saturation)

| Distance | Distance² | Fee |
|----------|-----------|-----|
| 1 | 1 | 0.10 Gluon |
| 5 | 25 | 0.63 Gluon |
| 10 | 100 | 2.50 Gluon |
| 15 | 225 | 5.63 Gluon |
| 25 | 625 | 15.63 Gluon |

### Speed Tax Impact (adjacent move, 50% saturation)

| Timing | Multiplier | Fee |
|--------|------------|-----|
| Patient (51s) | 1× | 0.10 Gluon |
| Moderate (26s) | 32× | 1.60 Gluon |
| Rushed (13s) | 72× | 3.60 Gluon |
| Immediate (0s) | 128× | 6.40 Gluon |

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| Elements | 26 | Total on board (Z=1 to Z=26) |
| Edge Elements | 12 | Allow Bind/Unbind (Z=1 to Z=12) |
| Saturation threshold | 100% | Reset trigger point |
| Speed tax max | 128× | Immediate action multiplier |
| Speed decay | 1024 slots | Full decay period |
| Decay time (L2) | ~51 seconds | At 50ms/slot |
| Min fee | 0.1 Gluon | Fee floor |
| Compression rate | 5% max | Of source pot |
| Gluon:USDC | 1:1 | Conversion rate |

## Commitment Share

Share measured at binding:

```
share = sigmoid(saturation) × balance × depth_factor(Z)
```

The sigmoid curve:
- **Inflection point**: 50% saturation (x = 3.0 internal)
- **Early efficiency**: ~20× vs late binding
- **Late efficiency**: Marginal share increase

| Saturation | Relative Share Efficiency |
|------------|--------------------------|
| 10% | ~18× |
| 30% | ~8× |
| 50% | ~2× |
| 70% | ~0.5× |
| 90% | ~0.1× |

## Quick Reference Card

### Entry Flow
```
USDC → Infuse → Wallet → Charge → Bind → Board
```

### Exit Flow
```
Board → Unbind → Discharge → Wallet → Extract → USDC
```

### Fee Direction
- **Inward**: Fee → Destination pot
- **Outward**: Fee → Source pot

### Key Timings
- **Speed tax decay**: ~51 seconds to 1×
- **Optimal move spacing**: 51+ seconds between moves

### Key Rates
- **Min fee**: 0.1 Gluon
- **Compression max**: 5% of pot
- **Speed tax max**: 128×

---

## See Also

- **[Concepts](CONCEPTS.md)** — Mental model
- **[Operations](OPERATIONS.md)** — Action details
- **[Strategy](STRATEGY.md)** — How to use this information
- **[Glossary](../GLOSSARY.md)** — Term definitions
