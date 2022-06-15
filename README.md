# Data Trust Protocol

We build a data trust based on a decentralized identity protocol

## Getting Started

We use substrate as a framwork for the blockchain, at first you should :
 - complete the [basic Rust setup instructions](./docs/rust-setup.md).
 - Complete some tutorial [here](https://docs.substrate.io/)

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```



### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/node-data_trust_protocol --tmp --dev
```

### Use Polkadot Js to interact with Blockchain Node
 - Visit polkadot apps https://polkadot.js.org/apps/#/explorer
 - Go to menu (developement) and switch to localhost

