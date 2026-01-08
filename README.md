# TOKAMAK64

A novel game where everything happens on a shared 8×8 board, where players navigate pressure, competition, and cooperation to generate emergent gameplay through mathematically elegant mechanics.

## What Is TOKAMAK64?

TOKAMAK64 is a strategy game set on a fixed chessboard divided into irregular regions called **Elements**. Unlike traditional board games where you control a single piece, you exist as a presence within an entire region—occupying all squares of that region simultaneously when you enter it.

The game is not about moving faster or capturing pieces. It's about managing **shared value**, understanding **pressure dynamics**, and making strategic choices about when to stay, exit, or redirect resources.

## The Board

The world is a static 8×8 grid—64 squares that never change, never grow, never rotate. Every player sees the same board. There is no fog of war, no hidden areas, no off-board waiting.

The board is divided into **Elements**—irregular, connected regions of varying sizes. Each square belongs to exactly one Element. Some Elements are single squares; others span multiple connected squares.

## Shared Presence

When a charge is in an Element, it occupies **all its squares at once**—not one after another, but all simultaneously. Multiple charges (yours or others') can exist in the same Element without conflict or competition for space.

This fundamental mechanic—shared presence without spatial exclusivity—creates the conditions for all emergent gameplay in the system. It means that positions on the board are not scarce; the scarcity is in the *value accumulated* by shared presence.

## Movement & Board Access

Movement is group-to-group, not square-to-square:

- A charge can move only to an **adjacent Element** (one that shares a full edge, not just a corner)
- The charge leaves its entire current Element and enters its entire destination Element instantly
- Movement is always possible between neighbors, regardless of how many charges are present
- Movement costs scale with the **depth** of the destination Element (deeper Elements are more expensive to reach)
- A speed tax applies: moving quickly costs more than waiting

**Critical constraints on board access:**

- Charges can only **enter the board (Inject) from edge Elements**—those on the perimeter that touch the board boundary
- Charges can only **exit the board (Eject) from edge Elements**—returning to unbound status outside play
- This creates natural "gateways" to the board and means deep Elements are harder to escape from (you must traverse back toward edges)

## How Value Flows

The game operates on a closed-loop economic system where costs feed back as opportunities:

### Pressure & Crowds

When many charges gather in an Element, **pressure accumulates** proportionally to the total value held by all charges present. The more crowded it becomes:

- Movement costs increase (it becomes more expensive to leave)
- An **oversaturation point** emerges—a threshold of accumulated pressure determined by a mathematical curve
- When the oversaturation point is reached, the Element **resets**: all charges are ejected and the pot is distributed

### Overload & Rewards

When an Element overloads (saturation exceeds oversaturation point):

- All accumulated value (the shared pot) is **distributed to bound charges** that were present during the accumulation
- Reward distribution is proportional to each charge's value relative to total value in the Element
- **All bound charges are automatically ejected for free**—no exit cost, instant departure
- Overload is not punishment—it's the core value redistribution mechanism
- **Unbound charges** (those that have already exited via Eject) receive nothing from overload

This creates a fundamental tension: you can voluntarily exit early (via Eject, paying costs from an edge Element), or stay and hope the Element overloads (getting rewards + free ejection).

### Costs as Investment

Voluntary actions incur costs:

- **Movement** (Rebind): Costs scale with destination depth and time elapsed since last move (speed tax)
- **Exiting voluntarily** (Eject): Costs apply when unbinding a charge from an Element
- **Compressing value** (Compress): Costs scale with pot size and destination depth; fees are added to the pot being moved
- **Donating to pot** (Vent): Transfers charge value to the Element's shared pot

Importantly: **costs are never destroyed**. All costs are redirected into the board as shared value in Elements, becoming part of future oversaturation points and rewards. This creates a closed economic loop.

### Total Value Locked (TVL) Invariant

The board tracks total active Gluon in **charge accounts** on-board. TVL represents the sum of all charge balances currently participating in board state (either bound or unbound-but-active). Fees paid to element pots are NOT subtracted from TVL—they remain part of the system as shared value.

TVL increases when:
- **Inject**: Charge enters board (`board.tvl += charge.balance` AFTER fee deduction). Fee is moved to pot, so total value is preserved.
- **Rebind/Compress**: Charge moves between elements (no TVL change; balance unchanged)
- **Overload**: One charge re-bound; other charges' TVL removed (net: old TVL - ejected charges' balances = new TVL)

