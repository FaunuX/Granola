# Granola User Guide:

## Installation:

Install from PyPi using the following commmand:

``pip install granola-py``

## Hello World:

To make a simple hello world api in Granola, use the following code:

```
# examples/hello_world.py
```

Let's go through the example line by line and explain what's going on.

```python
from granola import serve
```

Imports the serve function from the granola package.

```python
class App:
```

Defines the application class.

```python
def __granola__(self):
```

Defines the application's main route.

```python
return "hello world"
```

Returns the value "Hello World" to the client.

```python
serve(5000, App())
```

Serves the API at localhost:5000.

And that's it! Run the code and then visit localhost:5000 and you should see "hello world".
