# Glossary

Definitions of all terms used in TOKAMAK64.

---

## A

### Adjacent
Two Elements that share a full tile edge (not just corners). Movement via Rebind requires adjacency.

### Artefact
A snapshot of an Element at the moment of reset. Contains the pot amount, Element index, and commitment shares. Used by non-triggering Charges to claim rewards.

### Atomic Number (Z)
The identifier for an Element (1–26). Higher Z means deeper position on the board. Named after chemical elements: H=1, He=2, ... Fe=26.

### ATA (Associated Token Account)
A player's Solana token account for USDC/USDT. Entry and exit point for real-world value.

---

## B

### Balance
The amount of Gluon held by a Wallet or Charge.

### Bind
The operation that places an unbound Charge onto the board. Only allowed at edge Elements.

### Board
The 8×8 grid containing 26 Elements. The playing field for TOKAMAK64.

### Bound
A Charge state indicating it occupies an Element on the board. Bound Charges participate in saturation and can receive rewards at reset.

---

## C

### Charge
A player-controlled entity that enters the board. Each Charge has a Gluon balance, can be bound to one Element at a time, and earns commitment shares when bound.

### Claim
The operation to collect reward share from a reset Artefact. Used by Charges that were bound during a reset but did not trigger it.

### Commitment Share
A value measuring a Charge's stake in an Element. Determined by the sigmoid curve at binding time. Larger shares receive proportionally larger rewards at reset.

### Compress
The operation to move an Element's pot to an adjacent deeper Element. Requires dst.index > src.index.

### Compression Fee
A fee (up to 5% of pot) paid when compressing. Added to the destination pot along with the source pot.

### Curve
The sigmoid bonding curve that determines commitment share efficiency based on saturation. Early binding yields large shares; late binding yields marginal shares.

---

## D

### Decay
The reduction in speed tax multiplier over time. Full decay (1× multiplier) occurs after 1024 slots (~51 seconds).

### Depth
An Element's position relative to center. Higher atomic number = deeper. Fe (Z=26) is deepest.

### Discharge
The operation to merge an unbound Charge's balance back into the Wallet.

### Discriminator
The first 8 bytes of instruction data identifying which instruction to execute. Corresponds to the TokamakInstruction enum variant index.

---

## E

### Edge Element
One of the 12 Elements touching the board perimeter (Z=1–12: H, He, Li, Be, B, C, N, O, F, Ne, Na, Mg). Only edge Elements allow Bind and Unbind operations.

### Ejection Fee
Fee paid when unbinding voluntarily from an edge Element.

### Element
A contiguous region of tiles on the board, named after a chemical element. 26 total, identified by atomic number (Z).

### ElementIndex
A compound identifier encoding both atomic number (8 bits) and generation (56 bits). Enables detecting stale references after reset.

### Extract
The operation to convert Gluon from Wallet back to stablecoins (USDC/USDT).

---

## F

### Fee
Cost deducted from a Charge for voluntary actions. Fees are never burned—they flow into Element pots.

### Fee Multiplier
See Speed Tax.

---

## G

### Generation
A counter that increments each time an Element resets. Part of the ElementIndex. Ensures Charges can only claim from the specific reset they witnessed.

### Gluon
The in-game currency. Convertible 1:1 with USDC/USDT. Held in Wallets (liquid) or Charges (active).

### Gravity
The emergent tendency for value to flow inward. Caused by fee asymmetry: inward moves use destination saturation (typically lower), making them cheaper.

---

## I

### Infuse
The operation to convert stablecoins (USDC/USDT) into Gluon in a Wallet.

### Injection Fee
Fee paid when binding a Charge to an Element.

### Inward
Movement direction toward higher atomic number (toward Fe). Fees use destination saturation.

---

## L

### LUT (Lookup Table)
Precomputed sigmoid curve values used by the curve crate. Enables O(1) commitment share calculation.

---

## M

### MIN_FEE
The minimum fee floor (0.1 Gluon). Prevents dust transactions.

### Movement Fee
Fee paid when rebinding between Elements. Based on distance², saturation, speed tax, and balance.

---

## O

### Outward
Movement direction toward lower atomic number (toward edge). Fees use source saturation.

### Overload
The operation to trigger an Element reset when saturation exceeds threshold. Creates an Artefact and distributes rewards.

---

## P

### PDA (Program Derived Address)
A deterministic account address derived from seeds. Wallets and Charges are PDAs owned by the program.

### Pod
A trait from `bytemuck` indicating a type has a fixed memory layout and can be safely cast from bytes. All account types implement Pod for zero-copy access.

### Pot
Accumulated value in an Element. Grows from fees and donations. Distributed proportionally to bound Charges at reset.

---

## Q

### Q8.24
Fixed-point format used for saturation values. 8 integer bits, 24 fractional bits, stored as u32.

### Q16.48
Fixed-point format used for cumulative costs and commitment shares. 16 integer bits, 48 fractional bits, stored as u64.

### Quantum Pocket
(Not yet implemented) A global pool that receives external yield when Fe resets. Distributed sequentially from edge inward.

---

## R

### Rebind
The operation to move a bound Charge from one Element to an adjacent Element.

### Reset
The event when an Element's saturation exceeds threshold. Pot is distributed to bound Charges, Element clears, generation increments.

### Reset Cycle
The repeating pattern: empty Element → saturation builds → threshold crossed → reset → repeat.

---

## S

### Saturation
The sum of commitment shares of all Charges currently bound to an Element. Entry increases it, exit decreases it. Reset triggers when saturation exceeds threshold (100%).

### Share
See Commitment Share.

### Sigmoid Curve
The S-shaped function governing commitment share efficiency. Inflection at 50% saturation. Early binding is ~20× more efficient than late binding.

### Slot
A Solana time unit. On L2, approximately 50ms. Used for speed tax calculation.

### Speed Tax
A multiplier (1× to 128×) on movement fees based on time since the Charge's last action. Punishes rapid movement. Full decay after 1024 slots (~51 seconds).

---

## T

### Threshold
The saturation level (100% normalized to 1.0) at which an Element can be reset via Overload.

### Tile
A single cell on the 8×8 board. Elements are groups of adjacent tiles.

### Trigger
The Charge that executes Overload and causes a reset. Receives immediate reward and re-binds first in the new cycle.

### TVL (Total Value Locked)
Aggregate Gluon across all Elements and accounts. Tracked by the Board account.

---

## U

### Unbind
The operation to remove a bound Charge from the board. Only allowed at edge Elements.

### Unbound
A Charge state indicating it is off the board. Cannot participate in Element mechanics until bound.

---

## V

### Vault
Program-owned token account holding stablecoins backing Gluon.

### Vent
The operation to donate Gluon from a bound Charge to its Element's pot. Does not affect saturation.

---

## W

### Wallet
A player's Gluon account. Holds liquid Gluon that can be allocated to Charges or extracted to stablecoins.

---

## Z

### Z
See Atomic Number.

---

## See Also

- **[Player Concepts](players/CONCEPTS.md)** — Mental model explanation
- **[Reference](players/REFERENCE.md)** — Constants and formulas
- **[Architecture](developers/ARCHITECTURE.md)** — Technical structure
