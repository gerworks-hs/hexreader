gnu-linux:
	cargo build --release --target=x86_64-unknown-linux-gnu
windows:
	cargo build --release --target=x86_64-pc-windows-gnu
clean:
	rm -rf ./target