# Contributing

Contributions are more than welcome!
The library is still in its early development phase. 
There is not even a definite road map at this point.
Please utilise the Issues feature on GitHub 
or open a PR from your own fork to start a discussion ☺️..

## 🪝 Hooks 🪝

It is recommended to use git hooks while developing in this repository.
The hooks manager (pre-commit) is a python package which needs installing locally.
The best way to do this is via a virtual environment.

```shell
> python3 -m venv .venv
> source .venv/bin/activate
> pip install -r requirements.txt
> ./configure-hooks.sh install
```

## 🧱 Building 🧱

To build, simply follow the usual cargo flow.

```shell
> cargo build --all --all-features
```

## Tests

The project currently has a unit test, and documentation test coverage,
but no integration test coverage yet.
To run all the tests, follow the usual cargo flow.

When adding new features please try to cover the new code
with appropriate unit test cases.

```shell
> cargo test --all --all-features
```

## 🌍 A Tour of midi2 🌍 

todo
