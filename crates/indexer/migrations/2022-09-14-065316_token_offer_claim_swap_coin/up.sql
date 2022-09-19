-- Your SQL goes here
-- tracks all token activities
CREATE TABLE token_activities (
  -- TOKEN ID (with name and collection hashed)
  creator_address VARCHAR(66) NOT NULL,
  collection_name_hash VARCHAR(64) NOT NULL,
  name_hash VARCHAR(64) NOT NULL,
  property_version NUMERIC NOT NULL,
  event_key VARCHAR(100) NOT NULL,
  event_sequence_number BIGINT NOT NULL,
  collection_name TEXT NOT NULL,
  name TEXT NOT NULL,
  transaction_version BIGINT NOT NULL,
  transfer_type VARCHAR(50) NOT NULL,
  from_address VARCHAR(66),
  to_address VARCHAR(66),
  amount NUMERIC NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (
    -- To guarantee uniqueness we need token_id + transaction_version + event_index
    creator_address,
    collection_name_hash,
    name_hash,
    property_version,
    event_key,
    event_sequence_number
  )
);
CREATE INDEX ta_from_index ON token_activities (from_address);
CREATE INDEX ta_to_index ON token_activities (to_address);
CREATE INDEX ta_insat_index ON token_activities (inserted_at);
-- Tracks current pending claims
CREATE TABLE current_token_pending_claims (
  -- TOKEN ID (with name and collection hashed)
  creator_address VARCHAR(66) NOT NULL,
  collection_name_hash VARCHAR(64) NOT NULL,
  name_hash VARCHAR(64) NOT NULL,
  property_version NUMERIC NOT NULL,
  from_address VARCHAR(66) NOT NULL,
  to_address VARCHAR(66) NOT NULL,
  collection_name TEXT NOT NULL,
  name TEXT NOT NULL,
  latest_transaction_version BIGINT NOT NULL,
  -- 0 means either claimed or canceled
  amount NUMERIC NOT NULL,
  last_updated TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (
    -- This is the token offer id
    creator_address,
    collection_name_hash,
    name_hash,
    property_version,
    from_address,
    to_address
  )
);
CREATE INDEX ctpc_from_index ON current_token_pending_claims (from_address);
CREATE INDEX ctpc_to_index ON current_token_pending_claims (to_address);
-- Tracks current token listings
CREATE TABLE current_token_listings (
  -- TOKEN ID (with name and collection hashed)
  creator_address VARCHAR(66) NOT NULL,
  collection_name_hash VARCHAR(64) NOT NULL,
  name_hash VARCHAR(64) NOT NULL,
  property_version NUMERIC NOT NULL,
  lister_address VARCHAR(66) NOT NULL,
  collection_name TEXT NOT NULL,
  name TEXT NOT NULL,
  latest_transaction_version BIGINT NOT NULL,
  coin_type VARCHAR(66) NOT NULL,
  min_price_per_token NUMERIC NOT NULL,
  -- 0 means either claimed or canceled
  amount NUMERIC NOT NULL,
  last_updated TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (
    -- This is the token offer id
    creator_address,
    collection_name_hash,
    name_hash,
    property_version,
    lister_address
  )
);
CREATE INDEX ctl_lister_index ON current_token_listings (lister_address);