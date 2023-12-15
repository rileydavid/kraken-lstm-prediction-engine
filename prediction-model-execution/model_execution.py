# Python Script for Model Execution
import os
from dotenv import load_dotenv
from data_preperation import * 
from data_service import * 
from model_service import *
from keras.models import load_model
import cloudpickle

print("Starting!")

load_dotenv()

print("dotenv")

print(os.getenv('S3_BUCKET'))
print(os.getenv('MODEL_NAME'))
print(os.getenv('AWS_ACCESS_KEY_ID'))
print(os.getenv('AWS_SECRET_ACCESS_KEY'))
print(os.getenv('S3_ENDPOINT_URL'))

model_service = ModelService(os.getenv('S3_BUCKET'),
                            os.getenv('MODEL_NAME'),
                            os.getenv('AWS_ACCESS_KEY_ID'),
                            os.getenv('AWS_SECRET_ACCESS_KEY'),
                            os.getenv('S3_ENDPOINT_URL'))

#model_service.fetch_model('model.pkl')

data_service = DataService(os.getenv('BACKEND_ENDPOINT'))

# todo use some kind of config for these values so they can easily be switched
lookback = 16 
max_close_lag = 168

df = data_service.get_ohlc_hour_start_date("ETHUSD", lookback+max_close_lag, "2023-09-01 00:00:00+00:00")

print(df.dtypes)
print(df.head())
df.reset_index(inplace=True)
print(df.columns)

print(len(df))

data_preperation_service = DataPreparation(df)

# flaw in this is missing data points --> for example there is no bucket for the exact start date!
# this can be detected 
data_preperation_service.data_prep()

features = ['is_holiday', 'close_price', 'volume', 'is_weekend', 'count', 'hour_cos', 'hour_sin', 'day_of_week_cos', 'day_of_week_sin', 'month_cos', 'month_sin', 'day_of_month_sin', 'day_of_month_cos', 'year_normalized', 'close_lag12', 'close_lag168', 'ema_6_close_price', 'ema_12_close_price']

# X hold the sequence that is feed to the prediction model 
X = data_preperation_service.create_prediction_sequence_np(features=features, lookback=lookback)

print(X)
print(X.shape)

X_reshaped = X.reshape(1, 16, 18)

model_service.fetch_model("model.pkl")

with open('model.pkl', 'rb') as file:
    model = cloudpickle.load(file)

predicted_price_log = model.predict(X_reshaped)

# this seems to be correct
print(predicted_price_log)
print(np.expm1(predicted_price_log)[0, 0]) 

# where should those predicted values be saved? 
# kraken_prediction? 