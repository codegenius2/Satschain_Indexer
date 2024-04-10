<h1 align="center">
<strong>Satschain Indexer</strong>
</h1>
<p align="center">
<strong>Scalable SQL indexer for Satschain compatible blockchains</strong>
</p>

An indexer is a program that fetches and stores blockchain data for later analysis.

This indexer is specifically created to parse known data for satschain compatible chains.

It stores all the blockchain primitives (blocks, transactions, receipts, logs, traces, withdrawals) and some other useful information (contracts created, dex trades, erc20 transfers, erc721 transfers, erc1155 transfers)

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [ClickHouse](https://clickhouse.com/)

### Installing ClickHouse on Ubuntu
```
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv E0C56BD4
```
You will see output similar to the following:

```
Output
Executing: /tmp/apt-key-gpghome.JkkcKnBAFY/gpg.1.sh --keyserver keyserver.ubuntu.com --recv E0C56BD4

gpg: key C8F1E19FE0C56BD4: public key "ClickHouse Repository Key <milovidov@yandex-team.ru>" imported
gpg: Total number processed: 1
gpg:               imported: 1
```

Add the repository to your APT repositories list by executing:
```
echo "deb http://repo.yandex.ru/clickhouse/deb/stable/ main/" | sudo tee /etc/apt/sources.list.d/clickhouse.list
```

Now, update your packages:
```
sudo apt update
```

The clickhouse-server and clickhouse-client packages will now be available for installation. Install them with:
```
sudo apt install clickhouse-server clickhouse-client
```

### Starting the clickhouse service
Start the clickhouse-server service by running:
```
sudo service clickhouse-server start
```

The previous command will not display any output. To verify that the service is running successfully, execute:
```
sudo service clickhouse-server status
```
You’ll see output similar to the following:

```
Output
● clickhouse-server.service - ClickHouse Server (analytic DBMS for big data)
     Loaded: loaded (/etc/systemd/system/clickhouse-server.service; enabled; vendor preset: enabled)
     Active: active (running) since Wed 2020-09-16 05:18:54 UTC; 5s ago
   Main PID: 2697 (clickhouse-serv)
      Tasks: 46 (limit: 1137)
     Memory: 459.7M
     CGroup: /system.slice/clickhouse-server.service
             └─2697 /usr/bin/clickhouse-server --config=/etc/clickhouse-server/config.xml --pid-file=/run/clickhouse-server/clickhouse-server.pid
```

### Confirm your clickhouse client
start a client session by running the following command:
```
clickhouse-client --password
```
You will be asked to enter the password you had set during the installation—enter it to successfully to start the client session.



## Local

1. Clone the repository

```
git clone git@github.com:satschain/explorerbackend.git && cd explorerbackend
```


2. Copy the `.env.example` file to `.env` and add your environment variables.

```
DB_USER_NAME=your_clickhouse_username           # default
DB_USER_PASSWORD=your_clickhouse_password       # mysecretkey
DB_PORT=your_clickhouse_server_port             # 8123
DB_NAME=satschain                               # if you change this to another name, you need to change dbname of queries in create_tables.sql file so that the db name is equal with your DB_NAME
DB_HOST=your_clickhouse_server_host             # http://localhost
EXPLORER_SERVER_PORT=8300                       # server port for explore frontend, it must be same as frontend's NEXT_PUBLIC_API_PORT
```


3. Run sql queries in `create_tables.sql` on clickhouse

4. Build the program

```
make build
or
cargo build --release
```

5. Run the program

You can get new blocks and start server for explore by using following command
```
make getnewblock
```

You can sync data and start server for explore by using following command
```
make syncchain
```



## Program flags

| Flag            | Default | Purpose                                                |
| --------------- | :-----: | ------------------------------------------------------ |
| `--debug`       |  false  | Start log with debug.                                  |
| `--chain`       |    1    | Number identifying the chain id to sync.               |
| `--start-block` |    0    | Block to start syncing.                                |
| `--end-block`   |    0    | Last block to sync (0 to sync all the blocks).         |
| `--batch-size`  |   200   | Amount of blocks to fetch in parallel.                 |
| `--rpcs`        | `empty` | Comma separated list of rpcs to use to fetch blocks.   |
| `--database`    | `empty` | Clickhouse database string with username and password. |
| `--ws`          | `empty` | Url of the websocket endpoint to fetch new blocks.     |
