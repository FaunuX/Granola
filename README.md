# Granola

An Explosively Fast Object-Oriented Python Backend Microframework Written in Rust

## Description

Our library, called "Granola", is designed to provide developers with a fast and reliable way to build backend web applications in Python. It utilizes the power of the Rust programming language to provide efficient, high-performance functionality while maintaining the simplicity and ease of use of Python.

With Granola, developers can easily define and work with objects in their web application, leveraging the power of object-oriented programming to create clean and reusable code. The library includes a variety of useful tools for working with HTTP requests and responses, including support for routing, middleware, and handling different types of content.

## Features

- Built for python, a popular and versatile programming language that is easy to learn and use.
- Monstrously fast performance, thanks to being written in Rust.
- Object-Oriented, easy-to-use API for building backend applications quickly.
- Built-in support for common web development features, such as routing and request handling.
- JWT (maybe idk)
- Automatic Markdown documentation generator for API docs with no extra effort. (maybe idk)

## Installation

To install Granola, you will need to have [Python](https://www.python.org/) installed on your system. Then, you can install Granola using `pip`, the Python package manager, by running the following command:

`pip install beserk`

## Usage

Here is a simple example of a web server built with Granola:

```py
# examples/example.py

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

serve([App("app", "Hello World")])

```

## Contributing

We welcome contributions to Granola. If you have an idea for a new feature or have found a bug, please open an issue on our GitHub repository. If you would like to contribute code, please open a pull request with your changes.
