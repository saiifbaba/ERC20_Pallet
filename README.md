# ERC20 Pallet 

This repository contains a Substrate pallet that implements an ERC20-like token functionality for use in Polkaswap. It serves as a starting point for developers looking to create and integrate custom tokens in their Substrate-based blockchains.

## Overview

The ERC20 pallet provides essential features of the ERC20 token standard, including:

- Token creation and management
- Transfers between accounts
- Balance queries
- Event emission for transfers and approvals

This pallet is built using Substrate's FRAME framework, allowing easy integration into Substrate-based runtimes.

## Features

- **Token Management**: Create and manage fungible tokens.
- **Transfer Functionality**: Allow users to transfer tokens between accounts.
- **Event Emission**: Emit events for transfers, enabling front-end applications and external services to react to token movements.
- **Error Handling**: Includes custom error types for robust error management.

## Getting Started

### Prerequisites

- Install Rust: [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- Install Substrate: Follow the instructions at [Substrate Developer Hub](https://docs.substrate.io/install/)
- Ensure you have the latest version of `cargo` and other dependencies installed.

### Clone the Repository

```bash
git clone https://github.com/saiifbaba/ERC20_pallet.git
cd ERC20_pallet

### Build

Use the following command to build the node without launching it:

```sh
cargo build --package solochain-template-node --release
```

### Embedded Docs

After you build the project, you can use the following command to explore its
parameters and subcommands:

```sh
./target/release/solochain-template-node -h
```


### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the
hosted version of the [Polkadot/Substrate
Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944)
front-end by connecting to the local node endpoint. A hosted version is also
available on [IPFS](https://dotapps.io/). You can
also find the source code and instructions for hosting your own instance in the
[`polkadot-js/apps`](https://github.com/polkadot-js/apps) repository.