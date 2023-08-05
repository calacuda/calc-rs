#!/bin/zsh

cp ../target/release/libcalc_rs.so ./calc_rs.so

echo "testing python version:"
hyperfine --runs 50 ' python -c "import calculator as calc; calc.solve_func(\"f(x)= 4+3x+x**2+x**3\", -1000, 1000)" '
echo
echo "testing rust version:"
hyperfine --runs 50 ' python -c "import calc_rs as calc; calc.solve_func(\"f(x)= 4+3x+x^2+x^3\", -1000, 1000)" '
