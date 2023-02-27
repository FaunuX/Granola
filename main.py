from granola import serve 

class App:
    def __str__(self):
        return "WELCOME TO SERVER"

    def api(self):
        target = 0
        for i in range(10):
            target += i
        return target

serve(8685, App())
# from granola import serve

# class App:
#     def __init__(self, props):
#         self.data = props

#     def api(self):
#         return self.data

# serve(8685, App("Hello World"))
 
