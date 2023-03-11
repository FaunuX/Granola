from granola import serve 

class App:
    def __granola__(self):
        return "WELCOME TO SERVER"

    def api(self, request):
        target = 0
        for i in range(10):
            target += i
        return request

serve(8685, App())
