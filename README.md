# thegarii

[<img alt="github" src="https://img.shields.io/badge/github-ChainSafe/thegarii-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height = "20" />](https://github.com/ChainSafe/thegarii)
[<img alt="crates.io" src="https://img.shields.io/crates/v/thegarii?style=for-the-badge" height = "20" />](https://crates.io/crates/thegarii)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/ChainSafe/thegarii/CI/main?style=for-the-badge" height = "20" />](https://github.com/ChainSafe/thegarii/actions?query=branch%3Amain)

The polling service for Arweawve blocks

## Getting Started

```
> cargo install thegarii
> thegarii -h
thegaril 0.0.3
info@chainsafe.io
env arguments for CLI

USAGE:
    thegarii [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -B, --batch-blocks <batch-blocks>    how many blocks polling at one time [default: 20]
    -b, --block-time <block-time>        time cost for producing a new block in arweave [default: 20000]
    -c, --confirms <confirms>            safe blocks against to reorg in polling [default: 20]
    -e, --endpoints <endpoints>...       client endpoints [default: https://arweave.net/]
    -p, --ptr-path <ptr-path>            block ptr file path
    -r, --retry <retry>                  retry times when failed on http requests [default: 10]
    -t, --timeout <timeout>              timeout of http requests [default: 120000]
```


# Environments
    
| KEY          | DEFAULT_VALUE          | DESCRIPTION                                 |
|--------------|------------------------|---------------------------------------------|
| ENDPOINTS    | "https://arweave.net"  | for multiple endpoints, split them with ',' |
| BATCH_BLOCKS | 50                     | how many blocks batch at one time           |
| CONFIRMS     | 20                     | irreversibility condition                   |
| PTR_PATH     | $APP_DATA/thegarii/ptr | the file stores the block ptr for polling   |
| retry        | 10                     | retry times when failed on http requests    |
| timeout      | 120_000                | timeout of http requests                    |


## Dev

Build the source code with `cargo build --release --features full`.

To config the number of nodes to pull blocks from, define the env variable: `ENDPOINTS`, i.e. `export ENDPOINTS=http://178.62.222.154:1984,http://localhost:1984`.
The default node is `https://arweave.net/`.

To start estimating the total ingestion time using the following command:
```shell
./target/release/thegarii poll -h
```
