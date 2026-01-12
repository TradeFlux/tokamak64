# Instructions Reference

Complete reference for all 13 on-chain instructions, including account layouts, data formats, and error conditions.

## Program ID

```
DuJrE9ZB4TqcMByw9g4CiDQdNQosPQCQw2ECWGfLiyi
```

## Instruction Format

All instructions use the same encoding:

```
┌────────────────────────────────────────────────────────┐
│  Discriminator (8 bytes, little-endian u64)            │
├────────────────────────────────────────────────────────┤
│  Instruction Data (variable, instruction-specific)     │
└────────────────────────────────────────────────────────┘
```

Discriminator values match the `TokamakInstruction` enum variant indices (0–12).

---

## Account Initialization

### InitWallet (0)

Initialize a player's wallet account.

**Discriminator**: `0`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Wallet owner |
| 1 | wallet | No | Yes | Wallet PDA to initialize |
| 2 | mint | No | No | Token mint (USDC/USDT) |
| 3 | system_program | No | No | System program |

**Instruction Data**: None

**PDA Seeds**: `["wallet", authority, mint]`

**Errors**:
- `AccountAlreadyInitialized` — Wallet already exists

---

### InitCharge (1)

Initialize a new charge account.

**Discriminator**: `1`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Charge owner |
| 1 | wallet | No | Yes | Player's wallet |
| 2 | charge | No | Yes | Charge PDA to initialize |
| 3 | system_program | No | No | System program |

**Instruction Data**: None (charge index derived from wallet.charge_count)

**PDA Seeds**: `["charge", authority, charge_index]`

**Errors**:
- `AccountAlreadyInitialized` — Charge already exists

---

## Wallet & Balance Management

### Charge (2)

Allocate Gluon from wallet to charge.

**Discriminator**: `2`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Wallet authority |
| 1 | charge | No | Yes | Charge account |
| 2 | wallet | No | Yes | Player's wallet |

**Instruction Data**:
```
┌────────────────────────────────────────┐
│  amount: u64 (8 bytes)                 │
└────────────────────────────────────────┘
```

**Errors**:
- `InsufficientFunds` — Wallet balance < amount
- `InvalidAuthority` — Signer doesn't match wallet authority

---

### Claim (3)

Collect reward share from reset artefact.

**Discriminator**: `3`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Charge authority |
| 1 | charge | No | Yes | Charge account |
| 2 | artefact | No | Yes | Reset element snapshot |

**Instruction Data**: None

**Errors**:
- `IndexMismatch` — Charge index doesn't match artefact
- `AlreadyClaimed` — Charge share already claimed
- `InvalidAuthority` — Signer doesn't match charge authority

---

### Discharge (6)

Merge charge balance back to wallet.

**Discriminator**: `6`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Charge authority |
| 1 | charge | No | Yes | Charge account |
| 2 | wallet | No | Yes | Player's wallet |

**Instruction Data**: None

**Errors**:
- `ChargeBound` — Charge is still bound to an element
- `InvalidAuthority` — Signer doesn't match charge authority

---

### Infuse (11)

Convert stablecoins to Gluon.

**Discriminator**: `11`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Wallet authority |
| 1 | wallet | No | Yes | Player's wallet |
| 2 | source | No | Yes | Player's token account |
| 3 | mint | No | No | Token mint |
| 4 | vault | No | Yes | Program's token vault |
| 5 | token_program | No | No | SPL Token program |

**Instruction Data**:
```
┌────────────────────────────────────────┐
│  amount: u64 (8 bytes)                 │
└────────────────────────────────────────┘
```

**Errors**:
- `InsufficientFunds` — Source account balance < amount
- `InvalidMint` — Mint doesn't match wallet mint

---

### Extract (5)

Convert Gluon back to stablecoins.

**Discriminator**: `5`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Wallet authority |
| 1 | wallet | No | Yes | Player's wallet |
| 2 | vault | No | Yes | Program's token vault |
| 3 | mint | No | No | Token mint |
| 4 | destination | No | Yes | Player's token account |
| 5 | vault_authority | No | No | Vault authority PDA |
| 6 | token_program | No | No | SPL Token program |

**Instruction Data**:
```
┌────────────────────────────────────────┐
│  amount: u64 (8 bytes)                 │
└────────────────────────────────────────┘
```

**Errors**:
- `InsufficientFunds` — Wallet balance < amount
- `InvalidMint` — Mint doesn't match wallet mint

---

## Board Entry & Exit

### Bind (9)

Place charge onto board (edge element only).

**Discriminator**: `9`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Charge authority |
| 1 | charge | No | Yes | Charge account |
| 2 | element | No | Yes | Target edge element |
| 3 | board | No | Yes | Global board state |

**Instruction Data**: None

**Errors**:
- `ChargeBound` — Charge already bound
- `NotEdgeElement` — Target element is not at edge (Z > 12)
- `InsufficientBalance` — Charge balance insufficient for fee
- `InvalidAuthority` — Signer doesn't match charge authority

**Effects**:
- Charge becomes bound to element
- Commitment share measured and recorded
- Element saturation increases
- Movement fee deducted from charge, added to element pot (distance: Z=0 → element)

---

### Unbind (8)

Remove charge from board (edge element only).

**Discriminator**: `8`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Charge authority |
| 1 | charge | No | Yes | Charge account |
| 2 | element | No | Yes | Current edge element |
| 3 | board | No | Yes | Global board state |

**Instruction Data**: None

**Errors**:
- `ChargeUnbound` — Charge not bound
- `NotEdgeElement` — Current element is not at edge
- `IndexMismatch` — Charge not bound to this element
- `InsufficientBalance` — Charge balance insufficient for fee

