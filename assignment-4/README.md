# Anchor Escrow

A simple escrow protocol built with **Anchor** on **Solana**.

This project demonstrates a trustless token exchange flow between two users using PDA-controlled vault custody.

---

## Features

### Make

A maker creates an escrow offer by:

- defining the token they deposit (`mintA`)
- defining the token they want in return (`mintB`)
- locking tokens into a vault PDA
- setting an expiration time

The escrow state is stored on-chain.

---

### Take

A taker accepts the offer by:

- sending the requested tokens to the maker
- receiving locked tokens from the vault
- automatically closing escrow after successful exchange

Validation ensures:

- escrow is active
- escrow is not expired
- token accounts are correct
- PDA authority is valid

---

### Refund

If escrow expires before being accepted, the maker can reclaim locked funds.

This instruction:

- verifies escrow expiration
- transfers tokens back to maker
- closes vault account
- closes escrow state account

---

## Tech Stack

- Rust
- Anchor
- Solana
- SPL Token Program
- TypeScript Tests
- Mocha / Chai

---

## Project Structure

```txt
programs/
  anchor-escrow/
    src/
      instructions/
        make.rs
        take.rs
        refund.rs
      state/
      lib.rs

tests/
```

---

## Run Tests

```bash
anchor test -- --nocapture
```

For full backtraces:

```bash
RUST_BACKTRACE=1 anchor test -- --nocapture
```

---

## Core Concepts Practiced

This project reinforces:

- PDA derivation
- PDA signing
- Cross-program invocations (CPI)
- Token vault custody
- Account constraints
- Time-based state expiration
- Secure account closure
- Multi-party protocol design

---

## Tests Log

![Tests-log](./image/tests.png)
