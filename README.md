# thegarii
The Graph Arweave Integration Implementation.
This is meant to be run against an arweave node.

## Running an arweave node

Arweave is written in erlang, therefore we need to install erlang first.
Erlang compiler may be packaged in most distro, however installing it is not
very straightforward, or worse we have to install each module of the needed modules
in arweave.

### Installing erlang
Installing erlang from source is more straightforward.

Prerequisites. (Make sure root is used when issuing install commands)
```sh
apt install git curl build-essential cmake pkg-config libssl-dev libsqlite3-dev libgmp-dev
```

```sh
git clone --depth=1 -b maint-24 https://github.com/erlang/otp
cd otp
./configure
make
make install
```

Erlang will then be installed by default in `/usr/local/lib/erlang/bin`
and a shortcut is created in `/usr/local/bin`

### Checkout and compile arweave from source code
```sh
git clone --recursive https://github.com/ArweaveTeam/arweave.git
cd arweave
./rebar3 as prod tar
```

The compiled code will be located in `arweave/_build/prod/rel/arweave/`
Copy the `tar.gz` file into a foler where you intend to put the arweave binary.
```sh
mkdir -p ~/Applications/arweave
cp arweave/_build/prod/rel/arweave/arweave-2.5.1.0.tar.gz ~/Applications/arweave/
cd ~/Applications/arweave
tar -xzvf arweave-2.5.1.0.tar.gz
```
#### Open port 1984 for tcp conection
```sh
ufw allow 1984/tcp
```

### Starting the arweave node is the same as mining

```sh
./bin/start mine \
 mining_addr <arweave_address> \
 peer 188.166.200.45 \
 peer 188.166.192.169 \
 peer 163.47.11.64 \
 peer 139.59.51.59 \
 peer 138.197.232.192 \
 peer 178.62.222.154 \
 peer 51.75.206.225 \
 peer 90.70.52.14
```

Example:
```sh
./bin/start mine \
 mining_addr nKn0ZQET1VcpW6_OdpVOP-Pm6b-_BwagHTg3BtByVkA \
 peer 188.166.200.45 \
 peer 188.166.192.169 \
 peer 163.47.11.64 \
 peer 139.59.51.59 \
 peer 138.197.232.192 \
 peer 178.62.222.154 \
 peer 51.75.206.225 \
 peer 90.70.52.14
 ```

#### Check if the node is running
```sh
curl localhost:1984
```
