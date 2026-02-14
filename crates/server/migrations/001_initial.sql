-- Create extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- NFT Metadata table
CREATE TABLE IF NOT EXISTS nft_metadata (
    token_address BYTEA NOT NULL,
    token_id BYTEA NOT NULL,
    metadata JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    PRIMARY KEY (token_address, token_id)
);

-- Wallet Balances cache table
CREATE TABLE IF NOT EXISTS wallet_balances (
    address BYTEA NOT NULL,
    balance BYTEA NOT NULL,
    chain_id BIGINT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    PRIMARY KEY (address, chain_id)
);

-- API Logs table
CREATE TABLE IF NOT EXISTS api_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    endpoint VARCHAR(255) NOT NULL,
    address BYTEA,
    response_time_ms BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- NFT Transfers table
CREATE TABLE IF NOT EXISTS nft_transfers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token_address BYTEA NOT NULL,
    from_address BYTEA NOT NULL,
    to_address BYTEA NOT NULL,
    token_id BYTEA NOT NULL,
    transaction_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- ERC20 Transfers table
CREATE TABLE IF NOT EXISTS erc20_transfers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token_address BYTEA NOT NULL,
    from_address BYTEA NOT NULL,
    to_address BYTEA NOT NULL,
    value BYTEA NOT NULL,
    transaction_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    hash BYTEA NOT NULL UNIQUE,
    from_address BYTEA NOT NULL,
    to_address BYTEA,
    value BYTEA NOT NULL,
    gas_used BYTEA NOT NULL,
    gas_price BYTEA,
    block_number BIGINT NOT NULL,
    block_hash BYTEA NOT NULL,
    transaction_index BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for better performance
CREATE INDEX IF NOT EXISTS idx_nft_metadata_token_address ON nft_metadata(token_address);
CREATE INDEX IF NOT EXISTS idx_wallet_balances_address ON wallet_balances(address);
CREATE INDEX IF NOT EXISTS idx_api_logs_endpoint ON api_logs(endpoint);
CREATE INDEX IF NOT EXISTS idx_api_logs_created_at ON api_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_nft_transfers_from_address ON nft_transfers(from_address);
CREATE INDEX IF NOT EXISTS idx_nft_transfers_to_address ON nft_transfers(to_address);
CREATE INDEX IF NOT EXISTS idx_erc20_transfers_from_address ON erc20_transfers(from_address);
CREATE INDEX IF NOT EXISTS idx_erc20_transfers_to_address ON erc20_transfers(to_address);
CREATE INDEX IF NOT EXISTS idx_transactions_from_address ON transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_transactions_to_address ON transactions(to_address);
