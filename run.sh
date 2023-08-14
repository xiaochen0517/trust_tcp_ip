#!/bin/bash
cargo b --release
ext=$?
if [[ $ext -ne 0 ]]; then
	exit $ext
fi
sudo setcap cap_net_admin=eip ./target/release/trust_tcp_ip
./target/release/trust_tcp_ip &
pid=$!
sleep 1
sudo ip addr add 192.168.0.1/24 dev tun0
sleep 1
sudo ip link set tun0 up
trap "kill $pid" INT TERM
wait $pid