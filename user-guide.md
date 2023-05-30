# Granola User Guide:

## Installation:

Install from PyPi using the following commmand:

``pip install granola-py``

## Hello World:

To make a simple hello world api in Granola, use the following code:

```py
# examples/hello_world.py

from granola import serve

class App:
    def __granola__(self):
        return "hello world"

serve(5000, App())

```

Let's go through the example line by line and explain what's going on.

```py
from granola import serve
```

Imports the serve function from the granola package.

```py
class App:
```

Defines the application class.

```py
def __granola__(self):
```

Defines the application's main route.

```py
return "hello world"
```

Returns the value "Hello World" to the client.

```py
serve(5000, App())
```

Serves the API at localhost:5000.

And that's it! Run the code and then visit localhost:5000 and you should see "hello world".
