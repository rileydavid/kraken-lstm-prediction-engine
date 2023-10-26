
CREATE INDEX idx_kraken_symbol_time ON kraken_trade (symbol, time DESC);

ALTER TABLE kraken_trade  SET LOGGED;

SELECT create_hypertable('kraken_trade', 'time', migrate_data => true);


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

--SELECT * FROM kraken_candlestick_day WHERE symbol = 'ETHUSD' AND bucket >= NOW() - INTERVAL '2 years' ORDER BY bucket;
SELECT * FROM one_min_candle WHERE symbol = 'XRPUSD' ORDER BY bucket LIMIT 1000;


SELECT * FROM kraken_trade ORDER BY time DESC LIMIT 1000;


CREATE MATERIALIZED VIEW ten_min_average_price
WITH (timescaledb.continuous) AS
SELECT time_bucket('10 minutes', time) AS ten_min, avg(price)
FROM kraken_trade
GROUP BY ten_min
ORDER BY ten_min DESC;


SELECT add_continuous_aggregate_policy('ten_min_average_price',
    start_offset => INTERVAL '20 min',
    end_offset => INTERVAL '10 min',
    schedule_interval => INTERVAL '10 min');


SELECT
  time_bucket('1 mins', "time") AS bucket,
  average(price) AS price,
  symbol
FROM kraken_trade
GROUP BY bucket, symbol;

