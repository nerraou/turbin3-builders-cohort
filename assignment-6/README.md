# NFT Staking (MPL Core)

A Solana NFT staking program built with Anchor and MPL Core.

## Overview

Users can stake NFTs from a collection, earn rewards over time, and unstake them after a freeze period.

## Features

- Create MPL Core collections
- Mint NFTs
- Stake / unstake NFTs
- Claim rewards without unstaking
- Time-based reward system

## State

Staking data is stored directly on the NFT using MPL Core Attributes:

- `staked`
- `staked_at`
- `last_claimed_at`

## Rewards

Rewards are calculated based on time elapsed since the last claim using `rewards_bps`.

## Freeze Period

Users must wait a configured period before unstaking.

## Stack

- Anchor
- MPL Core
- Token-2022
- LiteSVM (testing)
