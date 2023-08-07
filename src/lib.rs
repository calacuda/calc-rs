use calc_rs;
use calc_rs::Num;
use calc_rs::Vars;
use pyo3::prelude::*;
use pyo3::PyResult;
use std::collections::HashMap;

#[pyfunction]
pub fn solve_equ(equation: &str, vars: Vars) -> PyResult<Num> {
    Ok(calc_rs::solve_equ(equation, &vars)?)
}

/// solves a list of equations
#[pyfunction]
pub fn solve_equs(equations: Vec<&str>) -> PyResult<Vec<Num>> {
    Ok(calc_rs::solve_equs(equations)?)
}

/// solves a single function, given a start and end of domain
#[pyfunction]
pub fn solve_func(
    function: &str,
    start: i64,
    stop: i64,
) -> PyResult<(String, (Vec<i64>, Vec<Num>))> {
    Ok(calc_rs::solve_func(function, start, stop)?)
}

/// solves functions and returns a python dictionary that maps function name to (x_values, y_valiues)
#[pyfunction]
pub fn solve_funcs(
    functions: Vec<&str>,
    start: i64,
    stop: i64,
) -> PyResult<HashMap<String, (Vec<i64>, Vec<Num>)>> {
    Ok(calc_rs::solve_funcs(functions, start, stop)?)
}

#[pymodule]
fn calculators(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_equs, m)?)?;
    m.add_function(wrap_pyfunction!(solve_equ, m)?)?;
    m.add_function(wrap_pyfunction!(solve_funcs, m)?)?;
    m.add_function(wrap_pyfunction!(solve_func, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // tests are handled by the underlying library found in the rust-lib subdir. go there to run
    // tests
}
