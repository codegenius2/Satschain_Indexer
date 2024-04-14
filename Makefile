build:
	cargo build --release
getnewblock:
	./target/release/satschain-indexer --rpcs "https://rpc.payload.de" --ws "wss://ethereum-rpc.publicnode.com" --new-blocks-only
fullblock:
	./target/release/satschain-indexer --rpcs "https://rpc.payload.de" --ws "wss://ethereum-rpc.publicnode.com" --start-block 0 --end-block 0 --batch-size 30