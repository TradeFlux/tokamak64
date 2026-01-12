# TOKAMAK64 Documentation

Complete documentation for TOKAMAK64—a fully deterministic, transparent strategy game played on the Solana blockchain.

---

## 🚀 Quick Start

**New to the game?** Start here: → **[players/QUICKSTART.md](players/QUICKSTART.md)** (5 min read)

Learn the basics: entry, exit, movement, and how to profit from resets.

---

## 📚 Player Documentation

### Learning Path

1. **[QUICKSTART.md](players/QUICKSTART.md)** — Getting Started (5 min)
   - What is TOKAMAK64?
   - Core concepts overview
   - Basic gameplay loop

2. **[CONCEPTS.md](players/CONCEPTS.md)** — How It Works (10 min)
   - Mental models and mechanics
   - Elements, Charges, and movement
   - Fees, saturation, and the sigmoid curve
   - The closed loop of value

3. **[OPERATIONS.md](players/OPERATIONS.md)** — How To Play (15 min)
   - Step-by-step instructions for each action
   - Infuse, Bind, Rebind, Unbind, Compress, Overload, Claim
   - Common patterns and workflows

4. **[STRATEGY.md](players/STRATEGY.md)** — How To Win (20 min)
   - Entry timing and depth selection
   - Mechanic interactions and synergies
   - Board analysis and pattern recognition
   - Common mistakes and pro tips

5. **[REFERENCE.md](players/REFERENCE.md)** — Numbers & Formulas (10 min)
   - Exact fee calculations with examples
   - Board layout and element adjacency
   - Constants and thresholds
   - Quick reference card

6. **[GLOSSARY.md](GLOSSARY.md)** — Definitions (lookup)
   - Term definitions
   - Concept explanations
   - Cross-references

### By Topic

