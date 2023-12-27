import requests
import pandas as pd

class DataService:
    def __init__(self, base_url):
        self.base_url_ohlc_hour = f"{base_url}{'ohlc'}/{'hour'}"
        self.base_url_ohlc_day = f"{base_url}{'ohlc'}/{'day'}"

    def get_ohlc_hour(self, symbol, interval):
        url = f"{self.base_url_ohlc_hour}/{symbol}/{interval}"
        return self.fetch_data_get(url)

    def get_ohlc_hour_start_date(self, symbol, interval, from_date):
        url = f"{self.base_url_ohlc_hour}/start"

        data = {
            "symbol_id": symbol,
            "interval": interval.item(),
            "from_date": from_date
        }

        print(data)
        return self.fetch_data_post(url, data)
    
    def get_ohlc_day_start_date(self, symbol, interval, from_date):
        url = f"{self.base_url_ohlc_day}/start"
        data = {
            "symbol_id": symbol,
            "interval": interval.item(),
            "from_date": from_date
        }
        return self.fetch_data_post(url, data)

    def get_ohlc_day(self, symbol, interval):
        url = f"{self.base_url_ohlc_day}/{symbol}/{interval}"
        return self.fetch_data_get(url)

    def fetch_data_post(self, url, data):
        response = requests.post(url, json=data)

        if response.status_code == 200:
            print("Status code")
            data = response.json()
            df = pd.DataFrame(data)
            return df
        else:
            print("Failed to fetch data: Status code", response.status_code)

    def fetch_data_get(self, url):
        response = requests.get(url)

        if response.status_code == 200:
            print("Status code")
            data = response.json()
            df = pd.DataFrame(data)
            return df
        else:
            print("Failed to fetch data: Status code", response.status_code)
