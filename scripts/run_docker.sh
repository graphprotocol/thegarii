#!/bin/bash

ifconfig

pwd

echo "hello..."

ls -la .
ls -la /opt/arweave/
ls -la /opt/arweave/bin

epmd -daemon

/opt/arweave/bin/start mine \
 mining_addr nKn0ZQET1VcpW6_OdpVOP-Pm6b-_BwagHTg3BtByVkA \
 peer 188.166.200.45 \
 peer 188.166.192.169 \
 peer 163.47.11.64 \
 peer 139.59.51.59 \
 peer 138.197.232.192 \
 peer 178.62.222.154 \
 peer 51.75.206.225 \
 peer 90.70.52.14
