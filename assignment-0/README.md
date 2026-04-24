# 🛠️ Solana Development Environment Setup

This repository documents the setup of my local Solana development environment as part of the Turbin3 bootcamp assignment.

I followed the official Solana installation guide:
https://solana.com/docs/intro/installation

---

## 📦 Installed Tools

The following dependencies were installed:

- Rust (for writing Solana programs)
- Solana CLI (for interacting with the blockchain)
- Anchor CLI (framework for Solana development)
- Node.js & Yarn (for frontend/testing)

These tools are required to build, test, and deploy Solana programs.

---

## ⚡ Quick Installation

I used the official one-command installer:

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
```

✅ Verification

After installation, I verified all tools:

```bash
rustc --version
solana --version
anchor --version
node --version
yarn --version
```

Successful output confirms the environment is ready.

---

## ⚠️ Issues Encountered

- Encountered snapshot/ledger error when starting validator
- Resolved by resetting local ledger:

```bash
rm -rf test-ledger
solana-test-validator
```
