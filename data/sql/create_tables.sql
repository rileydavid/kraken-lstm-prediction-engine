
CREATE TABLE IF NOT EXISTS kraken_trade (
    time TIMESTAMP NOT NULL,
    price NUMERIC NOT NULL,  
    volume NUMERIC NOT NULL, 
    side VARCHAR(1),
    order_type VARCHAR(1),
    symbol VARCHAR(14) NOT NULL -- One more just in case
);

SELECT create_hypertable('kraken_trade', 'time', migrate_data => true);

CREATE INDEX kraken_symbol_time ON kraken_trade (symbol, time DESC);

-- one minute candles 
CREATE MATERIALIZED VIEW one_min_candle
WITH (timescaledb.continuous) AS
SELECT
  time_bucket('1 min', "time") AS bucket,
  max(price) AS high,
  first(price, time) AS open,
  last(price, time) AS close,
  min(price) AS low,
  symbol
FROM kraken_trade
GROUP BY bucket, symbol;

SELECT add_continuous_aggregate_policy('one_min_candle',
    start_offset => INTERVAL '3 min',
    end_offset => INTERVAL '1 min',
    schedule_interval => INTERVAL '1 min');

-- one day candles
CREATE MATERIALIZED VIEW one_day_candle
WITH (timescaledb.continuous) AS
SELECT
  time_bucket('1 day', "time") AS bucket,
  max(price) AS high,
  first(price, time) AS open,
  last(price, time) AS close,
  min(price) AS low,
  symbol
FROM kraken_trade
GROUP BY bucket, symbol;

SELECT add_continuous_aggregate_policy('one_day_candle',
    start_offset => INTERVAL '3 day', -- might not be enough if data is importet --> find way to manually refresh
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');

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
