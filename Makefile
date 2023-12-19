export RUST_LOG:=INFO

.PHONY: server
server:
	cargo run --bin server

.PHONY: client
client:
	head -c 4096 /dev/urandom > file.bin
	cargo run --bin client -- client3 ./file.bin
