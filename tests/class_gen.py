import random
import string

functions = [
        lambda self: "Hello, World!",    # returns a constant string
        lambda self: 42             ,  # returns a constant number
        lambda self: [1, 2, 3]      ,  # returns a constant list
        lambda self: {"a": 1, "b": 2}, # returns a constant dictionary
        lambda self: None           ,  # returns None
        lambda self, request: "Hello, World!",    # returns a constant string
        lambda self, request: 42             ,  # returns a constant number
        lambda self, request: [1, 2, 3]      ,  # returns a constant list
        lambda self, request: {"a": 1, "b": 2}, # returns a constant dictionary
        lambda self, request: None           ,  # returns None
]

def generate_random_class():
    class_name = ''.join(random.choices(string.ascii_uppercase, k=random.randint(5, 10)))

    class_body = {}

    for i in range(random.randint(2, 5)):
        method_name = ''.join(random.choices(string.ascii_lowercase, k=random.randint(5, 10)))
        method_body = random.choice(functions)
        class_body[method_name] = method_body
        class_body['__granola__'] = random.choice(functions)

    random_class = type(class_name, (object,), class_body)

    return random_class()
