from granola import serve 

class Main:
    def main(self):
        return "data"

class App:
    def __granola__(self):
        return "WELCOME TO SERVER"

    def api(self):
        return Main()

serve(8685, App())
