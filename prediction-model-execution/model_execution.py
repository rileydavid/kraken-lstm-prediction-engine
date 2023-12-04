# Python Script for Model Execution
import os
from dotenv import load_dotenv
from data_preperation import * 
from data_service import * 
from model_service import *

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

df = data_service.get_ohlc_hour("ETHUSD", 100)

print(df.dtypes)
print(df.head())
df.reset_index(inplace=True)
print(df.columns)

data_preperation_service = DataPreparation(df)

data_preperation_service.data_prep()

print(data_preperation_service.data.head())


#print("after fetch ")
#
## maybe create a config which is stored in a database to enable dynamic switching of models
#lookback = 16
#
#features = ['is_holiday', 'close_price', 'volume', 'is_weekend', 'count', 'hour_cos', 
#            'hour_sin', 'day_of_week_cos', 'day_of_week_sin', 'month_cos', 'month_sin',
#            'day_of_month_sin', 'day_of_month_cos', 'year_normalized', 'close_lag12',
#            'close_lag168', 'ema_6_close_price', 'ema_12_close_price']
#
#hours_back = 168 + lookback








