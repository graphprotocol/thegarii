FROM debian:bullseye
RUN apt-get update && apt-get install -y git curl build-essential cmake pkg-config libssl-dev libsqlite3-dev libgmp-dev ncurses-bin libncurses-dev net-tools ufw
WORKDIR /usr/src
RUN git clone --depth=1 -b maint-24 https://github.com/erlang/otp erlang-otp
WORKDIR  /usr/src/erlang-otp
RUN ./configure && make
RUN make install
WORKDIR /usr/src/
RUN git clone -b N.2.5.1.0 --recursive https://github.com/ArweaveTeam/arweave.git
WORKDIR /usr/src/arweave
RUN ./rebar3 as prod tar
WORKDIR /opt/arweave
RUN cp /usr/src/arweave/_build/prod/rel/arweave/arweave-2.5.1.0.tar.gz /opt/arweave
RUN tar -xzvf arweave-2.5.1.0.tar.gz
WORKDIR /opt/

COPY scripts/run_docker.sh /opt/arweave/run.sh
RUN echo -n "fs.file-max=100000000" >> /etc/sysctl.conf
RUN echo -n "DefaultLimitNOFILE=10000000" >> /etc/systemd/user.conf
RUN echo -n "DefaultLimitNOFILE=10000000" >> /etc/systemd/system.conf
EXPOSE 1984
CMD ["/opt/arweave/run.sh"]