**Effects**:
- Charge becomes unbound
- Element saturation decreases
- Movement fee deducted from charge, added to element pot (distance: element → Z=0)

---

## Movement

### Rebind (7)

Move charge to adjacent element.

**Discriminator**: `7`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Charge authority |
| 1 | charge | No | Yes | Charge account |
| 2 | source | No | Yes | Source element |
| 3 | destination | No | Yes | Destination element |

**Instruction Data**: None

**Errors**:
- `ChargeUnbound` — Charge not bound
- `NotAdjacent` — Elements don't share an edge
- `IndexMismatch` — Charge not bound to source element
- `InsufficientBalance` — Charge balance insufficient for fee

**Effects**:
- Charge unbinds from source, binds to destination
- New commitment share measured at destination
- Saturation updated on both elements
- Fee routed based on direction (inward → destination pot, outward → source pot)
- Charge timestamp updated (for speed tax)

---

### Compress (4)

Move element pot to adjacent deeper element.

**Discriminator**: `4`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Charge authority |
| 1 | charge | No | Yes | Charge account |
| 2 | source | No | Yes | Source element |
| 3 | destination | No | Yes | Destination element |

**Instruction Data**: None

**Errors**:
- `ChargeUnbound` — Charge not bound
- `NotAdjacent` — Elements don't share an edge
- `InvalidCompression` — dst.index <= src.index
- `IndexMismatch` — Charge not bound to source element
- `InsufficientBalance` — Charge balance insufficient for fee

**Effects**:
- Movement fee + compression fee deducted from charge
- Source pot merged into destination pot
- Both fees added to destination pot
- Charge moves from source to destination

---

### Vent (12)

Donate charge balance to element pot.

**Discriminator**: `12`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Charge authority |
| 1 | charge | No | Yes | Charge account |
| 2 | element | No | Yes | Target element |

**Instruction Data**:
```
┌────────────────────────────────────────┐
│  amount: u64 (8 bytes)                 │
└────────────────────────────────────────┘
```

**Errors**:
- `ChargeUnbound` — Charge not bound
- `IndexMismatch` — Charge not bound to this element
- `InsufficientBalance` — Charge balance < amount

**Effects**:
- Charge balance decreases by amount
- Element pot increases by amount
- No saturation change

---

## Reset Operations

### Overload (10)

Trigger element reset when saturated.

**Discriminator**: `10`

**Accounts**:
| Index | Account | Signer | Writable | Description |
|-------|---------|--------|----------|-------------|
| 0 | authority | Yes | No | Charge authority |
| 1 | charge | No | Yes | Triggering charge |
| 2 | element | No | Yes | Element to reset |
| 3 | artefact | No | Yes | Artefact to create |
| 4 | board | No | Yes | Global board state |

**Instruction Data**: None

**Errors**:
- `ChargeUnbound` — Charge not bound
- `IndexMismatch` — Charge not bound to this element
- `NotSaturated` — Element saturation < threshold
- `ArtefactExists` — Artefact already initialized

**Effects**:
1. Artefact created with element snapshot (pot, index, shares)
2. Triggering charge receives proportional reward
3. Triggering charge re-binds to reset element (first position)
4. Element generation increments
5. Element pot and saturation cleared
6. All other charges' indices invalidated (become unbound)

---

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| 0 | `InvalidInstructionData` | Discriminator unknown or data malformed |
| 1 | `InvalidAccountData` | Account deserialization failed |
| 2 | `MissingRequiredSignature` | Authority not signer |
| 3 | `IllegalOwner` | Account not owned by program |
| 4 | `InsufficientFunds` | Balance insufficient for operation |
| 5 | `AccountAlreadyInitialized` | Account already exists |
| 6 | `ChargeBound` | Charge is bound (operation requires unbound) |
| 7 | `ChargeUnbound` | Charge is unbound (operation requires bound) |
| 8 | `NotEdgeElement` | Element is not at board edge |
| 9 | `NotAdjacent` | Elements don't share an edge |
| 10 | `IndexMismatch` | Charge/element index mismatch |
| 11 | `NotSaturated` | Element below saturation threshold |
| 12 | `InvalidCompression` | Compression direction invalid |
| 13 | `InvalidAuthority` | Signer doesn't match account authority |
| 14 | `InvalidMint` | Token mint mismatch |
| 15 | `ArtefactExists` | Artefact already created |
| 16 | `AlreadyClaimed` | Reward already claimed |

---

## Instruction Summary

| Index | Name | Purpose | Key Accounts |
|-------|------|---------|--------------|
| 0 | InitWallet | Create wallet PDA | authority, wallet, mint |
| 1 | InitCharge | Create charge PDA | authority, wallet, charge |
| 2 | Charge | Wallet → Charge transfer | wallet, charge |
| 3 | Claim | Collect reset reward | charge, artefact |
| 4 | Compress | Move pot inward | charge, src, dst |
| 5 | Extract | Gluon → stablecoin | wallet, vault, destination |
| 6 | Discharge | Charge → Wallet merge | charge, wallet |
| 7 | Rebind | Move between elements | charge, src, dst |
| 8 | Unbind | Exit board at edge | charge, element |
| 9 | Bind | Enter board at edge | charge, element |
| 10 | Overload | Trigger reset | charge, element, artefact |
| 11 | Infuse | Stablecoin → Gluon | wallet, source, vault |
| 12 | Vent | Donate to pot | charge, element |

---

## See Also

- **[Architecture](ARCHITECTURE.md)** — Crate structure and data flow
- **[Integration](INTEGRATION.md)** — Client SDK usage
