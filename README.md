# Stellar Poll Contract

Stellar Poll Contract is a simple Soroban smart contract for running a yes/no poll on Stellar testnet. It was built as a workshop submission and is intentionally different from the default workshop example: instead of returning a greeting or storing notes, it stores a poll question and counts yes/no votes on-chain.

## Application Description

The application keeps one active poll in Soroban contract instance storage. Users can set a poll question, vote yes, vote no, read the current vote counts, read the total number of votes, and reset the vote counters.

This contract is a learning project for workshop submission. The poll state is public on-chain data, so it should be used for public voting examples, demos, and educational workflows rather than private ballots.

## Features

- Set or update the active poll question
- Vote yes with `vote_yes()`
- Vote no with `vote_no()`
- Read yes vote count
- Read no vote count
- Read total vote count
- Read full poll result in one call
- Reset vote counters while keeping the current question
- Extend instance storage TTL after writes
- Unit tests for question, voting, result, and reset behavior

## Smart Contract ID

Testnet contract ID:

```text
CD4ZRYTEHTLKPGY2ORRRYICGHMRGYVGFCHSZUWX4JQBOJUYDOGUCLEEO
```

Explorer link:

```text
https://lab.stellar.org/r/testnet/contract/CD4ZRYTEHTLKPGY2ORRRYICGHMRGYVGFCHSZUWX4JQBOJUYDOGUCLEEO
```

If you redeploy the contract, replace this value with the new testnet contract ID returned by `stellar contract deploy`.

## Contract Functions

### `set_question(question: String) -> bool`

Sets or updates the active poll question.

### `get_question() -> String`

Returns the active poll question. If no question has been set, it returns `No question set`.

### `vote_yes() -> u32`

Adds one yes vote and returns the new yes vote count.

### `vote_no() -> u32`

Adds one no vote and returns the new no vote count.

### `yes_votes() -> u32`

Returns the current yes vote count.

### `no_votes() -> u32`

Returns the current no vote count.

### `total_votes() -> u32`

Returns `yes_votes + no_votes`.

### `get_result() -> PollResult`

Returns the poll question, yes vote count, no vote count, and total vote count.

### `reset_votes() -> bool`

Resets yes and no vote counts to zero while keeping the current question.

## Poll Result Structure

```rust
pub struct PollResult {
    pub question: String,
    pub yes_votes: u32,
    pub no_votes: u32,
    pub total_votes: u32,
}
```

## Tech Stack

- Stellar Soroban smart contracts
- Rust
- `soroban-sdk` 25.3.0
- Stellar CLI
- Vite frontend
- Stellar JavaScript SDK
- Freighter wallet API

## Build

```bash
stellar contract build
```

The optimized WASM file is generated at:

```text
target/wasm32v1-none/release/stellar_poll_contract.wasm
```

## Test

```bash
cargo test
```

## Frontend

Install frontend dependencies:

```bash
npm install
```

Run the local web app:

```bash
npm run dev
```

The frontend connects to the deployed testnet contract and supports:

- Connect Freighter wallet
- Read the current poll result
- Set or update the poll question
- Vote yes
- Vote no
- Reset vote counters

Use Freighter on `TESTNET` before submitting write transactions.

## Deploy To Testnet

Generate and fund a testnet identity if needed:

```bash
stellar keys generate alice --network testnet --fund
```

Deploy the contract:

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/stellar_poll_contract.wasm \
  --source alice \
  --network testnet
```

## Example Invocations

Set the poll question:

```bash
stellar contract invoke \
  --id CD4ZRYTEHTLKPGY2ORRRYICGHMRGYVGFCHSZUWX4JQBOJUYDOGUCLEEO \
  --source alice \
  --network testnet \
  -- \
  set_question \
  --question "Should Stellar be used for workshop apps?"
```

Vote yes:

```bash
stellar contract invoke \
  --id CD4ZRYTEHTLKPGY2ORRRYICGHMRGYVGFCHSZUWX4JQBOJUYDOGUCLEEO \
  --source alice \
  --network testnet \
  -- \
  vote_yes
```

Vote no:

```bash
stellar contract invoke \
  --id CD4ZRYTEHTLKPGY2ORRRYICGHMRGYVGFCHSZUWX4JQBOJUYDOGUCLEEO \
  --source alice \
  --network testnet \
  -- \
  vote_no
```

Read the full result:

```bash
stellar contract invoke \
  --id CD4ZRYTEHTLKPGY2ORRRYICGHMRGYVGFCHSZUWX4JQBOJUYDOGUCLEEO \
  --source alice \
  --network testnet \
  -- \
  get_result
```

Reset vote counters:

```bash
stellar contract invoke \
  --id CD4ZRYTEHTLKPGY2ORRRYICGHMRGYVGFCHSZUWX4JQBOJUYDOGUCLEEO \
  --source alice \
  --network testnet \
  -- \
  reset_votes
```

## Repository Name

Suggested GitHub repository name:

```text
stellar-poll-contract
```
