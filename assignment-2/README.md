# Solana Kit Token & NFT Project

A learning project built with:

- @solana/kit
- Metaplex
- Solana Devnet

This project was created to deeply understand how Solana transactions work under the hood by manually building:

- SPL Token creation
- Token metadata creation
- Minting tokens
- Token transfers
- NFT creation with Metaplex

Instead of relying on high-level abstractions, this project uses Solana Kit to expose the complete transaction lifecycle:

- RPC setup
- Transaction message creation
- Fee payer assignment
- Blockhash lifetime configuration
- Instruction composition
- Signing
- Sending and confirmation

---

# What I Built

## SPL Token Initialization

Created a custom SPL token from scratch by:

- generating a mint keypair
- allocating mint account storage
- funding rent exemption
- assigning ownership to the Token Program
- initializing mint configuration

Configured:

- decimals
- mint authority
- token program ownership

---

## Token Metadata (Metaplex)

Attached on-chain metadata to the SPL token using Metaplex:

- token name
- symbol
- metadata URI

This makes the token human-readable by wallets and explorers.

---

## Associated Token Accounts (ATA)

Derived and created associated token accounts to hold token balances.

Learned how ATA addresses are deterministic using:

- wallet address
- mint address
- token program

---

## Minting Tokens

Minted token supply into associated token accounts.

Learned the difference between:

- mint account (token definition)
- token account (balance holder)
- wallet owner

---

## Token Transfers

Transferred SPL tokens between token accounts.

Understood how Solana transfers operate between token accounts rather than wallet addresses directly.

---

## NFT Creation with Metaplex

Created NFTs using Metaplex tooling.

Included:

- asset creation
- metadata registration
- devnet minting

---

# Project Structure

```txt
src/
 ├── config/
 │   ├── rpc.ts
 │   └── consts.ts
 │
 ├── wallet/
 │   └── load_wallet.ts
 │
 ├── spl/
 │   ├── init_mint.ts
 │   ├── mint_token.ts
 │   ├── transfer_token.ts
 │   └── metadata.ts
 │
 ├── nft/
 │   ├── nft_image.ts
 │   ├── nft_metadata.ts
 │   └── nft_mint.ts
 │
 ├── utils/
 │   ├── load_json.ts
 │   └── save_json.ts
 │
 └──
```

---

# Key Concepts Learned

## Solana Accounts

Every Solana object is an account with allocated storage.

---

## Rent Exemption

Accounts must hold enough SOL to remain permanently active.

---

## Transaction Messages

Transactions are composed by combining ordered instructions.

---

## Instruction Ordering

Execution order matters.

Example:

1. create account
2. initialize mint
3. mint supply
4. transfer tokens

Reordering breaks execution.

---

## Signers

Transactions require cryptographic approval from required authorities.

Examples:

- fee payer signer
- mint signer
- authority signer

---

## RPC vs Subscriptions

RPC handles requests.

WebSocket subscriptions handle confirmation notifications.

---

# Running the Project

Install dependencies:

```bash
cd assignment-2/spl_and_nft
npm install
```

Available scripts:

```bash
npm run spl:init
npm run spl:metadata
npm run spl:mint
npm run spl:transfer
npm run nft:image
npm run nft:mint
npm run nft:metadata
```

---

# Network

Built and tested on Solana Devnet.

Airdrop SOL:

```bash
solana airdrop 2
```
