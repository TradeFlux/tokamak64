# program

Solana on-chain program for TOKAMAK64. For game design, see the [main README](../../README.md).

## Program ID

```
DuJrE9ZB4TqcMByw9g4CiDQdNQosPQCQw2ECWGfLiyi
```

## Instructions

| # | Name | # | Name |
|---|------|---|------|
| 0 | InitCharge | 7 | Rebind |
| 1 | InitWallet | 8 | Eject |
| 2 | Charge | 9 | Inject |
| 3 | Claim | 10 | Overload |
| 4 | Compress | 11 | Infuse |
| 5 | Extract | 12 | Vent |
| 6 | Discharge | | |

## Account Layouts

### InitWallet / InitCharge
```
[0] signer    (signer)    - Authority
[1] wallet    (writable)  - Wallet PDA
[2] charge    (writable)  - Charge PDA (InitCharge only)
[2] mint      (readonly)  - Token mint (InitWallet only)
```

### Infuse / Extract
```
[0] authority (signer)    - Wallet authority
[1] wallet    (writable)  - Player wallet
[2] src/vault (writable)  - Source (Infuse) or vault (Extract)
[3] mint      (readonly)  - Token mint
[4] vault/dst (writable)  - Vault (Infuse) or destination (Extract)
[5] authority (readonly)  - Vault authority PDA (Extract only)
```

### Charge / Discharge
```
[0] signer    (signer)    - Wallet authority
[1] charge    (writable)  - Charge account
[2] wallet    (writable)  - Player wallet
```

### Inject / Eject
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] element   (writable)  - Edge element (dst for Inject, src for Eject)
[3] board     (writable)  - Global board state
```

### Rebind / Compress
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] src       (writable)  - Source element
[3] dst       (writable)  - Destination element (adjacent)
```

### Claim
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] artefact  (writable)  - Overloaded element snapshot
```

### Overload
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] target    (writable)  - Element to overload
[3] artefact  (writable)  - Artefact to create
[4] board     (writable)  - Global board state
```

### Vent
```
[0] signer    (signer)    - Charge authority
[1] charge    (writable)  - Charge account
[2] target    (writable)  - Element to receive donation
```

## Building

```bash
cargo build-sbf -p program
solana program deploy target/deploy/program.so
```
