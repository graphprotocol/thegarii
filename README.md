# thegarii
The Graph Arweave Integration Implementation.
This is meant to be run against an arweave node.

## Dev
Build the source code with `cargo build --release`.

To config the number of nodes to pull blocks from, define the env variable: `ENDPOINTS`, i.e. `export ENDPOINTS=http://178.62.222.154:1984,http://localhost:1984`.
The default node is `https://arweave.net/`.

To start estimating the total ingestion time using the following command:
```shell
./target/release/thegarii poll -h
```

To compile, set env variables and run in one go, you can use:
```shell
ENDPOINTS=http://178.62.222.154:1984,http://localhost:1984 cargo run --release -- poll -h
```
