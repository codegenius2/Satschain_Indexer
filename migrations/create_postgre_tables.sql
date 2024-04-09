-- Creating the logs table
CREATE TABLE blocks (
    base_block_reward numeric,          -- PostgreSQL doesn't have UInt256; use numeric for arbitrary precision
    base_fee_per_gas bigint,            -- Using bigint as a substitute for Nullable(UInt64)
    burned numeric,                     -- Again, using numeric for arbitrary precision
    chain bigint,                       -- BigInt to accommodate large numbers
    difficulty numeric,                 -- Using numeric for arbitrary precision
    extra_data text,                    -- Assuming text will suffice; no CODEC in PostgreSQL
    gas_limit integer,                  -- UInt32 can be represented as integer
    gas_used integer,                   -- UInt32 can be represented as integer
    hash text,                          -- String is equivalent to text in PostgreSQL
    is_uncle boolean,                   -- Boolean remains the same
    logs_bloom text,                    -- Text for string, ignoring CODEC
    miner text,                         -- String is equivalent to text
    mix_hash text,                      -- Nullable translates to just text as text can be null by default
    nonce text,                         -- String is equivalent to text
    number integer,                     -- UInt32 as integer
    parent_hash text,                   -- String as text
    receipts_root text,                 -- String as text
    sha3_uncles text,                   -- String as text
    size integer,                       -- UInt32 as integer
    state_root text,                    -- String as text
    timestamp timestamp,                -- DateTime as timestamp
    total_difficulty numeric,           -- Nullable UInt256 as numeric
    total_fee_reward numeric,           -- UInt256 as numeric
    transactions smallint,              -- UInt16 as smallint
    transactions_root text,             -- String as text
    uncle_rewards numeric,              -- UInt256 as numeric
    uncles text[],                      -- Array of strings as text[]
    withdrawals_root text               -- Nullable string as text
) PARTITION BY RANGE (timestamp);   

CREATE TABLE satschain.contracts (
    block_number integer,          -- UInt32 can be represented as integer in PostgreSQL
    chain bigint,                  -- UInt64 can be represented as bigint in PostgreSQL
    contract_address text,         -- String in ClickHouse is equivalent to text in PostgreSQL
    creator text,                  -- String to text
    transaction_hash text          -- String to text
);

CREATE TABLE logs (
  address text,
  block_number integer,
  chain bigint,
  data text,
  log_index smallint,
  log_type text,
  removed boolean,
  timestamp timestamp,
  topic0 text,
  topic1 text,
  topic2 text,
  topic3 text,
  transaction_hash text,
  transaction_log_index smallint
) PARTITION BY RANGE (timestamp);

CREATE TABLE satschain.erc20_transfers (
    address text,                  -- String maps to text in PostgreSQL
    amount numeric,                -- UInt256 is not directly supported, numeric is used for arbitrary precision
    block_number integer,          -- UInt32 maps to integer
    chain bigint,                  -- UInt64 maps to bigint
    "from" text,                   -- 'from' is a reserved keyword in PostgreSQL, using quotes to specify it as a column name
    log_index smallint,            -- UInt16 maps to smallint
    log_type text,                 -- Nullable(String) translates to text, since text can be NULL by default
    removed boolean,               -- Boolean remains the same
    timestamp timestamp,           -- DateTime maps to timestamp
    "to" text,                     -- 'to' is a reserved keyword in PostgreSQL, using quotes to specify it as a column name
    token_address text,            -- String maps to text
    transaction_hash text,         -- String maps to text
    transaction_log_index smallint -- Nullable(UInt16) maps to smallint, can be NULL by default
);

CREATE TABLE satschain.erc721_transfers (
    address text,                  -- String maps to text in PostgreSQL
    block_number integer,          -- UInt32 maps to integer
    chain bigint,                  -- UInt64 maps to bigint
    "from" text,                   -- 'from' is a reserved keyword in PostgreSQL, using quotes to specify it as a column name
    id numeric,                    -- UInt256 is not directly supported, numeric is used for arbitrary precision
    log_index smallint,            -- UInt16 maps to smallint
    log_type text,                 -- Nullable(String) translates to text, since text can be NULL by default
    removed boolean,               -- Boolean remains the same
    timestamp timestamp,           -- DateTime maps to timestamp
    "to" text,                     -- 'to' is a reserved keyword in PostgreSQL, using quotes to specify it as a column name
    token_address text,            -- String maps to text
    transaction_hash text,         -- String maps to text
    transaction_log_index smallint -- Nullable(UInt16) maps to smallint, can be NULL by default
);

CREATE TABLE satschain.erc1155_transfers (
  address String,
  amounts Array(UInt256),
  block_number UInt32,
  chain UInt64,
  from String,
  ids Array(UInt256),
  log_index UInt16,
  log_type Nullable(String),
  operator String,
  removed Boolean,
  timestamp DateTime,
  to String,
  token_address String,
  transaction_hash String,
  transaction_log_index Nullable(UInt16)
)