TVL decreases when:
- **Eject**: Charge exits board (`board.tvl -= charge.balance` AFTER fee deduction). Fee stays in pot.
- **Claim**: Reward distributed from artefact to charge (`board.tvl` unchanged; internal redistribution)
- **Overload**: All charges ejected except bonus charge (`board.tvl -= ejected_charges_balance`)

## Core Mechanics

The game provides 13 actions (instructions) that shape gameplay, organized into 5 functional groups:

### Entry & Exit Operations
- **Inject**: Bind a charge onto the board into a **target edge Element only** (must be on the board's perimeter); charge becomes bound and participates in pressure mechanics
- **Eject**: Voluntarily unbind a charge from a **current edge Element only**, moving it outside the board; applies exit cost; unbound charges can no longer claim future rewards
- **Overload (automatic)**: When an Element's pressure exceeds its oversaturation point, all bound charges are automatically ejected for free, receive proportional reward share, and become unbound

### Account Initialization
- **InitCharge**: Initialize a new charge account (PDA) for a player; derives from signer + counter; sets signer as authority
- **InitWallet**: Initialize a new wallet account (PDA) for a player; derives from signer + mint; sets signer as authority

### Wallet & Balance Management
- **Charge**: Create a new charge by allocating Gluon from wallet to a charge account; multiple charges per player allowed
- **Discharge**: Merge a charge's remaining Gluon back into your wallet account
- **TopUp**: Convert stable tokens (USDT/USDC) into Gluon in your wallet (1:1 conversion); entry point for on-chain value
- **Drain**: Convert wallet Gluon back to stable tokens in your ATA; exit point for on-chain value

### Movement & Value Transfer
- **Rebind**: Move a bound charge from one Element to an adjacent Element; incurs movement costs. Fee routing: moving **inward** (toward center, higher atomic number) pays fee to **destination** (funds deeper element); moving **outward** (toward edge, lower atomic number) pays fee to **source** (taxes departing charge)
- **Compress**: Move an Element's accumulated pot inward to a deeper adjacent Element while rebinding the charge; incurs both migration fee (depth cost) and merge fee (consolidation cost); both fees accumulate in destination pot (fuels deeper element)
- **Vent**: Donate part of a bound charge's value to its current Element's shared pot; **charge must be currently bound to that element** (validated by index match); reduces individual share but accelerates reaching Element's oversaturation point

### Overload & Rewards
- **Overload**: Forcefully trigger an Element reset when saturation exceeds threshold; **the charge invoking the Overload receives a bonus**: it immediately re-binds to the reset element at genesis (gaining advantage in the new pot). **All other charges** in the element are automatically ejected for free and become unbound. Reward distribution: charges present at reset receive proportional share based on their value during accumulation; unbound charges (already exited) receive nothing
- **Claim**: Collect your charge's proportional reward share from an Element's pot after it overloads; distributes based on share value at time of reset; **charge must have the exact element index (atomic number + generation) matching the artefact to claim**

## Technical Clarifications

### Element Identity and Binding State

Each Element has a unique **index** that encodes both identity and versioning:
- **Atomic Number** (8 MSB): 1–26 (27 distinct Elements: Hydrogen through Iron)
- **Generation** (56 LSB): Counter incremented when Element resets (overloads)
- **Index Format**: `[generation_56bits | atomic_number_8bits]`

**Bound vs. Unbound:**
- A charge is **bound** when `index.atomic_number() != 0` (holds an atomic number 1–26)
- A charge is **unbound** when `index == 0` (atomic number and generation both zero)
- Unbound charges are off-board and cannot participate in Element mechanics or claim rewards

**Why This Matters:**
- Generation increments allow detecting stale charge references after Element reset
- A charge can only **claim** from an artefact if its index exactly matches (same atomic # AND generation)
- This prevents claiming from the wrong Element reset or double-claiming

### Fee Routing Logic

When a charge moves between Elements via **Rebind**, the fee goes to one of two places:

1. **Moving Inward** (increasing atomic number, toward center)
   - Direction: `src.index < dst.index`
   - Fee recipient: **destination Element**
   - Rationale: Funds deeper Elements, accelerating value accumulation in high-risk zones

2. **Moving Outward** (decreasing atomic number, toward edge)
    - Direction: `src.index > dst.index`
    - Fee recipient: **source Element**
    - Rationale: Taxes departing charges, incentivizing staying or exiting via deliberate Eject

**Compress** always moves inward: both migration and merge fees go to destination.

### Overload Mechanics: Bonus System

When **Overload** is triggered:

1. **Snapshot**: Element pot and index are copied to Artefact
2. **Claim (triggering charge only)**: Triggering charge receives reward based on its share
3. **Reset**: Element generation increments, curve/pot cleared
4. **Bonus**: Triggering charge is immediately re-bound to reset Element at genesis (first mover advantage in new accumulation cycle)
5. **Ejection**: All other bound charges are automatically unbound (become off-board) and may later claim via separate Claim action

**Key Invariant:** Only the triggering charge stays bound; all others are ejected. This is a reward for paying high fees during accumulation (speed tax compounds at high saturation).

### Claim Sequence

Claiming happens in two phases:

1. **During Overload** (internal):
   - Overload processor calls `claim()` for triggering charge
   - Reward computed and added to charge balance
   - Charge share cleared (share = 0)
   - Charge becomes unbound (index.clear())

2. **Separate Claim Transaction** (external):
   - Any other charge that was bound during overload can claim later
   - Requires exact index match (atomic # + generation) to validate presence at reset time
   - Reward distributed proportionally
   - Charge becomes unbound

**Why Index Must Match:**
- Validates charge was actually bound to this Element when it reset
- Different generation = different Element reset, so charge ineligible
- Prevents claiming from wrong Elements or re-claiming

## Emergence & Depth

### Board Topology: Depth Matters

TOKAMAK64 is designed around a principle: **depth creates gradient**. The board has natural layers defined by distance from edges:

- **Edge Elements**: Only entry/exit points; all charges must Inject here and must Eject here. Easier to access but shallower pots.
- **Shallow Elements** (one step inward): Reachable from edges, but deeper than perimeter. Medium-risk, medium-reward.
- **Deep Elements** (toward center): Require multiple moves to reach, require multiple moves to escape. Harder to access but accumulate larger, more valuable pots.
- Costs scale with depth, creating natural incentives for different player strategies

This topological gradient is not arbitrary—it structures where value concentrates, creates natural "traps" (deep pots that are expensive to escape), and determines how the game evolves over time. Players must always navigate back toward edges to exit, creating natural pressure and strategic chokepoints.

### Emergent Patterns

No explicit "strategy" is coded into the system. Instead, strategies emerge from cost incentives and value flow:

- **Shallow holding**: Charges may stay in edge Elements to minimize movement costs while waiting for pots to mature, knowing they can exit at low cost
- **Deep gambling**: Charges trapped in deep Elements may bet on overload before the pressure tax becomes unbearable—overload gives free exit + rewards
- **Deep compression**: Sophisticated players compress pots inward to seed deeper Elements with larger, harder-to-escape pots
- **Speed vs. patience**: The speed tax creates tension—immediate action costs more than waiting, but waiting sacrifices opportunity
- **Strategic ejection**: Players may deliberately stay to absorb pressure, betting that overload is imminent rather than paying voluntary exit costs
- **Unintended cooperation**: Even selfish actions (moving, venting) fund future rewards for others, creating emergent mutual benefit
- **Value oscillation**: Overload in deep Elements sends charges outward for free, creating waves of activity and value redistribution

## Why Play?

TOKAMAK64 offers several sources of engagement:

1. **Economic gameplay**: Understand pressure curves, predict when elements will overload, time your exits
2. **Spatial puzzles**: Navigate the board's topology, plan efficient paths, understand Element connectivity
3. **Competition within cooperation**: Every action you take funds future rewards for others, creating subtle incentives
4. **Emergent narratives**: The natural flow of players concentrating value creates hot zones, cold zones, rushes, and reversals
5. **Accessibility with depth**: Simple rules support sophisticated decision-making

## System Architecture

TOKAMAK64 is implemented as a Solana on-chain program (smart contract), ensuring:

- **Complete transparency**: All game state is public and verifiable on-chain
- **No hidden information**: Every pot, every pressure value, every reward is visible to all players at all times
- **Fairness**: Game rules cannot be changed or bypassed; enforcement is deterministic and auditable
- **Permanence**: All actions are immutably recorded; the game state cannot be rolled back or modified retroactively

### Technical Components

- **Board**: Tracks Element state including accumulated pots, pressure levels, and topology
- **Player Accounts**: Wallets (holding Gluon) and charges (distinct entities holding Gluon, positioned on board)
- **Curve Logic**: Mathematical functions that determine movement costs, overload thresholds (oversaturation points), and reward distribution based on Element state
- **Processors**: Transaction handlers that atomically execute game actions and update state

## The Closed Loop

The elegance of TOKAMAK64 is that it requires no external input to sustain itself:

1. Players move and take actions
2. Actions cost resources
3. Costs accumulate as shared value in Elements (pots)
4. Shared value creates pressure
5. Pressure triggers overload and redistribution of accumulated rewards
6. Redistribution sends players outside of the board
7. Activity continues, creating new value

This loop generates complexity and emergent behavior from simple, deterministic rules.

## Player Flow: From Wallet to Board and Back

The game has a clear progression through different states:

### Setup
1. Create a Solana wallet with an ATA (Associated Token Account) for USDT or USDC
2. Fund your ATA with stable tokens
3. **TopUp**: Convert stable tokens into Gluon in your in-game wallet (1:1 conversion)

### Creating & Positioning Charges
4. **Charge**: Create a new charge account by allocating Gluon from your wallet to it
5. A single player wallet can create unlimited charges (limited only by available Gluon)
6. **Inject**: Send a charge onto the board into an Element of your choice
7. Multiple charges can occupy the same Element; they accumulate pressure together

### Active Play
8. While bound to an Element, a charge participates in pressure mechanics, overload events, and reward claims
9. **Rebind**: Move a charge to an adjacent Element
10. **Compress**: Move a pot deeper into the board, increasing its value
11. **Vent**: Donate part of a charge's value to its Element's shared pot
12. **Overload**: Forcefully trigger an Element to overload (reset)
13. **Claim**: Collect your share of rewards after overload

### Exiting & Consolidation
14. **Eject**: Unbind a charge from its Element, moving it outside the board
15. Once outside, a charge cannot participate in Element mechanics or claims
16. **Discharge**: Merge a charge's Gluon back into your wallet
17. Charges can be combined outside the board (one is discharged into your wallet, then used to top up another)

### Withdrawal
18. **Drain**: Convert wallet Gluon back to stable tokens in your ATA
19. Withdraw stable tokens from Solana whenever you wish

### Key Distinctions

- **Wallet**: Holds Gluon in liquid form (unallocated); serves as your in-game resource pool. Created implicitly on first TopUp.
- **Charge**: A distinct account holding Gluon; the actual entity that occupies Elements and claims rewards. Charges are **bound** while on the board (index set, can claim if Element breaks) and **unbound** after Eject (index cleared, can never claim).
- **Gluon**: In-game currency representing economic value within TOKAMAK64. Used for all actions. Cannot be transferred directly between players.
- **Stable tokens** (USDT/USDC): Held in your Solana ATA. The entry and exit point for real-world value; convertible to/from Gluon via TopUp/Drain (1:1).
- **Element pot**: Shared value accumulated by charges present during Element's accumulation; distributed to bound charges when Element breaks.

## Getting Started

A typical gameplay session:

1. Fund your Solana wallet with USDT or USDC
2. Create an ATA (Associated Token Account) if needed
3. Use **TopUp** to convert stable tokens → Gluon in your in-game wallet (1:1)
4. Use **Charge** to create new charges, allocating Gluon from your wallet to each
5. Use **Inject** to position charges onto the board into edge Elements (perimeter only)
6. Navigate strategically using **Rebind** to move between Elements, building exposure to high-value pots
7. Use **Vent** to donate value to your current Element's shared pot if you believe it will break soon
8. Use **Compress** to move pots deeper if you're positioned to benefit from richer payouts
9. Use **Claim** to collect your reward share when Elements overload
10. Use **Overload** to trigger early overloads if you believe timing favors you
11. **Decide**: Do you stay hoping for overload (free exit + rewards) or exit voluntarily?
12. If exiting voluntarily: Use **Rebind** to navigate back toward the board's edge
13. Use **Eject** to unbind charges only from edge Elements, paying exit costs (note: unbound charges cannot claim future rewards)
14. If overload happens: You're automatically ejected for free with your reward share already distributed
15. Use **Discharge** to merge unbound charges back into your wallet
16. Use **Drain** to convert remaining Gluon back to stable tokens and withdraw

