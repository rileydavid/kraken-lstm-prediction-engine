import os
import numpy as np
import cloudpickle
from dateutil import parser
from datetime import timedelta, datetime
from dotenv import load_dotenv
# Make sure to import your custom modules
from data_preperation import * 
from data_service import * 
from model_provider import *

class ModelExecution:
    def __init__(self, config):
        print("Initializing Model Execution")
        load_dotenv()
        self.config = config

        self.model_provider = ModelProvider(os.getenv('S3_BUCKET'),
                                           os.getenv('MODEL_NAME'),
                                           os.getenv('AWS_ACCESS_KEY_ID'),
                                           os.getenv('AWS_SECRET_ACCESS_KEY'),
                                           os.getenv('S3_ENDPOINT_URL'))

        self.data_service = DataService(os.getenv('BACKEND_ENDPOINT'))

    def execute(self):
        print("Starting execution")
        
        df = pd.DataFrame(self.config['data'])
        print(df.head())

        df.reset_index(inplace=True)

        data_preperation_service = DataPreparation(df, self.config)

        X = data_preperation_service.prediction_sequence
        X_reshaped = X.reshape(1, int(self.config['lookback']), len(self.config['features']))

        self.model_provider.fetch_model(self.config['modelname'])
        with open(self.config['modelname'], 'rb') as file:
            model = cloudpickle.load(file)

        predicted_price_log = model.predict(X_reshaped, verbose=0)
        predicted_price = np.expm1(predicted_price_log)[0, 0]
        
        parsed_date = datetime.strptime(self.config['from_date'], '%Y-%m-%dT%H:%M:%S%z')
        new_date = parsed_date + timedelta(hours=1)
        output_date = new_date.strftime('%Y-%m-%dT%H:%M:%S') + 'Z'
        print(output_date)
        
        return output_date, predicted_price
