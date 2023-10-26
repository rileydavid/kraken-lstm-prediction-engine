CREATE TABLE IF NOT EXISTS kraken_trade (
    time TIMESTAMPTZ NOT NULL,
    price NUMERIC NOT NULL,  
    volume NUMERIC NOT NULL, 
    side TEXT,
    order_type TEXT,
    symbol_id SERIAL NOT NULL
);

CREATE TABLE IF NOT EXISTS symbols (
    id SERIAL PRIMARY KEY,
    symbol TEXT NOT NULL 
);

-- index for symbols
CREATE INDEX idx_symbols_symbol ON symbols (symbol);

CREATE OR REPLACE FUNCTION get_trades (
    input_interval INTEGER, -- in minutes
    input_symbol TEXT 
)
RETURNS TABLE (
    time_ TIMESTAMPTZ,
    price NUMERIC,
    volume NUMERIC, 
    side TEXT,
    order_type TEXT
)
LANGUAGE plpgsql    
AS $$
DECLARE 
symbol_id_result INTEGER;
interval_duration INTERVAL; 
BEGIN
   -- fill in here
    SELECT id INTO symbol_id_result FROM symbols WHERE symbol = input_symbol;
	
	interval_duration := input_interval * INTERVAL '1 minute';
	
    RETURN QUERY SELECT 
		kraken_trade.time,
		kraken_trade.price,
		kraken_trade.volume,
		kraken_trade.side,
		kraken_trade.order_type
	FROM kraken_trade WHERE kraken_trade.symbol_id = symbol_id_result 
	AND kraken_trade.time >= NOW() - interval_duration
	ORDER BY kraken_trade.time; 
END;$$

-- Example
-- SELECT * FROM get_trades (15, 'ETHUSD');

CREATE OR REPLACE PROCEDURE insert_trade (
    _time TIMESTAMPTZ,
    _price NUMERIC,  
    _volume NUMERIC, 
    _side TEXT,
    _order_type TEXT,
    _symbol TEXT
)
LANGUAGE plpgsql    
AS $$
DECLARE symbol_result INTEGER;
BEGIN
    SELECT id INTO symbol_result FROM symbols WHERE symbol = _symbol;

    IF symbol_result IS NULL THEN
        INSERT INTO symbols (symbol) VALUES (_symbol) RETURNING id INTO symbol_result;
    END IF;

    INSERT INTO kraken_trade (time, price, volume, side, order_type, symbol_id) 
    VALUES (_time, _price, _volume, _side, _order_type, symbol_result);

    COMMIT;
END;$$





/*
Procedure Test 

-- Sample data for testing
DO $$
DECLARE
    sample_time TIMESTAMPTZ := '2023-10-18 12:00:00+00';
    sample_price NUMERIC := 100.0;
    sample_volume NUMERIC := 1.0;
    sample_side TEXT := 'buy';
    sample_order_type TEXT := 'limit';
    sample_symbol TEXT := 'XBTUSD'; -- Replace with an existing symbol in your 'symbols' table
BEGIN
    CALL insert_trade(sample_time, sample_price, sample_volume, sample_side, sample_order_type, sample_symbol);
    RAISE NOTICE 'Trade inserted successfully.';
END;
$$;

insert into kraken_trade (time, price, volume, side, order_type, symbol) values ($1, $2, $3, $4, $5, $6)

*/

/*
CREATE TABLE IF NOT EXISTS kraken_trade_prediction (
    time TIMESTAMPTZ NOT NULL,
    price NUMERIC NOT NULL,  
    volume NUMERIC NOT NULL, 
    side TEXT,
    order_type TEXT,
    symbol TEXT NOT NULL 
);
*/






-- ALTER TABLE kraken_trade  SET UNLOGGED;


 -- CREATE INDEX idx_kraken_symbol_time ON kraken_trade (symbol, time DESC);

 -- SELECT create_hypertable('kraken_trade', 'time', migrate_data => true);
-- SELECT create_hypertable('kraken_trade_prediction', 'time', migrate_data => true);
-- one minute candles 


-- Grafana select to show candle 
--SELECT * FROM kraken_candlestick_day WHERE symbol = 'ETHUSD' AND bucket >= NOW() - INTERVAL '2 years' ORDER BY bucket;

--CREATE TABLE IF NOT EXISTS ethusd_spread ()
--CREATE TABLE IF NOT EXISTS ethusd_book ()

-- improve copy performance by setting the table to unlogged
--ALTER TABLE ethusd_ohlcvt_1  SET UNLOGGED;
-- ALTER TABLE ethusd_trade SET UNLOGGED;

-- copy historical data from csv
-- COPY ethusd_ohlcvt_1 (time, open, high, low, close, volume, trades)
-- FROM '/import/ethusd_ohlcvt_1.csv'
-- DELIMITER ','
-- CSV HEADER;

-- Set Logged again 
--ALTER TABLE ethusd_ohlcvt_1  SET LOGGED;
--ALTER TABLE ethusd_trade  SET LOGGED;

-- set up hypertable
--SELECT create_hypertable('ethusd_ohlcvt_1', 'time', migrate_data => true);
--SELECT create_hypertable('ethusd_trade', 'time', migrate_data => true);


-- update this to use the kraken_trade table and also group by symbol 
-- create view for one minute candles 
--CREATE MATERIALIZED VIEW one_min_candle
--WITH (timescaledb.continuous) AS
--    SELECT
--        time_bucket('1 min', time) AS bucket,
--        FIRST(price, time) AS "open",
--        MAX(price) AS high,
--        MIN(price) AS low,
--        LAST(price, time) AS "close"
--    FROM ethusd_trade
--    GROUP BY bucket


--SELECT add_continuous_aggregate_policy('one_min_candle',
---    start_offset => INTERVAL '3 min',
--    end_offset => INTERVAL '1 min',
--    schedule_interval => INTERVAL '1 min');
-- ethusd historical data table interval 1

--CREATE TABLE IF NOT EXISTS ethusd_ohlcvt_1 (
--    time TIMESTAMP NOT NULL,
--    open DOUBLE PRECISION,
--    high DOUBLE PRECISION,
--    low DOUBLE PRECISION,
--    close DOUBLE PRECISION,
--    volume DOUBLE PRECISION,
--    trades INTEGER
--);

--CREATE TABLE IF NOT EXISTS ethusd_trade (
--    time TIMESTAMP NOT NULL,
--    price DOUBLE PRECISION, 
--    volume DOUBLE PRECISION,
--    side VARCHAR(1),
--    orderType VARCHAR(1)
--);
