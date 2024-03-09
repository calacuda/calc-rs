#!/usr/bin/env zsh

error-out() {
  echo "*** this code must be run from repos root! ***"
  exit 1
}

echo "building newest lib version"
maturin build --release || error-out 
echo "built"

cd ./tests

cp ../target/release/libcalculators.so ./calculators.so

echo "starting tests"

echo "testing python version:"
hyperfine --show-output --runs 25 ' python -c "import calculator as calc; calc.solve_func(\"f(x)= 4+3x+x**2+x**3\", -1000, 1000)" '
echo
echo "testing rust version:"
hyperfine --show-output --runs 25 ' python -c "import calculators as calc; calc.solve_func(\"f(x)= 4+3x+x^2+x^3\", -1000, 1000)" '
