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
base_fee = balance × saturation × distance² × MIN_FEE_RATE
```

| Factor | Range | Notes |
|--------|-------|-------|
| balance | Charge's Gluon | Linear |
| saturation | 0.0–1.0 | Direction-dependent |
| distance² | 1–625 | Quadratic |
| MIN_FEE_RATE | 0.001 | Constant |

**Direction:**
- **Inward** (dst.Z > src.Z): destination saturation
- **Outward** (dst.Z < src.Z): source saturation

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

100 Gluon Charge, patient movement (~51s between moves):

### Adjacent Moves (distance = 1)

| Saturation | Base Fee | With Min |
|------------|----------|----------|
| 5% | 0.005 | 0.10 |
| 25% | 0.025 | 0.10 |
| 50% | 0.050 | 0.10 |
| 75% | 0.075 | 0.10 |
| 95% | 0.095 | 0.10 |

### Long Jumps (patient, 25% saturation)

| Distance | Distance² | Fee |
|----------|-----------|-----|
| 1 | 1 | 0.10 |
| 5 | 25 | 0.63 |
| 10 | 100 | 2.50 |
| 15 | 225 | 5.63 |
| 25 | 625 | 15.63 |

### Speed Tax (adjacent, 50% saturation)

| Timing | Multiplier | Fee |
|--------|------------|-----|
| Patient (51s) | 1× | 0.10 |
| Moderate (26s) | 32× | 1.60 |
| Rushed (13s) | 72× | 3.60 |
| Immediate (0s) | 128× | 6.40 |

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| Elements | 26 | H through Fe |
| Edge Elements | 12 | H through Mg |
| Saturation threshold | 100% | Reset trigger |
| Speed tax max | 128× | Immediate action |
| Speed decay | 1024 slots | Full decay |
| Decay time (L2) | ~51s | At 50ms/slot |
| Min fee | 0.1 Gluon | Floor |
| Compression rate | 5% max | Of source pot |
| Gluon:USDC | 1:1 | Conversion |

## Commitment Share

```
share = sigmoid(saturation) × balance × depth_factor(Z)
```

Sigmoid curve:
- Inflection: 50% saturation
- Early efficiency: ~20× vs late
- Late efficiency: marginal

| Saturation | Relative Efficiency |
|------------|---------------------|
| 10% | ~18× |
| 30% | ~8× |
| 50% | ~2× |
| 70% | ~0.5× |
| 90% | ~0.1× |

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
