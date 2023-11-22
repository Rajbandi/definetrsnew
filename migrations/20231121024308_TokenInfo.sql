-- Add migration script here
-- token_info_migration.sql

CREATE TABLE IF NOT EXISTS token_info (
    contract_address TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL ,
    decimals INTEGER NOT NULL,
    total_supply TEXT NOT NULL,
    owner TEXT,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    is_renounced BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_scam BOOLEAN NOT NULL DEFAULT FALSE,
    is_rug_pull BOOLEAN NOT NULL DEFAULT FALSE,
    is_dump_risk BOOLEAN NOT NULL DEFAULT FALSE,
    previous_contracts INTEGER NOT NULL DEFAULT 0,
    liquidity_pool_address TEXT,
    liqudity_period INTEGER NOT NULL DEFAULT 0,
    initial_liquidity FLOAT NOT NULL DEFAULT 0.0,
    current_liquidity FLOAT NOT NULL DEFAULT 0.0,
    is_liquidy_locked BOOLEAN NOT NULL DEFAULT FALSE,
    locked_liquidity FLOAT NOT NULL DEFAULT 0.0,
    is_tax_modifiable BOOLEAN NOT NULL DEFAULT FALSE,
    sell_tax FLOAT NOT NULL DEFAULT 0.0,
    buy_tax FLOAT NOT NULL DEFAULT 0.0,
    transfer_tax FLOAT NOT NULL DEFAULT 0.0,
    score INTEGER NOT NULL DEFAULT 0,
    holders_count INTEGER NOT NULL DEFAULT 0,
    is_v3 BOOLEAN NOT NULL DEFAULT FALSE,
    retry_count INTEGER NOT NULL DEFAULT 0,
    code TEXT, 
    data TEXT , -- Storing JSON as text
    error TEXT,
    date_created DATETIME NOT NULL,
    date_updated DATETIME 
);
