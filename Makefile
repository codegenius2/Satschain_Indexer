build:
	cargo build --release
getnewblock:
	./target/release/satschain-indexer --rpcs "https://rpc.payload.de" --ws "wss://ethereum-rpc.publicnode.com" --new-blocks-only
syncchain:
	./target/release/satschain-indexer --rpcs "https://rpc.payload.de" --start-block 0 --end-block 18000000 --batch-size 500