import pandas as pd
import numpy as np
import holidays
from calendar import monthrange
from sklearn.preprocessing import StandardScaler

class DataPreparation:
    def __init__(self, data):
        self.data = data

    # Data Preperation as input for model training/prediction 
    def create_sequences_np(self, features, lookback, datatype=np.float16):
        data_array = self.data[features].to_numpy()
        X, y = [], []
        for i in range(lookback, len(data_array)):
            X.append(data_array[i-lookback:i])
            y.append(data_array[i, self.data.columns.get_loc('close_price')])
        return np.array(X).astype(datatype), np.array(y).astype(datatype)

    # Feature Engineering
    def convert_timestamp(self): 
        self.data['bucket'] = pd.to_datetime(self.data['bucket'])
        self.data.sort_values('bucket', inplace=True)

    def normalization_standard_scaler(self):
        scaler = StandardScaler()
        self.data[['open_price', 'high', 'low', 'close_price', 'volume']] = scaler.fit_transform(self.data[['open_price', 'high', 'low', 'close_price', 'volume']])

    def add_feature_date(self):
        # time-based features
        self.data['hour'] = self.data['bucket'].dt.hour
        self.data['day_of_week'] = self.data['bucket'].dt.dayofweek  # Monday=0, Sunday=6
        self.data['day_of_month'] = self.data['bucket'].dt.day
        self.data['month'] = self.data['bucket'].dt.month
        self.data['year'] = self.data['bucket'].dt.year
        # binary feature for weekend
        self.data['is_weekend'] = self.data['day_of_week'].apply(lambda x: 1 if x > 4 else 0)

    def add_holiday_feature(self):
        us_holidays = holidays.UnitedStates()
        self.data['is_holiday'] = self.data['bucket'].apply(lambda x: x in us_holidays).astype(int)

    def add_feature_high_low_range(self):
        self.data['hl_range'] = self.data['high'] - self.data['low']

    def add_feature_lag(self, lags=[1]): 
        for lag in lags: 
            self.data['close_lag'+str(lag)] = self.data['close_price'].shift(lag)
        #data = data.dropna() ## drop first few rows that are now NaN

    def normalize_feature(value, min_value, max_value):
        return (value - min_value) / (max_value - min_value)

    def get_distinct_count(self, column): 
        return len(pd.unique(self.data[column]))

    # returns the min and max value of the range
    def get_min_max_value(self, column): 
        return self.data[column].min(), self.data[column].max()

    # cyclic encoding for time based features
    def encode_cyclic_feature(value, cycle_length, encoding_type):
        value_rad = (value / cycle_length) * 2 * np.pi
        if encoding_type == 'sin':
            return np.sin(value_rad) 
        else: 
            return np.cos(value_rad)

    def apply_cyclic_encoding(self, columms=['hour', 'day_of_week', 'month']): 
        for column in columms: 
            cycle_length = self.get_distinct_count(column) 
            print(cycle_length)
            self.data[column + "_cos"] = self.data[column].apply(lambda x: DataPreparation.encode_cyclic_feature(x, cycle_length, encoding_type='cos')).astype(float)
            self.data[column + "_sin"] = self.data[column].apply(lambda x: DataPreparation.encode_cyclic_feature(x, cycle_length, encoding_type='sin')).astype(float)

    def normalize_year(self, columns=['year']): 
        for column in columns: 
            min, max = self.get_min_max_value(column)
            self.data[column+'_normalized'] = self.data[column].apply(lambda x: DataPreparation.normalize_feature(x, min, max+1)) # +1 to account for 2024
        
    def get_month_length(month, year):
        return monthrange(year, month)[1]

    # inspired by https://stats.stackexchange.com/questions/126230/optimal-construction-of-day-feature-in-neural-networks
    def encode_day_of_month(day_of_month, month, year, type):
        normalized_day = (day_of_month - 1) / DataPreparation.get_month_length(month, year)  # Subtract 1 to start from 0
        if type == 'sin': 
            return np.sin(2 * np.pi * normalized_day) 
        else: 
            return np.cos(2 * np.pi * normalized_day) 

    def apply_day_of_month_encoding(self): 
        self.data["day_of_month_sin"] = self.data.apply(lambda row: DataPreparation.encode_day_of_month(row['day_of_month'], row['month'], row['year'], 'sin'), axis=1)
        self.data["day_of_month_cos"] = self.data.apply(lambda row: DataPreparation.encode_day_of_month(row['day_of_month'], row['month'], row['year'], 'cos'), axis=1)

    def process_timestamp(self, fill=False): 
        # fills in gaps in the dataset and creates 1 hour buckets
        self.data['bucket'] = pd.to_datetime(self.data['bucket'])
        self.data.set_index('bucket', inplace=True)
        if fill: 
            self.data = self.data.resample('H').asfreq()
            self.data['close_price'] = self.data['close_price'].fillna(method='ffill') # carry over values from last hour 
            self.data['volume'] = self.data['volume'].fillna(0) # fill missing values with 0 because there have been no trades 
            self.data['count'] = self.data['count'].fillna(0)
        self.data.sort_values('bucket', inplace=True)
        self.data.reset_index(inplace=True)
        
    def apply_log_scaler(self, columns=['close_price', 'volume', 'close_lag12', 'close_lag168']): 
        for column in columns:

            self.data[column] = np.log1p(self.data[column])

    def simple_moving_average(self, window_sizes, columns=['close_price']): 
        for column in columns: 
            for window_size in window_sizes: 
                self.data['sma_'+str(window_size)+"_"+column] = self.data[column].rolling(window=window_size).mean()

    def exponential_moving_average(self, span_sizes, columns=['close_price']):
        for column in columns: 
            for span in span_sizes: 
                self.data['ema_'+str(span)+"_"+column] = self.data[column].ewm(span=span).mean()

    def convert_to_numeric(self, columns=["close_price, count, volume"]): 
        for column in columns: 
            self.data[column] = pd.to_numeric(self.data[column])

    def data_prep(self):
        # data prep
        self.data.drop(columns=['open_price', 'high', 'low', 'symbol'], axis=1)
        self.process_timestamp(fill=True)
        self.convert_to_numeric()
        self.add_feature_date()
        self.add_holiday_feature()
        self.apply_cyclic_encoding(columms=['hour', 'day_of_week', 'month'])
        self.apply_day_of_month_encoding()
        self.normalize_year() # has to be updated yearly (because of the min-max scaler - max+1 is currently set = 2024)
        self.apply_log_scaler(columns=['close_price', 'volume', 'count'])
        self.add_feature_lag(lags=[12,168])
        #data = simple_moving_average(data, window_sizes=[6,12,168], columns=['close_price']) # window sizes are in hours
        self.exponential_moving_average(span_sizes=[6,12], columns=['close_price']) # span sizes are in hours