CREATE TABLE satschain.dex_trades (
    address text,                  -- String maps to text in PostgreSQL
    block_number integer,          -- UInt32 maps to integer
    chain bigint,                  -- UInt64 maps to bigint
    log_index smallint,            -- UInt16 maps to smallint
    log_type text,                 -- Nullable(String) translates to text, since text can be null by default
    maker text,                    -- String maps to text
    pair text,                     -- String maps to text
    receiver text,                 -- String maps to text
    removed boolean,               -- Boolean remains the same
    timestamp timestamp,           -- DateTime maps to timestamp
    token0_amount numeric,         -- UInt256 is not directly supported, numeric is used for arbitrary precision
    token1_amount numeric,         -- As above, numeric for large or arbitrary precision values
    transaction_hash text,         -- String maps to text
    transaction_log_index smallint -- Nullable(UInt16) maps to smallint, can be NULL by default
);

CREATE TABLE satschain.traces (
    action_type action_type,                   -- Using ENUM type created above
    address text,                              -- Nullable by default in PostgreSQL
    author text,                               -- Nullable by default
    balance numeric,                           -- Using numeric to handle large values, nullable by default
    block_hash text,                           -- Non-nullable by default
    block_number integer,                      -- Non-nullable by default
    call_type call_type,                       -- Using ENUM type, nullable by default
    chain bigint,                              -- Non-nullable by default
    code text,                                 -- Nullable by default
    error text,                                -- Nullable by default
    "from" text,                               -- 'from' is a reserved keyword in SQL, quoted to use as a column name, nullable
    gas integer,                               -- Nullable by default
    gas_used integer,                          -- Nullable by default
    init text,                                 -- Nullable by default, originally with compression
    input text,                                -- Nullable by default, originally with compression
    output text,                               -- Nullable by default, originally with compression
    refund_address text,                       -- Nullable by default
    reward_type reward_type,                   -- Using ENUM type, nullable by default
    subtraces smallint,                        -- Non-nullable by default
    "to" text,                                 -- 'to' is a reserved keyword in SQL, quoted to use as a column name, nullable
    trace_address integer[],                   -- Array of integers, nullable by default
    transaction_hash text,                     -- Nullable by default
    transaction_position smallint,             -- Nullable by default
    value numeric                              -- Using numeric to handle large values, nullable by default
);

-- Create ENUM types for status and transaction_type
CREATE TYPE transaction_status AS ENUM ('unknown', 'failure', 'success');
CREATE TYPE transaction_type AS ENUM ('legacy', 'access_list', 'eip_1559');

-- Create a composite type for the access_list. PostgreSQL does not directly support nested collections like Array(Tuple(String, Array(String)))
-- Therefore, a simplification or a redesign might be needed depending on the specific requirements.
CREATE TYPE access_list_element AS (
    key text,
    values text[]
);

-- Create the transactions table
CREATE TABLE satschain.transactions (
    access_list access_list_element[],     -- Array of composite type
    base_fee_per_gas bigint,               -- UInt64, nullable
    block_hash text,
    block_number integer,                  -- UInt32
    burned numeric,                        -- UInt256, using numeric for arbitrary precision, nullable
    chain bigint,                          -- UInt64
    contract_created text,                 -- Nullable
    cumulative_gas_used integer,           -- Nullable
    effective_gas_price numeric,           -- UInt256, nullable
    effective_transaction_fee numeric,     -- UInt256, nullable
    "from" text,                           -- 'from' is a reserved keyword in SQL, quoted to use as a column name
    gas integer,                           -- UInt32
    gas_price numeric,                     -- UInt256, nullable
    gas_used integer,                      -- UInt32
    hash text,
    input text,                            -- Originally with compression, not supported directly in PostgreSQL
    max_fee_per_gas numeric,               -- UInt256, nullable
    max_priority_fee_per_gas numeric,      -- UInt256, nullable
    method text,
    nonce integer,                         -- UInt32
    status transaction_status,             -- Using ENUM type
    timestamp timestamp,                   -- DateTime
    "to" text,                             -- 'to' is a reserved keyword in SQL, quoted to use as a column name
    transaction_index smallint,            -- UInt16
    transaction_type transaction_type,     -- Using ENUM type
    value numeric                          -- UInt256, using numeric for arbitrary precision
);

CREATE TABLE satschain.withdrawals (
    address text,                 -- String maps to text in PostgreSQL
    amount numeric,               -- UInt256 is not directly supported; numeric is used for arbitrary precision
    block_number integer,         -- UInt32 maps to integer
    chain bigint,                 -- UInt64 maps to bigint
    timestamp timestamp,          -- DateTime maps to timestamp
    validator_index integer,      -- UInt32 maps to integer
    withdrawal_index integer      -- UInt32 maps to integer
);