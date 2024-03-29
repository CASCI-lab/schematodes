# Schematodes
A tool for expressing permutation symmetries in sets of tuples. 

Inspired by the study of input symmetry in Boolean network regulatory functions, and developed with [cana](https://github.com/CASCI-lab/CANA) integration in mind.

## Installing
Schematodes is available on PyPI:
```shell
pip install schematodes
```

## Building
We wrote `schematodes` in `rust` for `python` with `pyO3` bindings using `maturin`. The recommended build steps are as follows:

1. Make sure `rust` is installed. See the [rust website](https://www.rust-lang.org/tools/install) for details.
2. Install `maturin`: `pip install maturin`.
3. For testing and development, tuild using the command `maturin develop` in the root directory of this repository. This will build and install `schematodes` in your current `virtualenv`.
4. For distribution building, you can use `maturin build --release --out dist --find-interpreter`. However, this should be done automatically by GitHub when pushing to main.
