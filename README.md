# calc-rs

a calculator module for Python and Rust capable of computing arbitrarily long equations and functions.

## Speed Test

These tests were done using [hyperfine](https://github.com/sharkdp/hyperfine) and the [speed-test.sh](test/speed-test.sh) script in the test folder

```
testing python version:
Benchmark 1:  python -c "import calculator as calc; calc.solve_func(\"f(x)= 4+3x+x**2+x**3\", -1000, 1000)"
  Time (mean ± σ):      1.150 s ±  0.018 s    [User: 1.130 s, System: 0.016 s]
  Range (min … max):    1.126 s …  1.231 s    50 runs


testing rust version:
Benchmark 1:  python -c "import calculators as calc; calc.solve_func(\"f(x)= 4+3x+x^2+x^3\", -1000, 1000)"
  Time (mean ± σ):      11.8 ms ±   0.8 ms    [User: 10.6 ms, System: 3.6 ms]
  Range (min … max):    11.1 ms …  14.5 ms    25 runs
```

As you can see the rust version is significantly faster.

## Usage

there are three functions that this libray exposes:
| **Function** | **Args** | **Description** |
| -- | -- | -- |
| `solve` | equasions: list(str) | takes a list of equations as strings and solves eatch one returning a list of answers. |
| `solve_func` | f: str, start: int, stop: int | solves the function, f, at all integer points between start and stop. |
| `solve_funcs` | fs: list(str), start: int, stop: int | solves a list of functions, fs, for all integer points between start and stop. returns a dictionary whose keys are function names and whose values are a tuple of a list of x values and a list of y values. |

## Installation

with pip:

```
pip install calc-rs
```
