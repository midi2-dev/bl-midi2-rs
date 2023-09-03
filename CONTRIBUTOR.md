# Contributing

Contributions are more than welcome!
The library is still in its early development phase. 
There is not even a definite road map at this point.
Please utilise the Issues feature on GitHub 
or open a PR from your own branch to start a discussion ☺️.

## Hooks

We recommend using the hooks while developing in this repository.
The hooks manager is a python package which needs installing locally.
The best way to do this is via a virtual environment.

```shell
> python3 -m venv .venv
> source .venv/bin/activate
> pip install -r requirements.txt
> pre-commit install --install-hooks
```

## Building

To build and run the tests, simply follow the usual cargo flow.
The project currently has a good unit tests coverage,
but no integration test coverage yet.

```shell
> cargo test
```