**Understanding the Game:**
- [CONCEPTS.md](players/CONCEPTS.md#the-board) — Elements (H through Fe, 26 total)
- [CONCEPTS.md](players/CONCEPTS.md#charges) — Charges and movement
- [CONCEPTS.md](players/CONCEPTS.md#fees) — Fee structure and routing
- [REFERENCE.md](players/REFERENCE.md#board-layout) — Board map and adjacency
- [GLOSSARY.md](GLOSSARY.md) — All terms defined

**Playing the Game:**
- [OPERATIONS.md](players/OPERATIONS.md#wallet-management) — Infuse, Charge, Discharge, Extract
- [OPERATIONS.md](players/OPERATIONS.md#board-entry-and-exit) — Bind, Unbind
- [OPERATIONS.md](players/OPERATIONS.md#movement) — Rebind between Elements
- [OPERATIONS.md](players/OPERATIONS.md#element-reset) — Compress, Overload, Claim

**Strategy & Optimization:**
- [STRATEGY.md](STRATEGY.md#1-entry-timing) — When to bind (early vs late)
- [STRATEGY.md](STRATEGY.md#2-depth-selection) — Where to position (edge vs core)
- [STRATEGY.md](STRATEGY.md#mechanic-interactions) — Fee asymmetry, speed tax, compression
- [STRATEGY.md](STRATEGY.md#board-analysis) — Pattern recognition and flow detection
- [STRATEGY.md](STRATEGY.md#common-mistakes) — Pitfalls to avoid

**Exact Numbers & Reference:**
- [REFERENCE.md](players/REFERENCE.md#fee-formulas) — Fee calculations with examples
- [REFERENCE.md](players/REFERENCE.md#fee-examples) — Sample calculations (10,000 Gluon base)
- [REFERENCE.md](players/REFERENCE.md#constants) — All game constants
- [REFERENCE.md](players/REFERENCE.md#commitment-share) — Sigmoid efficiency table
- [REFERENCE.md](players/REFERENCE.md#quick-card) — Quick reference card (flows, timings, rates)

---

## 🔍 Common Questions

### "How do I start playing?"
→ [QUICKSTART.md](players/QUICKSTART.md) - Get started in 5 minutes

### "How does the sigmoid curve work?"
→ [CONCEPTS.md](players/CONCEPTS.md#the-sigmoid-curve) - Efficiency by saturation level
→ [REFERENCE.md](players/REFERENCE.md#commitment-share) - Exact efficiency table

### "What are the fees for moving?"
→ [REFERENCE.md](players/REFERENCE.md#fee-formulas) - Unified fee formula
→ [REFERENCE.md](players/REFERENCE.md#fee-examples) - Real-world examples

### "How do I trigger a reset?"
→ [OPERATIONS.md](players/OPERATIONS.md#overload) - Step-by-step instructions
→ [STRATEGY.md](players/STRATEGY.md#the-snipe) - Strategic considerations

### "What's the best strategy?"
→ [STRATEGY.md](players/STRATEGY.md) - Complete strategy guide
→ [STRATEGY.md](players/STRATEGY.md#common-mistakes) - What to avoid

### "What does X mean?"
→ [GLOSSARY.md](GLOSSARY.md) - Complete terminology reference

---

## 📖 Reading Tips

### For New Players
1. Start with [QUICKSTART.md](players/QUICKSTART.md)
2. Read [CONCEPTS.md](players/CONCEPTS.md) to understand mechanics
3. Reference [OPERATIONS.md](players/OPERATIONS.md) when learning actions
4. Keep [REFERENCE.md](players/REFERENCE.md) open for numbers

### For Strategic Players
1. Master [CONCEPTS.md](players/CONCEPTS.md) - understanding enables strategy
2. Study [STRATEGY.md](players/STRATEGY.md) - mechanic interactions and patterns
3. Reference [REFERENCE.md](players/REFERENCE.md) - exact calculations for planning
4. Look up terms in [GLOSSARY.md](GLOSSARY.md)

### For Developers
→ **[developers/README.md](developers/README.md)** — Developer documentation index

---

## 📊 Game Overview

TOKAMAK64 is played on a fixed 8×8 grid divided into 26 **Elements**, named after chemical elements from Hydrogen (H, Z=1) at the edge to Iron (Fe, Z=26) at the center.

**Core Loop:**
1. Players bind **Charges** to Elements
2. Charges earn **commitment shares** based on when they bind (sigmoid curve)
3. Movement and reset fees fill the Element's **pot**
4. When **saturation** reaches 100%, the Element **resets**
5. The pot distributes proportionally to all bound Charges
6. The cycle repeats forever

**Key Principles:**
- **Fully transparent** — All state is visible: pots, saturation, positions
- **Deterministic** — All outcomes are calculable; no randomness
- **No burning** — All fees become rewards; value circulates
- **Measured, not staked** — Gluon determines shares but isn't locked

**See:** [REFERENCE.md](players/REFERENCE.md) for exact formulas, constants, and board layout.

---

## 🎮 Quick Reference

### Entry/Exit Flow
```
USDC → Infuse → Wallet → Charge → Bind → Board
Board → Unbind → Discharge → Wallet → Extract → USDC
```

**[Learn more](players/OPERATIONS.md)**

### Key Concepts

| Term | Description | Learn More |
|------|-------------|------------|
| **Gluon** | In-game currency (1:1 with USDC) | [GLOSSARY.md](GLOSSARY.md#gluon) |
| **Charge** | Entity that enters the board | [GLOSSARY.md](GLOSSARY.md#charge) |
| **Element** | Board region (26 total, H through Fe) | [CONCEPTS.md](players/CONCEPTS.md#the-board) |
| **Saturation** | How "full" an Element is (resets at 100%) | [CONCEPTS.md](players/CONCEPTS.md#saturation) |
| **Reset** | Element clears; pot splits among bound Charges | [CONCEPTS.md](players/CONCEPTS.md#the-reset-cycle) |

### Quick Numbers

**Fees:**
- Movement formula: `balance × distance² × saturation / DENOMINATOR`
- Speed decay: 128× to 1× over ~51 seconds
- Early binding: ~12× more efficient than late (at 10% vs 90% saturation)

**[See:** [REFERENCE.md](players/REFERENCE.md) for complete formulas and constants]

---

## 📋 Document Status

| Document | Last Updated | Status |
|----------|--------------|--------|
| QUICKSTART.md | 2026-01-13 | ✅ Current |
| CONCEPTS.md | 2026-01-13 | ✅ Current |
| OPERATIONS.md | 2026-01-13 | ✅ Current |
| STRATEGY.md | 2026-01-13 | ✅ Current |
| REFERENCE.md | 2026-01-13 | ✅ Current |
| GLOSSARY.md | 2026-01-13 | ✅ Current |

---

## 🔗 External Resources

- **Solana Documentation**: https://docs.solana.com/
- **Rust**: https://www.rust-lang.org/

---

**TOKAMAK64** — A fully deterministic, transparent strategy game on Solana.

*All information is visible. All outcomes are calculable. No randomness. No hidden state.*
