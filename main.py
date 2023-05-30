from granola import serve 

class Main:
    def main(self):
        return {"source": "main"}

class App:
    def __granola__(self):
        return "<h1> Hello WOrld </h1>"

    def api(self):
        return Main()

serve(8685, App())
