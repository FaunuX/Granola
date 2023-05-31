from granola import serve 
import asyncio

class Main:
    def main(self):
        return {"source": "main"}

class App:
    def __granola__(self):
        return "<h1> Hello WOrld </h1>"

    def api(self):
        return Main()

async def main():
    await serve(8685, App())

asyncio.run(main())
