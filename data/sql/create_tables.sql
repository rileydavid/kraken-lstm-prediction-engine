
-- import historical data missing here
-- it might be possible to convert the epoch ms field into a date using https://stackoverflow.com/questions/16609722/postgresql-how-to-convert-from-unix-epoch-to-date
-- issue symbol this might require a look up similar to insert

-- FIRST DRAFT UNTESTED

-- Create the procedure
/*
CREATE OR REPLACE FUNCTION insert_data_from_csv(csv_filename text, symbol text) RETURNS VOID AS $$
DECLARE
    symbol_id integer;
BEGIN
    -- Check if the symbol already exists in the symbols table
    SELECT id INTO symbol_id FROM symbols WHERE symbol = symbol;
    
    -- If the symbol doesn't exist, insert it into the symbols table
    IF symbol_id IS NULL THEN
        INSERT INTO symbols (symbol) VALUES (symbol) RETURNING id INTO symbol_id;
    END IF;

    -- Read and insert data from the CSV file
    COPY kraken_trade (time, price, volume, side, order_type, symbol_id)
    FROM csv_filename WITH CSV;

END;
$$ LANGUAGE plpgsql;
*/


-- not sure if it is a good idea to use NUMERIC instead of double precision 
-- https://stackoverflow.com/questions/20884405/is-there-a-performance-hit-using-decimal-data-types-mysql-postgres
CREATE TABLE IF NOT EXISTS kraken_trade (
    time TIMESTAMPTZ NOT NULL,
    price NUMERIC NOT NULL,  
    volume NUMERIC NOT NULL, 
    side TEXT,
    order_type TEXT,
    symbol_id SERIAL NOT NULL
);

-- hyper table
SELECT create_hypertable('kraken_trade', 'time', migrate_data => true);

CREATE TABLE IF NOT EXISTS symbols (
    id SERIAL PRIMARY KEY,
    symbol TEXT NOT NULL 
);

-- index for symbols
CREATE INDEX idx_symbols_symbol ON symbols (symbol);

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
END;$$;

CREATE OR REPLACE FUNCTION get_trades (
    input_interval INTEGER, -- in minutes
    input_symbol TEXT -- all returns all symbols
)
RETURNS TABLE (
    time_ TIMESTAMPTZ,
    price NUMERIC,
    volume NUMERIC, 
    side TEXT,
    order_type TEXT, 
    symbol TEXT
)
LANGUAGE plpgsql    
AS $$
DECLARE 
symbol_id_result INTEGER;
interval_duration INTERVAL; 
BEGIN
    interval_duration := input_interval * INTERVAL '1 minute';

    IF input_symbol = 'all' THEN
        RETURN QUERY SELECT 
            kraken_trade.time,
            kraken_trade.price,
            kraken_trade.volume,
            kraken_trade.side,
            kraken_trade.order_type, 
            _symbol.symbol AS symbol
        FROM kraken_trade 
        JOIN symbols AS _symbol ON kraken_trade.symbol_id = _symbol.id 
        AND kraken_trade.time >= NOW() - interval_duration
        ORDER BY kraken_trade.time; 
    ELSE 
        SELECT id INTO symbol_id_result FROM symbols WHERE symbols.symbol = input_symbol;
        
        RETURN QUERY SELECT 
            kraken_trade.time,
            kraken_trade.price,
            kraken_trade.volume,
            kraken_trade.side,
            kraken_trade.order_type, 
            _symbol.symbol AS symbol
        FROM kraken_trade 
        JOIN symbols AS _symbol ON kraken_trade.symbol_id = _symbol.id 
        WHERE kraken_trade.symbol_id = symbol_id_result 
        AND kraken_trade.time >= NOW() - interval_duration
        ORDER BY kraken_trade.time; 
    END IF;
END;$$;

-- Example: SELECT * FROM get_trades (15, 'ETHUSD');


-- create ohlc (open, high, low, close) table for hour
CREATE MATERIALIZED VIEW kraken_ohlc_hour
WITH (timescaledb.continuous) AS
    SELECT
        time_bucket('1 hour', time) AS bucket,
        symbol_id,
        FIRST(price, time) AS open_price,
        MAX(price) AS high, 
        MIN(price) AS low,
        LAST(price, time) AS close_price,
        SUM(volume) AS volume
    FROM kraken_trade
