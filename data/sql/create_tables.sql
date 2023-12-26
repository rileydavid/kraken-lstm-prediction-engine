-- create database for mlflow parameter tracking 
CREATE DATABASE mlflow;

-- https://www.timescale.com/blog/best-practices-for-picking-postgresql-data-types/
-- https://stackoverflow.com/questions/20884405/is-there-a-performance-hit-using-decimal-data-types-mysql-postgres
CREATE TABLE IF NOT EXISTS trade (
    time TIMESTAMPTZ NOT NULL,
    price DOUBLE PRECISION NOT NULL,  
    volume DOUBLE PRECISION NOT NULL, 
    side TEXT,
    order_type TEXT,
    symbol_id SERIAL NOT NULL
);

-- TODO: create index for symbols and time? 

-- hyper table
SELECT create_hypertable('trade', 'time', migrate_data => true);

-- maybe change this use a singular name symbol
CREATE TABLE IF NOT EXISTS symbols (
    id SERIAL PRIMARY KEY,
    symbol TEXT NOT NULL UNIQUE
);

-- index for symbols
CREATE INDEX idx_symbols_symbol ON symbols (symbol);


CREATE OR REPLACE FUNCTION insert_symbol(
    input_symbol text
)
RETURNS TABLE(
    out_id INTEGER, 
    out_symbol TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    -- Try to insert the new symbol
    INSERT INTO symbols (symbol)
    VALUES (input_symbol)
    ON CONFLICT (symbol) DO NOTHING
    RETURNING id INTO out_id;

    -- If the insert did not happen, select the existing row
    IF NOT FOUND THEN
        RETURN QUERY SELECT id, symbol FROM symbols WHERE symbol = input_symbol;
    ELSE
        out_symbol := input_symbol;
        RETURN NEXT;
    END IF;
END;
$$;

-- This seems inefficient --> might be better to keep a table of all symbols and their ids
-- in the backend
CREATE OR REPLACE PROCEDURE insert_trade (
    _time TIMESTAMPTZ,
    _price DOUBLE PRECISION,  
    _volume DOUBLE PRECISION, 
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

    INSERT INTO trade (time, price, volume, side, order_type, symbol_id) 
    VALUES (_time, _price, _volume, _side, _order_type, symbol_result);
END;$$;

-- insert trade 
CREATE OR REPLACE PROCEDURE insert_trade (
    _time TIMESTAMPTZ,
    _price DOUBLE PRECISION,  
    _volume DOUBLE PRECISION, 
    _side TEXT,
    _order_type TEXT,
    _symbol_id INTEGER 
)
LANGUAGE plpgsql    
AS $$
BEGIN
    INSERT INTO trade (time, price, volume, side, order_type, symbol_id) 
    VALUES (_time, _price, _volume, _side, _order_type, _symbol_id);
END;$$;

CREATE OR REPLACE FUNCTION get_trades (
    input_interval INTEGER, -- in minutes
    input_symbol TEXT -- all returns all symbols
)
RETURNS TABLE (
    time_ TIMESTAMPTZ,
    price DOUBLE PRECISION,
    volume DOUBLE PRECISION, 
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
            trade.time,
            trade.price,
            trade.volume,
            trade.side,
            trade.order_type, 
            _symbol.symbol AS symbol
        FROM trade 
        JOIN symbols AS _symbol ON trade.symbol_id = _symbol.id 
        AND trade.time >= NOW() - interval_duration
        ORDER BY trade.time; 
    ELSE 
        SELECT id INTO symbol_id_result FROM symbols WHERE symbols.symbol = input_symbol;
        
        SELECT
            trade.time,
            trade.price,
            trade.volume,
            trade.side,
            trade.order_type, 
            _symbol.symbol AS symbol
        FROM trade 
        JOIN symbols AS _symbol ON trade.symbol_id = _symbol.id 
        WHERE trade.symbol_id = symbol_id_result 
        AND trade.time >= NOW() - interval_duration
        ORDER BY trade.time; 
    END IF;
END;$$;

-- Example: SELECT * FROM get_trades (15, 'ETHUSD');

-- create ohlc (open, high, low, close) table for hour
CREATE MATERIALIZED VIEW ohlc_hour
WITH (timescaledb.continuous) AS
    SELECT
        time_bucket('1 hour', time) AS bucket,
        symbol_id,
        FIRST(price, time) AS open_price,
        MAX(price) AS high, 
        MIN(price) AS low,
        LAST(price, time) AS close_price,
        SUM(volume) AS volume,
        CAST(COUNT(*) AS INTEGER) AS count -- this could lead to an overflow if more than 2,147,483,647 trades occur in one hour highest value was 25k
    FROM trade
GROUP BY bucket, symbol_id; -- do i need the bucket here? TODO Test this? might improve performance

SELECT add_continuous_aggregate_policy('ohlc_hour',
    start_offset => INTERVAL '3 hour', 
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

CREATE OR REPLACE FUNCTION get_ohlc_hour (
    input_interval INTEGER, -- in days / all for all symbols
    input_symbol_id INTEGER 
)
RETURNS TABLE (
    bucket TIMESTAMPTZ,
    open_price DOUBLE PRECISION,
    high DOUBLE PRECISION,
    low DOUBLE PRECISION,
    close_price DOUBLE PRECISION,
    volume DOUBLE PRECISION,
    count INTEGER,
    symbol_id INTEGER
)
LANGUAGE plpgsql    
AS $$
DECLARE 
interval_duration INTERVAL; 
BEGIN
    interval_duration := input_interval * INTERVAL '1 hour';
    
    RETURN QUERY SELECT 
        ohlc_hour.bucket,
        ohlc_hour.open_price,
        ohlc_hour.high,
        ohlc_hour.low,
        ohlc_hour.close_price,
        ohlc_hour.volume,
        ohlc_hour.count,
        ohlc_hour.symbol_id
    FROM ohlc_hour
    WHERE ohlc_hour.symbol_id = input_symbol_id
    AND ohlc_hour.bucket >= NOW() - interval_duration
    ORDER BY ohlc_hour.bucket; 
END;$$;

-- using id instead of text for symbol
CREATE OR REPLACE FUNCTION get_ohlc_hour_start_date (
    input_interval INTEGER, -- in days / all for all symbols
    input_start_date TIMESTAMPTZ,
    input_symbol_id INTEGER 
)
RETURNS TABLE (
    bucket TIMESTAMPTZ,
    close_price DOUBLE PRECISION,
    volume DOUBLE PRECISION,
    count INTEGER, 
    symbol_id INTEGER
)
LANGUAGE plpgsql    
AS $$
DECLARE 
interval_duration INTERVAL; 
BEGIN
    interval_duration := input_interval * INTERVAL '1 hour';
    
    RETURN QUERY SELECT 
        ohlc_hour.bucket,
        ohlc_hour.close_price,
        ohlc_hour.volume,
        ohlc_hour.count,
        ohlc_hour.symbol_id
    FROM ohlc_hour
    WHERE ohlc_hour.symbol_id = input_symbol_id 
    AND ohlc_hour.bucket >= input_start_date - interval_duration 
    AND input_start_date >= ohlc_hour.bucket 
    ORDER BY ohlc_hour.bucket; 
END;$$;


-- create ohlc (open, high, low, close) table for day
CREATE MATERIALIZED VIEW ohlc_day 
WITH (timescaledb.continuous) AS
    SELECT
        time_bucket('1 day', time) AS bucket,
        symbol_id,
        FIRST(price, time) AS open_price,
        MAX(price) AS high, 
        MIN(price) AS low,
        LAST(price, time) AS close_price,
        SUM(volume) AS volume,
        CAST(COUNT(*) AS INTEGER) AS count
    FROM trade
GROUP BY bucket, symbol_id;

SELECT add_continuous_aggregate_policy('ohlc_day',
    start_offset => INTERVAL '3 day',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');

-- using symbol_id
CREATE OR REPLACE FUNCTION get_ohlc_day (
    input_interval INTEGER, -- in days / all for all symbols
    input_symbol_id INTEGER 
)
RETURNS TABLE (
    bucket TIMESTAMPTZ,
    open_price DOUBLE PRECISION,
    high DOUBLE PRECISION,
    low DOUBLE PRECISION,
    close_price DOUBLE PRECISION,
    volume DOUBLE PRECISION,
    count INTEGER,
    symbol_id INTEGER
)
LANGUAGE plpgsql    
AS $$
DECLARE 
interval_duration INTERVAL; 
BEGIN
    interval_duration := input_interval * INTERVAL '1 day';
    
    RETURN QUERY SELECT 
        ohlc_day.bucket,
        ohlc_day.open_price,
        ohlc_day.high,
        ohlc_day.low,
        ohlc_day.close_price,
        ohlc_day.volume,
        ohlc_day.count,
        ohlc_day.symbol_id
    FROM ohlc_day
    WHERE ohlc_day.symbol_id = input_symbol_id
    AND ohlc_day.bucket >= NOW() - interval_duration
    ORDER BY ohlc_day.bucket; 
END;$$;


-- get ohlc using start date and symbol_id
CREATE OR REPLACE FUNCTION get_ohlc_day_start_date (
    input_interval INTEGER, -- in days / all for all symbols
    input_start_date TIMESTAMPTZ,
    input_symbol_id INTEGER 
)
RETURNS TABLE (
    bucket TIMESTAMPTZ,
    close_price DOUBLE PRECISION,
    volume DOUBLE PRECISION,
    count INTEGER, 
    symbol_id INTEGER
)
LANGUAGE plpgsql    
AS $$
DECLARE 
interval_duration INTERVAL; 
BEGIN
    interval_duration := input_interval * INTERVAL '1 day';

    RETURN QUERY SELECT 
        ohlc_day.bucket,
        ohlc_day.close_price,
        ohlc_day.volume,
        ohlc_day.count,
        ohlc_day.symbol_id
    FROM ohlc_day
    WHERE ohlc_day.symbol_id = input_symbol_id
    AND ohlc_day.bucket >= input_start_date - interval_duration 
    AND input_start_date >= ohlc_day.bucket 
    ORDER BY ohlc_day.bucket; 
END;$$;

-- model config for easy swapping of models
CREATE TABLE model_configs (
    id SERIAL PRIMARY KEY,
    symbol_id SERIAL NOT NULL,
    modelname TEXT,
    data_type TEXT,
    lookback INTEGER,
    lags INTEGER[],
    window_sizes INTEGER[],
    span_sizes INTEGER[],
    active BOOLEAN,
    features TEXT[]
);

-- there should only be one activ model for each currency/pair
ALTER TABLE model_configs ADD CONSTRAINT model_configs_constraint
UNIQUE (symbol_id, active);


CREATE OR REPLACE FUNCTION get_model_config (
    input_symbol TEXT 
)
RETURNS TABLE (
    id INTEGER,
    symbol TEXT,
    modelname TEXT,
    data_type TEXT,
    lookback INTEGER,
    lags INTEGER[],
    window_sizes INTEGER[],
    span_sizes INTEGER[],
    active BOOLEAN,
    features TEXT[]
)
LANGUAGE plpgsql    
AS $$
DECLARE 
symbol_id_result INTEGER;
BEGIN
    SELECT id INTO symbol_id_result FROM symbols WHERE symbols.symbol = input_symbol;

    RETURN QUERY SELECT 
        model_configs.id,
        _symbol.symbol AS symbol,
        model_configs.modelname,
        model_configs.data_type,
        model_configs.lookback,
        model_configs.lags,
        model_configs.window_sizes,
        model_configs.span_sizes,
        model_configs.active,
        model_configs.features
    FROM model_configs
    JOIN symbols AS _symbol ON model_configs.symbol_id = _symbol.id 
    WHERE model_configs.symbol_id = symbol_id_result 
    AND model_configs.active IS TRUE;
END;$$;


CREATE OR REPLACE FUNCTION get_model_config (
    input_symbol_id INTEGER 
)
RETURNS TABLE (
    id INTEGER,
    symbol_id INTEGER,
    modelname TEXT,
    data_type TEXT,
    lookback INTEGER,
    lags INTEGER[],
    window_sizes INTEGER[],
    span_sizes INTEGER[],
    active BOOLEAN,
    features TEXT[]
)
LANGUAGE plpgsql    
AS $$
BEGIN
    RETURN QUERY SELECT 
        model_configs.id,
        model_configs.symbol_id,
        model_configs.modelname,
        model_configs.data_type,
        model_configs.lookback,
        model_configs.lags,
        model_configs.window_sizes,
        model_configs.span_sizes,
        model_configs.active,
        model_configs.features
    FROM model_configs
    WHERE model_configs.symbol_id = input_symbol_id 
    AND model_configs.active IS TRUE;
END;$$;



-- Prediction Data
CREATE TABLE IF NOT EXISTS kraken_prediction (
    time TIMESTAMPTZ NOT NULL,
    price DOUBLE PRECISION NOT NULL,  
    symbol_id SERIAL NOT NULL,
    model_config_id SERIAL NOT NULL
);

-- there should only be one entry for a model/timestamp
ALTER TABLE kraken_prediction ADD CONSTRAINT kraken_prediction_constraints
UNIQUE (time, model_id, symbol_id);

CREATE OR REPLACE PROCEDURE insert_prediction (
    _time TIMESTAMPTZ,
    _price DOUBLE PRECISION,
    _model_name TEXT,
    _symbol TEXT
)
LANGUAGE plpgsql    
AS $$
DECLARE 
symbol_result INTEGER;
model_result INTEGER;
BEGIN
    SELECT id INTO symbol_result FROM symbols WHERE symbol = _symbol;
    SELECT id INTO model_result FROM models WHERE model_name = _model_name;

    IF symbol_result IS NULL THEN
        INSERT INTO symbols (symbol) VALUES (_symbol) RETURNING id INTO symbol_result;
    END IF;

    IF model_result IS NULL THEN
        INSERT INTO models (model_name) VALUES (_model_name) RETURNING id INTO model_result;
    END IF;

    INSERT INTO kraken_prediction (time, price, symbol_id, model_id) 
    VALUES (_time, _price, symbol_result, model_result) ON CONFLICT (time, model_id, symbol_id) 
    DO UPDATE
    SET price = _price;
END;$$;

-- hourly prediction for now  (day?)
CREATE OR REPLACE FUNCTION get_predictions (
    input_interval INTEGER,
    input_start_date TIMESTAMPTZ,
    input_model TEXT,
    input_symbol TEXT
)
RETURNS TABLE (
    bucket TIMESTAMPTZ,
    price DOUBLE PRECISION
)
LANGUAGE plpgsql    
AS $$
DECLARE 
symbol_id_result INTEGER;
model_id_result INTEGER;
interval_duration INTERVAL; 
BEGIN
    interval_duration := input_interval * INTERVAL '1 hour';

    SELECT id INTO symbol_id_result FROM symbols WHERE symbols.symbol = input_symbol;
    SELECT id INTO model_id_result FROM models WHERE models.model_name = input_model;

    RETURN QUERY SELECT 
        kraken_prediction.time,
        kraken_prediction.price
    FROM kraken_prediction
    WHERE kraken_prediction.symbol_id = symbol_id_result 
    AND kraken_prediction.model_id = model_id_result 
    AND kraken_prediction.time >= input_start_date
    AND kraken_prediction.time < input_start_date + interval_duration  
    ORDER BY kraken_prediction.time; 
  
END;$$;