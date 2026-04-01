build: clean
	cargo build --release && cp target/release/passive ./passive
clean:
	clear
