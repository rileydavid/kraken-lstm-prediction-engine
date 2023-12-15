import asyncio
import datetime
import time
import json 

import websockets

class WebSocketServer:

    def __init__(self, host, port, secret):
        self._secret = secret
        self._server = websockets.serve(self._start, host, port)
        self.x = 1 #'{"x": 0, "y": 0}' 
        self.y = 1
        self.data = b'{"x":"{0}", "y":{1}}' 

    def start(self):
        asyncio.get_event_loop().run_until_complete(self._server)
        asyncio.get_event_loop().run_forever()

    async def _start(self, websocket, path):
        print(f"Connected from path ={path}")
        while True:
            secret = await websocket.recv()
            if secret == self._secret:
                time.sleep(1)
                if self.x > 15:
                    self.x = 1
                    self.y = 1
                else: 
                    self.x = self.x + 1
                    self.y = self.y + 1 

                data = {"x": self.x, "y": self.y}

                #msg = f"Sending message {datetime.datetime.now()}"
                msg = json.dumps(data);
                print(msg)
                await websocket.send(msg)
                print(msg)


if __name__ == "__main__":
    server = WebSocketServer("192.168.1.43", 6969, "secret")
    print(server)
    server.start()