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
    info_retry_count INTEGER NOT NULL DEFAULT 0,
    data TEXT , -- Storing JSON as text
    date_created DATETIME NOT NULL,
    date_updated DATETIME 
);
