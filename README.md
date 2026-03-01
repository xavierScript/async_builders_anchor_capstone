# Async Builders Anchor Capstone — inQrio (DevNet: DE4Q3C4boDBeMobfbaSxETX9EmW4984HbZP9rKfDbMPQ)

This repository is the capstone project for the Async Builders program at the Solana Turbine Institute. It contains an Anchor-based Solana program, TypeScript client bindings, on-chain program IDL, tests, and deployment runbooks.

## Project Overview

`inQrio` is a Solana program written with Anchor that demonstrates async patterns, robust account/state design, and TypeScript integration. It was developed as the capstone for Async Builders at the Solana Turbine Institute.

## Architecture

- Program logic lives in `programs/async_builders_anchor_capstone/src/` and is implemented using Anchor accounts, instructions, and events.
- TypeScript client types are generated under `types/` for convenient on-chain interaction.
- `migrations/` contains scripts to deploy or run migrations.
- `runbooks/` contains deployment transactions and helper artifacts used to set up signers and initial state.

High-level components:

- Accounts: persistent state types in `programs/.../src/state/`
- Instructions: handlers in `programs/.../src/instructions/`
- Errors: custom error definitions in `programs/.../src/errors.rs`

## Prerequisites

- Rust toolchain (nightly or as specified in `rust-toolchain.toml`)
- Anchor CLI (`npm i -g @coral-xyz/anchor-cli` or follow Anchor docs)
- Solana CLI (`solana`)
- Node.js (16+ recommended) and npm/yarn/pnpm
- `cargo`, `rustup` configured

Confirm basic tools:

```bash
rustup show
solana --version
anchor --version
node --version
npm --version
```

## Quick Start

Clone the repo and install dependencies:

```bash
git clone <repo-url>
cd async_builders_anchor_capstone
# (optional) install JS deps for frontend/tests
npm install
```

Set Solana CLI to a network for local development (localnet is the default used by `anchor test`):

```bash
solana config set --url http://127.0.0.1:8899
```

### Build (Anchor)

```bash
# build the Anchor program (produces idl/ and target/ artifacts)
anchor build
```

### Test (Anchor)

Anchor runs a local validator and executes tests written in TypeScript.

If you need to run tests manually with a running local validator:

```bash
# Start a local validator in another terminal
surfpool start
# In this repo
anchor test --skip-local-validator
```

## Recent Deployment (example)

The program was recently deployed; key deployment details are recorded here for convenience.

- **Deploying cluster:** https://api.devnet.solana.com
- **Program name:** async_builders_anchor_capstone
- **Program Id:** DE4Q3C4boDBeMobfbaSxETX9EmW4984HbZP9rKfDbMPQ
- **Signature:** 4ftfnSmvQ94oygg6RuEgzB3o9n1fMLqv2tyt5gU59EE7YWMnLDG9x3qnKkY5Xn8c8RENGZHVw8dtgsWVpEFx39Rx
- **IDL account created:** 3hQdBhSj6ArmRr3JNZVgaiFdokWqynF8hz6xSinA1uyJ
- **IDL data length:** 1858 bytes

You can reference the deployed program id in `Anchor.toml`
