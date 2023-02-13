from granola import serve

class App:
    def __init__(self, props):
        self.data = props

    def api(self, request_type, q):
        match request_type:
            case "GET":
                return self.props
            case "POST":
                return q
            case _ :
                return None

serve(8685, App("Hello World"))
