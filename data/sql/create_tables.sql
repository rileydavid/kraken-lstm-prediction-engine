-- create database for mlflow parameter tracking 
CREATE DATABASE mlflow;

-- https://www.timescale.com/blog/best-practices-for-picking-postgresql-data-types/
-- https://stackoverflow.com/questions/20884405/is-there-a-performance-hit-using-decimal-data-types-mysql-postgres
-- change from text to varchar(1)
CREATE TABLE IF NOT EXISTS trade (
    time TIMESTAMPTZ NOT NULL,
    price DOUBLE PRECISION NOT NULL,  
    volume DOUBLE PRECISION NOT NULL, 
    side VARCHAR(1),
    order_type VARCHAR(1),
    symbol_id SERIAL NOT NULL
);

-- index for better query performance (get_trades_range)
CREATE INDEX idx_trade_symbol_id ON trade (symbol_id, time DESC);

-- hyper table
SELECT create_hypertable('trade', 'time', migrate_data => true);

CREATE TABLE IF NOT EXISTS symbol (
    id SERIAL PRIMARY KEY,
    symbol TEXT NOT NULL UNIQUE
);

-- index for symbols
CREATE INDEX idx_symbol_symbol ON symbol (symbol);

-- for inserting new symbols --> collector get's the id back to cache locally
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
    INSERT INTO symbol (symbol)
    VALUES (input_symbol)
    ON CONFLICT (symbol) DO NOTHING
    RETURNING id INTO out_id;

    -- If the insert did not happen, select the existing row
    IF NOT FOUND THEN
        RETURN QUERY SELECT id, symbol FROM symbol WHERE symbol = input_symbol;
    ELSE
        out_symbol := input_symbol;
        RETURN NEXT;
    END IF;
END;
$$;

-- insert trade --> used by the collector
CREATE OR REPLACE PROCEDURE insert_trade (
    _time TIMESTAMPTZ,
    _price DOUBLE PRECISION,  
    _volume DOUBLE PRECISION, 
    _side VARCHAR(1),
    _order_type VARCHAR(1),
    _symbol_id INTEGER 
)
LANGUAGE plpgsql    
AS $$
BEGIN
    INSERT INTO trade (time, price, volume, side, order_type, symbol_id) 
    VALUES (_time, _price, _volume, _side, _order_type, _symbol_id);
END;$$;

-- Example: SELECT * FROM get_trades_range('2023-01-01 00:00:00+00', '2023-12-31 23:59:59+00', 1);
CREATE OR REPLACE FUNCTION get_trades_range(
    input_from_date TIMESTAMPTZ,
    input_to_date TIMESTAMPTZ,
    input_symbol_id INTEGER
)
RETURNS TABLE (
    time_ TIMESTAMPTZ,
    price DOUBLE PRECISION,
    volume DOUBLE PRECISION,
    side VARCHAR(1),
    order_type VARCHAR(1),
    symbol_id INTEGER
)
LANGUAGE sql
STABLE
AS $$
SELECT
    time,
    price,
    volume,
    side,
    order_type,
    symbol_id
FROM trade
WHERE symbol_id = $3
  AND time BETWEEN $1 AND $2
ORDER BY time;
$$;

-- create ohlc (open, high, low, close) table for hour
-- finalized=TRUE --> only data that is fully aggregated is returned (full hours only)
CREATE MATERIALIZED VIEW ohlc_hour
WITH (timescaledb.continuous, timescaledb.create_group_indexes=TRUE, timescaledb.finalized=TRUE) AS
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

-- index for better query performance (get_ohlc_hour_range)
CREATE INDEX ohlc_hour_idx ON ohlc_hour (bucket, symbol_id);

