from granola import serve

class App:
    def __granola__(self):
        return "hello world"

serve(5000, App())