GROUP BY bucket, symbol_id;

SELECT add_continuous_aggregate_policy('kraken_ohlc_hour',
    start_offset => INTERVAL '3 hour',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');


CREATE OR REPLACE FUNCTION get_ohlc_hour (
    input_interval INTEGER, -- in days / all for all symbols
    input_symbol TEXT 
)
RETURNS TABLE (
    bucket TIMESTAMPTZ,
    open_price NUMERIC,
    high NUMERIC,
    low NUMERIC,
    close_price NUMERIC,
    volume NUMERIC,
    symbol TEXT
)
LANGUAGE plpgsql    
AS $$
DECLARE 
symbol_id_result INTEGER;
interval_duration INTERVAL; 
BEGIN
    interval_duration := input_interval * INTERVAL '1 day';
    
    IF input_symbol = 'all' THEN
        RETURN QUERY SELECT 
            kraken_ohlc_hour.bucket,
            kraken_ohlc_hour.open_price,
            kraken_ohlc_hour.high,
            kraken_ohlc_hour.low,
            kraken_ohlc_hour.close_price,
            kraken_ohlc_hour.volume,
            _symbol.symbol AS symbol
        FROM kraken_ohlc_hour
        JOIN symbols AS _symbol ON kraken_ohlc_hour.symbol_id = _symbol.id 
        WHERE kraken_ohlc_hour.bucket >= NOW() - interval_duration
        ORDER BY kraken_ohlc_hour.bucket;

    ELSE
        SELECT id INTO symbol_id_result FROM symbols WHERE symbols.symbol = input_symbol;
	
        RETURN QUERY SELECT 
            kraken_ohlc_hour.bucket,
            kraken_ohlc_hour.open_price,
            kraken_ohlc_hour.high,
            kraken_ohlc_hour.low,
            kraken_ohlc_hour.close_price,
            kraken_ohlc_hour.volume,
            _symbol.symbol AS symbol
        FROM kraken_ohlc_hour
        JOIN symbols AS _symbol ON kraken_ohlc_hour.symbol_id = _symbol.id 
        WHERE kraken_ohlc_hour.symbol_id = symbol_id_result 
        AND kraken_ohlc_hour.bucket >= NOW() - interval_duration
        ORDER BY kraken_ohlc_hour.bucket; 
    END IF;
END;$$;

-- Example Query: SELECT * FROM get_ohlc_hour (15, 'ETHUSD');

-- create ohlc (open, high, low, close) table for day
CREATE MATERIALIZED VIEW kraken_ohlc_day 
WITH (timescaledb.continuous) AS
    SELECT
        time_bucket('1 day', time) AS bucket,
        symbol_id,
        FIRST(price, time) AS open_price,
        MAX(price) AS high, 
        MIN(price) AS low,
        LAST(price, time) AS close_price,
        SUM(volume) AS volume
    FROM kraken_trade
GROUP BY bucket, symbol_id;

SELECT add_continuous_aggregate_policy('kraken_ohlc_day',
    start_offset => INTERVAL '3 day',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');

CREATE OR REPLACE FUNCTION get_ohlc_day (
    input_interval INTEGER, -- in days / all for all symbols
    input_symbol TEXT 
)
RETURNS TABLE (
    bucket TIMESTAMPTZ,
    open_price NUMERIC,
    high NUMERIC,
    low NUMERIC,
    close_price NUMERIC,
    volume NUMERIC,
    symbol TEXT
)
LANGUAGE plpgsql    
AS $$
DECLARE 
symbol_id_result INTEGER;
interval_duration INTERVAL; 
BEGIN
    interval_duration := input_interval * INTERVAL '1 day';
    
    IF input_symbol = 'all' THEN
        RETURN QUERY SELECT 
            kraken_ohlc_day.bucket,
            kraken_ohlc_day.open_price,
            kraken_ohlc_day.high,
            kraken_ohlc_day.low,
            kraken_ohlc_day.close_price,
            kraken_ohlc_day.volume,
            _symbol.symbol AS symbol
        FROM kraken_ohlc_day
        JOIN symbols AS _symbol ON kraken_ohlc_day.symbol_id = _symbol.id 
        WHERE kraken_ohlc_day.bucket >= NOW() - interval_duration
        ORDER BY kraken_ohlc_day.bucket;

    ELSE
        SELECT id INTO symbol_id_result FROM symbols WHERE symbols.symbol = input_symbol;
	
        RETURN QUERY SELECT 
            kraken_ohlc_day.bucket,
            kraken_ohlc_day.open_price,
            kraken_ohlc_day.high,
            kraken_ohlc_day.low,
            kraken_ohlc_day.close_price,
            kraken_ohlc_day.volume,
            _symbol.symbol AS symbol
        FROM kraken_ohlc_day
        JOIN symbols AS _symbol ON kraken_ohlc_day.symbol_id = _symbol.id 
        WHERE kraken_ohlc_day.symbol_id = symbol_id_result 
        AND kraken_ohlc_day.bucket >= NOW() - interval_duration
        ORDER BY kraken_ohlc_day.bucket; 
    END IF;
END;$$;


/*


CREATE OR REPLACE FUNCTION get_all_trades (
    input_interval INTEGER -- in minutes
)
RETURNS TABLE (
    time_ TIMESTAMPTZ,
    price NUMERIC,
    volume NUMERIC, 
    side TEXT,
    order_type TEXT,
    symbol TEXT
)
LANGUAGE plpgsql    
AS $$
DECLARE 
interval_duration INTERVAL; 
BEGIN
	interval_duration := input_interval * INTERVAL '1 minute';
    -- get all (all symbols) trades from a give interval 
    RETURN QUERY SELECT kraken_trade.time,
        kraken_trade.price,
        kraken_trade.volume,
        kraken_trade.side,
        kraken_trade.order_type,
        _symbol.symbol
    FROM kraken_trade
    JOIN symbols AS _symbol ON kraken_trade.symbol_id = _symbol.id 
    WHERE kraken_trade.time >= NOW() - interval_duration
    ORDER BY kraken_trade.time;
END;$$
-- Example: SELECT * FROM get_all_trades (15);

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
*/




-- Example Query: SELECT * FROM get_ohlc_day (15, 'ETHUSD');

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



--CREATE MATERIALIZED VIEW price_one_hour_4
--WITH (timescaledb.continuous)
--AS SELECT symbol_id,
--    time_bucket('1 hour'::interval, time) as bucket,
--    stats_agg(price)
--FROM kraken_trade
--WHERE symbol_id = 4
--GROUP BY bucket, symbol_id;

-- not sure if this produces good results as it takes the last3 value but might use all
-- symbols 
--SELECT 
--    bucket,
--    average(rolling(stats_agg) OVER last3), 
--    sum(rolling(stats_agg) OVER last3)
--FROM price_one_hour
--WHERE symbol_id = 4
--WINDOW last3 as 
--(ORDER BY bucket RANGE '3 hours' PRECEDING);


-- downsample trade price values to 1 hour aggregates
--SELECT time_bucket('1 hour'::interval, time) as bucket,  
--	average(stats_agg(price)), 
--	stddev(stats_agg(price)), 
--    kraken_trade.symbol_id
--FROM kraken_trade
--GROUP BY bucket, symbol_id;

-- downsample trade price values to 24 hour aggregates 
--SELECT time_bucket('24 hour'::interval, time) as bucket,  
--	average(stats_agg(response_time)), 
--	stddev(stats_agg(response_time)),
--    kraken_trade.symbol_id
--FROM price
--GROUP BY bucket, symbol_id;

--CREATE MATERIALIZED VIEW price_one_hour_agg
--WITH (timescaledb.continuous)
--AS SELECT symbol_id,
--    time_bucket('1 hour'::interval, time) as bucket,
--    stats_agg(price)
--FROM kraken_trade
--GROUP BY 1, 2;

--SELECT bucket, 
--	average(rolling(stats_agg) OVER last30), 
--	stddev(rolling(stats_agg) OVER last30)
--FROM response_times_five_min
--WHERE api_id = 32
--WINDOW last30 as 
--(ORDER BY bucket RANGE '30 min' PRECEDING);


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
