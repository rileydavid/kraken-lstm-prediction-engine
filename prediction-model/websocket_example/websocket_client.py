import asyncio
import websockets
import time
import json
from dateutil import parser
from sklearn.linear_model import LinearRegression

class WebSocketClient:
    def __init__(self, host, port, secret):
        self._secret = secret
        self._host = host
        self._port = port
        self.x = []
        self.y = []

    def start(self):
        asyncio.get_event_loop().\
        run_until_complete(self._start_receiving())

    async def _start_receiving(self):
        url = f'ws://{self._host}:{self._port}/ws'
        try:
            async with websockets.connect(url) as websocket:
                await websocket.send("XBTUSD")
                while True:
                    ##time.sleep(0.8)
                    response = await websocket.recv()
                    if "order_type" in response:
                        print(f'Received Message: {response}')
                        trade = json.loads(response); 
                        self.x.append([parser.isoparse(trade["time"]).timestamp() * 1_000])

                        self.y.append(trade["price"])

                        print(self.x)

                        model = LinearRegression()
                        model.fit(self.x, self.y)

                        print(self.x[0][0])
                        print("pred ", model.predict([[self.x[0][0] + 900000]]))



        except websockets.ConnectionClosed as ex:
            print(ex)
            raise

if __name__ == "__main__":
    client = WebSocketClient("127.0.0.1", 8000, "secret")
    client.start()
    print("returns? ")
    while True:
        time.sleep(5)
        print("data ", client._data)