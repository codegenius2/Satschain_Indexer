build:
	cargo build --release
getnewblock:
	./target/release/satschain-indexer --rpcs "https://rpc.payload.de" --ws "wss://ethereum-rpc.publicnode.com" --new-blocks-only
getnewblockwithparam:
	./target/release/satschain-indexer --rpcs ${rpc} --ws ${ws} --new-blocks-only
syncchain:
	./target/release/satschain-indexer --rpcs "https://rpc.payload.de" --start-block 0 --end-block 18000000 --batch-size 500
syncchainwithparam:
	./target/release/satschain-indexer --rpcs ${rpc} --start-block ${start} --end-block ${end} --batch-size ${batch}
fulldata:
	./target/release/satschain-indexer --rpcs "https://rpc.payload.de" --start-block 0 --batch-size 500
fulldatawithparam:
	./target/release/satschain-indexer --rpcs ${rpc} --start-block ${start} --batch-size 500