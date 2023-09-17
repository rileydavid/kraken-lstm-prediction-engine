DEV TODOS: 

Good: sqlx migrate run --database-url postgres://admin:password@localhost:5432/db

Update the converter --> to set up one_min_candle etc

Update Database Create statements to reflect changes made in the Collector
    - Create materialized views
    - Auto aggregates for 
    - Look into Windows  
Create a persitent good grafana dashboard --> save the config somehow
Done - make sure git only records things that really matter (update git ignore)

Implement linear regression
Check if converter converts the timestamp correctly


Backend and simple charts javascript dashboard? 
Grafana not performing well in my opinion 


SELECT * FROM kraken_candlestick_day
WHERE symbol = 'ETHUSD' AND bucket >= NOW() - INTERVAL '2 years'
ORDER BY bucket;


-- other important notes

SELECT * FROM one_min_candle WHERE symbol = 'ETHUSD' AND bucket >= NOW() - INTERVAL '15 mins' ORDER BY bucket;

## TimescaleDB connecting using the terminal 

#### Startup command 
docker run -d --name timescaledb -p 127.0.0.1:5432:5432 \2 \
-e POSTGRES_PASSWORD=password timescale/timescaledb-ha:pg15-latest

#### Connecting 
psql -p 5432 -h localhost -U postgres
Password for user postgres: password

#### APLHA VANTAGE API KEY LINUX ENV Variable 
export ALPHAVANTAGE_API_KEY=17AUX98OD3PA8RXB

API-Key 17AUX98OD3PA8RXB


### Create Table
CREATE TABLE ethusd (time TIMESTAMP NOT NULL, price DOUBLE PRECISION, amount DOUBLE PRECISION);

## UNLOGGED for data import 
ALTER TABLE ethusd SET LOGGED;

## Create Hypertable 
SELECT create_hypertable('ethusd', 'time', migrate_data => true);

### copy data file to pgadmin docker 
docker cp ./converted_ethusd.csv e9647d84f354:/var/lib/pgadmin/storage/rileydavid_live.at/

### fix permission issue for pgadmin docker 
sudo chown -R 5050:5050 ./pgadmin

### fix permission issue grafana
sudo chown -R 472:472 ./grafana/

### creating a continous view for one day candles 
CREATE MATERIALIZED VIEW one_min_candle
WITH (timescaledb.continuous) AS
    SELECT
        time_bucket('1 min', time) AS bucket,
        FIRST(price, time) AS "open",
        MAX(price) AS high,
        MIN(price) AS low,
        LAST(price, time) AS "close",
	FROM ethusd
    GROUP BY bucket;

CREATE MATERIALIZED VIEW one_min_candle
WITH (timescaledb.continuous) AS
    SELECT
        time_bucket('1 min', time) AS bucket,
        FIRST(price, time) AS "open",
        MAX(price) AS high,
        MIN(price) AS low,
        LAST(price, time) AS "close",
	FROM ethusd_ohlcvt_1
    GROUP BY bucket;


## Working one_min_candle
CREATE MATERIALIZED VIEW one_min_candle
WITH (timescaledb.continuous) AS
    SELECT
        time_bucket('1 min', time) AS bucket,
        FIRST(price, time) AS "open",
        MAX(price) AS high,
        MIN(price) AS low,
        LAST(price, time) AS "close",
        LAST(volume, time) AS min_volume
    FROM ethusd_trade
    GROUP BY bucket

### refresh the view above 
SELECT add_continuous_aggregate_policy('one_day_candle',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');

### Grafana Candle Sticks
SELECT * FROM one_day_candle
ORDER BY bucket;

SELECT * FROM one_min_candle
ORDER BY bucket;


SELECT * FROM ethusd_ohlcvt_1 ORDER BY time DESC LIMIT 100;


### Rebuild Docker Rust 
docker build --no-cache -t collector-trade . 
