#!/bin/bash
set -e

apt install -y git curl build-essential cmake pkg-config libssl-dev libsqlite3-dev libgmp-dev libncurses-dev ncurses-bin

rm -r ~/compilation
mkdir -p ~/compilation/

cd ~/compilation

git clone --depth=1 -b maint-24 https://github.com/erlang/otp
cd ~/compilation/otp
./configure
make
make install


cd ~/compilation
git clone -b N.2.5.1.0 --recursive https://github.com/ArweaveTeam/arweave.git
cd ~/compilation/arweave
./rebar3 as prod tar


mkdir -p ~/Applications/arweave
cp ~/compilation/arweave/_build/prod/rel/arweave/arweave-2.5.1.0.tar.gz ~/Applications/arweave/
cd ~/Applications/arweave
tar -xzvf arweave-2.5.1.0.tar.gz

echo -n "fs.file-max=100000000" >> /etc/sysctl.conf
sysctl -p
echo -n "DefaultLimitNOFILE=10000000" >> /etc/systemd/user.conf
echo -n "DefaultLimitNOFILE=10000000" >> /etc/systemd/system.conf
