-- Add up migration script here
CREATE TABLE IF NOT EXISTS kraken_trade (
    time TIMESTAMPTZ NOT NULL,
    price NUMERIC NOT NULL,  
    volume NUMERIC NOT NULL, 
    side TEXT,
    order_type TEXT,
    symbol TEXT NOT NULL -- One more just in case
);