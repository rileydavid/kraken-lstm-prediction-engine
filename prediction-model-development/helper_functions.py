import pandas as pd
import numpy as np
import holidays
from calendar import monthrange
from sklearn.preprocessing import StandardScaler

# Data Preperation as input for model training/prediction 
def create_sequences_np(data, features, lookback, datatype=np.float16):
    data_array = data[features].to_numpy()
    X, y = [], []
    for i in range(lookback, len(data_array)):
        X.append(data_array[i-lookback:i])
        y.append(data_array[i, data.columns.get_loc('close_price')])
    return np.array(X).astype(datatype), np.array(y).astype(datatype)

# Feature Engineering
def convert_timestamp(data): 
    data['bucket'] = pd.to_datetime(data['bucket'])
    data.sort_values('bucket', inplace=True)
    return data

def normalization_standard_scaler(data):
    scaler = StandardScaler()
    data[['open_price', 'high', 'low', 'close_price', 'volume']] = scaler.fit_transform(data[['open_price', 'high', 'low', 'close_price', 'volume']])
    return data

def add_feature_date(data):
    # time-based features
    data['hour'] = data['bucket'].dt.hour
    data['day_of_week'] = data['bucket'].dt.dayofweek  # Monday=0, Sunday=6
    data['day_of_month'] = data['bucket'].dt.day
    data['month'] = data['bucket'].dt.month
    data['year'] = data['bucket'].dt.year

    # binary feature for weekend
    data['is_weekend'] = data['day_of_week'].apply(lambda x: 1 if x > 4 else 0)
    return data

def add_holiday_feature(data):
    us_holidays = holidays.UnitedStates()
    data['is_holiday'] = data['bucket'].apply(lambda x: x in us_holidays).astype(int)
    return data

def add_feature_high_low_range(data):
    data['hl_range'] = data['high'] - data['low']
    return data

def add_feature_lag(data, lags=[1]): 
    for lag in lags: 
        data['close_lag'+str(lag)] = data['close_price'].shift(lag)
    #data = data.dropna() ## drop first few rows that are now NaN
    return data

## cyclic encoding for time based features
def encode_cyclic_feature(value, cycle_length, type):
    value_rad = (value / cycle_length) * 2 * np.pi
    if type == 'sin':
        return np.sin(value_rad) 
    else: 
        return np.cos(value_rad)

def normalize_feature(value, min_value, max_value):
    return (value - min_value) / (max_value - min_value)

def get_distinct_count(data, column): 
    temp = data.copy()
    return len(pd.unique(temp[column]))

# returns the min and max value of the range
def get_min_max_value(data, column): 
    return data[column].min(), data[column].max()

def apply_cyclic_encoding(data, columms=['hour', 'day_of_week', 'month']): 
    for column in columms: 
        cycle_length = get_distinct_count(data, column)
        data[column + "_cos"] = data[column].apply(lambda x: encode_cyclic_feature(x, cycle_length, type='cos')).astype(float)
        data[column + "_sin"] = data[column].apply(lambda x: encode_cyclic_feature(x, cycle_length, type='sin')).astype(float)
    return data

def normalize_year(data, columns=['year']): 
    for column in columns: 
        min, max = get_min_max_value(data, column)
        data[column+'_normalized'] = data[column].apply(lambda x: normalize_feature(x, min, max+1)) # +1 to account for 2024
    return data

def cyclic_encoding_year(data):
    day = 24*60*60
    year = (365.2425)*day
    data['year_sin'] = data['year'].apply(lambda x: np.sin(2 * np.pi / year))
    data['year_cos'] = data['year'].apply(lambda x: np.cos(2 * np.pi / year))
    return data

def get_month_length(month, year):
    return monthrange(year, month)[1]

# inspired by https://stats.stackexchange.com/questions/126230/optimal-construction-of-day-feature-in-neural-networks
def encode_day_of_month(day_of_month, month, year, type):
    normalized_day = (day_of_month - 1) / get_month_length(month, year)  # Subtract 1 to start from 0
    if type == 'sin': 
        return np.sin(2 * np.pi * normalized_day) 
    else: 
        return np.cos(2 * np.pi * normalized_day) 

def apply_day_of_month_encoding(data): 
    data["day_of_month_sin"] = data.apply(lambda row: encode_day_of_month(row['day_of_month'], row['month'], row['year'], 'sin'), axis=1)
    data["day_of_month_cos"] = data.apply(lambda row: encode_day_of_month(row['day_of_month'], row['month'], row['year'], 'cos'), axis=1)
    return data

def process_timestamp(data, fill=False): 
    # fills in gaps in the dataset and creates 1 hour buckets
    data['bucket'] = pd.to_datetime(data['bucket'])
    data.set_index('bucket', inplace=True)
    if fill: 
        data = data.resample('H').asfreq()
        data['close_price'] = data['close_price'].fillna(method='ffill') # carry over values from last hour 
        data['volume'] = data['volume'].fillna(0) # fill missing values with 0 because there have been no trades 
        data['count'] = data['count'].fillna(0)
    data.sort_values('bucket', inplace=True)
    data.reset_index(inplace=True)
    return data

def apply_log_scaler(data, columns=['close_price', 'volume', 'close_lag12', 'close_lag168']): 
    for column in columns:
        data[column] = np.log1p(data[column])
    return data

def simple_moving_average(data, window_sizes, columns=['close_price']): 
    for column in columns: 
        for window_size in window_sizes: 
            data['sma_'+str(window_size)+"_"+column] = data[column].rolling(window=window_size).mean()
    return data

def exponential_moving_average(data, span_sizes, columns=['close_price']):
    for column in columns: 
        for span in span_sizes: 
            data['ema_'+str(span)+"_"+column] = data[column].ewm(span=span).mean()
    return data