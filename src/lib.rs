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
    use crate::Result;

    #[test]
    fn equation_solver() -> Result<()> {
        use crate::solve;

        fn test_expr(equation: &str, answer: Option<f64>) -> Result<()> {
            assert_eq!(solve(vec![equation])?, vec![answer]);
            Ok(())
        }

        test_expr("1 + 2 + 3", Some(6.0))?;
        test_expr("1 + 2 + 3 + 4", Some(10.0))?;
        test_expr("1 + 2 + 3 - 4", Some(2.0))?;
        test_expr("4/(10+4)^2", Some(0.02040816326530612))?;
        test_expr("4(10+4)^2", Some(784.0))?;

        Ok(())
    }

    #[test]
    fn function_solver() -> Result<()> {
        use crate::solve_func;

        fn test_expr(equation: &str, answers: Vec<(i64, Option<f64>)>) -> Result<()> {
            let is = solve_func(equation, -2, 2)?.1;
            println!("{:?}", is);
            let mut should_be: (Vec<i64>, Vec<Option<f64>>) =
                (Vec::with_capacity(5), Vec::with_capacity(5));

            for (x, y) in answers {
                should_be.0.push(x);
                should_be.1.push(y);
            }

            assert_eq!(is, should_be);

            Ok(())
        }

        // vec![(-2, Some()), (-1, Some()), (0, Some()), (1, Some()), (2, Some())]
        test_expr(
            "f(x) = 0.1x^3",
            vec![
                (-2, Some(-0.8)),
                (-1, Some(-0.1)),
                (0, Some(0.0)),
                (1, Some(0.1)),
                (2, Some(0.8)),
            ],
        )?;
        test_expr(
            "g(x) = 1/x",
            vec![
                (-2, Some(-0.5)),
                (-1, Some(-1.0)),
                (0, None),
                (1, Some(1.0)),
                (2, Some(0.5)),
            ],
        )?;
        test_expr(
            "h(x) = 15x^2",
            vec![
                (-2, Some(60.0)),
                (-1, Some(15.0)),
                (0, Some(0.0)),
                (1, Some(15.0)),
                (2, Some(60.0)),
            ],
        )?;

        Ok(())
    }
}
