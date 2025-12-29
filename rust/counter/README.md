# PDA-Based Counter (Solana + Anchor)

A production-style counter program built using **Solana PDAs** and **Anchor**, designed to go beyond a basic tutorial by enforcing ownership, safety, and testability.

---

## ✨ Features

- **PDA-based counters**  
  Each counter is a Program Derived Address, not a random account.

- **Multiple counters per user**  
  Users can create and manage multiple counters using deterministic seeds.

- **Authority check**  
  Only the counter owner can increment or decrement.

- **Cutom errors**  
  Safe arithmetic using checked operations.

- **Event/Emit**  
  Counter updates emit events for indexing and off-chain tracking.
---

## 🧠 What I Learned

- How **PDAs represent program state**, not just storage
- Designing **deterministic seeds** for scalable on-chain data
- Using `has_one` to enforce **account ownership**
- Why **checked math is mandatory** in smart contracts
- How and why to **emit events instead of relying on state reads**
- Writing tests that **try to break the contract**, not just pass

---

## 🧱 Tech Stack

- Solana
- Anchor
- Rust
- TypeScript (Anchor tests)

---

## 🚀 Use Cases

This pattern can be extended to:
- Escrow contracts
- Vaults
- User-owned state
- Order books
- Any multi-account Solana program

---

Built to understand Solana at a **protocol level**, not just “make it compile”.
