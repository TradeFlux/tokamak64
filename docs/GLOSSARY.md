# Glossary

All TOKAMAK64 terms.

**See also:** [players/README.md](players/README.md) for documentation index.

---

## A

### Adjacent
Two Elements sharing a full tile edge (not corners). Rebind requires adjacency.

### Artefact
Snapshot of an Element at reset. Stores pot, index, shares. Non-triggering Charges use it to Claim.

### Atomic Number (Z)
Element identifier (1–26). Higher = deeper. H=1, He=2, ... Fe=26.

### ATA (Associated Token Account)
Solana token account for USDC/USDT. Entry/exit point for real money.

---

## B

### Balance
Gluon held by a Wallet or Charge.

### Bind
Operation placing an unbound Charge on the board. Edge Elements only. Fee calculated as distance from Z=0 (offboard) to destination element.

### Board
The 8×8 grid with 26 Elements. The playing field.

### Bound
Charge state: on the board, occupying an Element. Participates in saturation, earns at reset.

---

## C

### Charge
Player-controlled entity on the board. Has Gluon balance, binds to Elements, earns shares.

### Churn
Repeated entry/exit. Each exit leaves phantom saturation behind.

### Claim
Collect reward from a reset Artefact. For Charges that didn't trigger.

### Commitment Share
Stake in an Element. Sigmoid curve determines it at binding. Bigger share = bigger reward.

### Compress
Move an Element's pot to an adjacent deeper Element. dst.index > src.index.

### Compression Fee
Up to 5% of pot, paid when compressing. Goes to destination.

### Curve
Sigmoid bonding curve. Early binding = big shares. Late = marginal.

---

## D

### Decay
Speed tax multiplier dropping over time. Full decay (1×) after 1024 slots (~51s).

### Depth
Position relative to center. Higher Z = deeper. Fe (Z=26) is deepest.

### Discharge
Merge unbound Charge balance back into Wallet.

### Discriminator
First 8 bytes of instruction data. Identifies which instruction.

---

## E

### Edge Element
One of 12 Elements touching board perimeter (Z=1–12). Bind and Unbind happen here.

### Ejection Fee
Fee for voluntarily unbinding from an edge Element.

### Element
Contiguous tile region on the board. Named after chemical elements. 26 total.

### ElementIndex
Compound ID: atomic number (8 bits) + generation (56 bits). Detects stale references.

### Extract
Convert Wallet Gluon back to stablecoins.

---

## F

### Fee
Cost for actions. Never burned—flows into pots, becomes rewards.

### Fee Multiplier
See Speed Tax.

---

## G

### Generation
Counter incrementing each reset. Part of ElementIndex. Ensures Charges claim from correct reset.

### Gluon
In-game currency. 1:1 with USDC/USDT. Held in Wallets (liquid) or Charges (active).

### Gravity
Value flowing toward center. Fee uses destination curve's saturation when moving inward, source curve's saturation when moving outward. Deeper elements typically have lower saturation due to larger curves.

---

## I

### Infuse
Convert stablecoins to Gluon in Wallet.

### Bind Fee
Movement fee for binding a Charge to an Element. See: Movement Fee.

### Inward
Toward higher Z (toward Fe). Fees use destination saturation.

---

## L

### LUT (Lookup Table)
Precomputed sigmoid values. O(1) share calculation.

---

## M

### MIN_FEE
0.1 Gluon floor. No dust transactions.

### Movement Fee
Rebind cost. Based on distance², saturation, speed tax, balance. Same formula for Bind, Unbind, Rebind. See: [players/REFERENCE.md](players/REFERENCE.md#fee-formulas).

---

## O

### Outward
Toward lower Z (toward edge). Fees use source saturation.

### Overload
Trigger Element reset when saturation exceeds 100%. Anyone can call overload once saturation >= 100%, but in practice the binder who crosses the threshold typically calls it immediately. Creates Artefact, distributes rewards.

---

## P

### Overheat
Element resets with significant phantom saturation. Actual shares < 100%, rest is phantom from churn.

### PDA (Program Derived Address)
Deterministic account address from seeds. Wallets and Charges are PDAs.

### Phantom Saturation
Saturation not owned by any Charge. Created when fees reduce balance—exit removes less saturation than entry added. Gap between saturation and shares indicates phantom level. Incumbents benefit: rewards split among real shares only.

### Pod
Bytemuck trait for fixed memory layout. All account types implement it.

### Pot
Accumulated value in an Element. Grows from fees and donations. Splits at reset.

---

## Q

### Q8.24
Fixed-point: 8 integer, 24 fractional bits. u32. Used for saturation.

### Q16.48
Fixed-point: 16 integer, 48 fractional bits. u64. Used for costs and shares.

### Quantum Pocket
(Not implemented) Global pool receiving yield when Fe resets. Distributes edge-inward.

---

## R

### Rebind
Move bound Charge to adjacent Element. Fee calculated as atomic number distance between source and destination.

### Reset
Saturation crosses threshold. Pot distributes, Element clears, generation ticks.

### Reset Cycle
Empty → saturation builds → threshold → reset → repeat.

---

## S

### Saturation
Sum of shares of all bound Charges. Entry adds, exit removes. Reset at 100%.

### Share
See Commitment Share.

### Sigmoid Curve
S-curve for share efficiency. Inflection at 50%. Early binders more efficient than late (theoretical max: 20× at 0% saturation; at 10% saturation: ~12× more efficient than at 90%).

### Slot
Solana time unit. ~50ms on L2. Used for speed tax.

### Speed Tax
1× to 128× fee multiplier based on time since last action. Punishes rushing. Full decay after ~51s.

---

## T

### Threshold
100% saturation. Overload becomes possible.

### Tile
Single cell on 8×8 board. Elements are tile groups.

### Trigger
Charge that Overloads. Gets immediate reward, re-binds first in new cycle.

### TVL (Total Value Locked)
Aggregate Gluon across all Elements and accounts. Board account tracks it.

---

## U

### Unbind
Remove bound Charge from board. Edge Elements only. Fee calculated as distance from source element to Z=0 (offboard).

### Unbound
Charge off the board. Can't participate until bound.

---

## V

### Vacuum
Saturation gap after large exit. Phantom remains, apparent opportunity. Often a trap—speed tax punishes the rush.

### Vault
Program-owned token account. Stablecoins backing Gluon.

### Vent
Donate Charge Gluon to Element pot. No saturation change.

---

## W

### Wallet
Player's Gluon account. Liquid. Allocate to Charges or Extract to stablecoins.

---

## Z

### Z
See Atomic Number.

---

## See Also

- **[players/README.md](players/README.md)** — player documentation index
- **[developers/README.md](developers/README.md)** — developer documentation index
- **[Concepts](players/CONCEPTS.md)** — mental model
- **[Reference](players/REFERENCE.md)** — constants and formulas
- **[Architecture](developers/ARCHITECTURE.md)** — technical structure