-- continuous aggregate policy to aggregate data from trades as time passes
SELECT add_continuous_aggregate_policy('ohlc_hour',
    start_offset => INTERVAL '3 hour', 
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

CREATE OR REPLACE FUNCTION get_ohlc_hour_range (
    input_from_date TIMESTAMPTZ,
    input_to_date TIMESTAMPTZ,
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
BEGIN
    RETURN QUERY SELECT 
        ohlc_hour.bucket,
        ohlc_hour.close_price,
        ohlc_hour.volume,
        ohlc_hour.count,
        ohlc_hour.symbol_id
    FROM ohlc_hour
    WHERE ohlc_hour.symbol_id = input_symbol_id
    AND ohlc_hour.bucket >= input_from_date  
    AND input_to_date >= ohlc_hour.bucket 
    ORDER BY ohlc_hour.bucket; 
END;$$;

-- create ohlc (open, high, low, close) table for day
-- finalized=TRUE --> only data that is fully aggregated is returned (full days only)
CREATE MATERIALIZED VIEW ohlc_day 
WITH (timescaledb.continuous, timescaledb.create_group_indexes=TRUE, timescaledb.finalized=TRUE) AS
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

-- Not needed because the option timescaledb.create_group_indexes=TRUE is set
-- CREATE INDEX ohlc_day_idx ON ohlc_day (bucket, symbol_id);

-- continuous aggregate policy to aggregate data from trades as time passes
SELECT add_continuous_aggregate_policy('ohlc_day',
    start_offset => INTERVAL '3 day',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');

CREATE OR REPLACE FUNCTION get_ohlc_day_range (
    input_from_date TIMESTAMPTZ,
    input_to_date TIMESTAMPTZ,
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
BEGIN
    RETURN QUERY SELECT 
        ohlc_day.bucket,
        ohlc_day.close_price,
        ohlc_day.volume,
        ohlc_day.count,
        ohlc_day.symbol_id
    FROM ohlc_day
    WHERE ohlc_day.symbol_id = input_symbol_id
    AND ohlc_day.bucket >= input_from_date  
    AND input_to_date >= ohlc_day.bucket 
    ORDER BY ohlc_day.bucket; 
END;$$;

-- model config for easy swapping of models
CREATE TABLE model_config (
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
ALTER TABLE model_config ADD CONSTRAINT model_config_constraint
UNIQUE (symbol_id, active);

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
        model_config.id,
        model_config.symbol_id,
        model_config.modelname,
        model_config.data_type,
        model_config.lookback,
        model_config.lags,
        model_config.window_sizes,
        model_config.span_sizes,
        model_config.active,
        model_config.features
    FROM model_config
    WHERE model_config.symbol_id = input_symbol_id 
    AND model_config.active IS TRUE;
END;$$;

-- Inserting ETH/USD to make sure ETH/USD has id 1
SELECT * FROM insert_symbol('ETH/USD');

-- INSERT INTO model_config 
-- (symbol_id, modelname, data_type, lookback, lags, window_sizes, span_sizes, active, features) 
-- VALUES 
-- (1, 'model.pkl', 'np.float32', 16, ARRAY[12, 168], ARRAY[]::INTEGER[], ARRAY[6, 12], true, 
-- ARRAY['is_holiday', 'close_price', 'volume', 'is_weekend', 'count', 'hour_cos', 'hour_sin', 
-- 'day_of_week_cos', 'day_of_week_sin', 'month_cos', 'month_sin', 'day_of_month_sin', 
-- 'day_of_month_cos', 'year_normalized', 'close_lag12', 'close_lag168', 'ema_6_close_price', 
-- 'ema_12_close_price']::TEXT[]);

-- INSERT INTO model_config 
-- (symbol_id, modelname, data_type, lookback, lags, window_sizes, span_sizes, active, features) 
-- VALUES 
-- (1, 'awesome-bird-301.pkl', 'np.float32', 18, ARRAY[12, 96], ARRAY[]::INTEGER[], ARRAY[12,48,96,168], true, 
-- ARRAY['is_holiday', 'close_price', 'volume', 'is_weekend', 'count', 'hour_cos', 'hour_sin', 
-- 'day_of_week_cos', 'day_of_week_sin', 'month_cos', 'month_sin', 'day_of_month_sin', 
-- 'day_of_month_cos', 'year_normalized', 'close_lag12', 'close_lag96', 'ema_12_close_price',
--  'ema_48_close_price', 'ema_96_close_price', 'ema_168_close_price']::TEXT[]);

INSERT INTO model_config 
(symbol_id, modelname, data_type, lookback, lags, window_sizes, span_sizes, active, features) 
VALUES 
(1, 'clean-mare-368.pkl', 'np.float32', 12, ARRAY[12, 96], ARRAY[]::INTEGER[], ARRAY[12,48,96,168], true, 
ARRAY['is_holiday', 'close_price', 'volume', 'is_weekend', 'count', 'hour_cos', 'hour_sin',
'day_of_week_cos', 'day_of_week_sin', 'month_cos', 'month_sin', 'day_of_month_sin',
'day_of_month_cos', 'year_normalized', 'close_lag12', 'close_lag96', 'ema_12_close_price',
'ema_48_close_price', 'ema_96_close_price', 'ema_168_close_price']::TEXT[]);



-- currently not in use
-- Prediction Data
CREATE TABLE IF NOT EXISTS kraken_prediction (
    time TIMESTAMPTZ NOT NULL,
    price DOUBLE PRECISION NOT NULL,  
    symbol_id SERIAL NOT NULL,
    model_id SERIAL NOT NULL
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
    SELECT id INTO symbol_result FROM symbol WHERE symbol = _symbol;
    SELECT id INTO model_result FROM models WHERE model_name = _model_name;

    IF symbol_result IS NULL THEN
        INSERT INTO symbol (symbol) VALUES (_symbol) RETURNING id INTO symbol_result;
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

    SELECT id INTO symbol_id_result FROM symbol WHERE symbol.symbol = input_symbol;
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
