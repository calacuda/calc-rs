pub mod ast;
pub mod compiler;
pub mod parser;
pub use crate::ast::{Node, Operator};
pub use crate::compiler::interpreter::Interpreter;
pub use crate::compiler::interpreter::Num;
pub use compiler::interpreter::Vars;
use eyre;
use pyo3::prelude::*;
use regex;
use std::collections::HashMap;

pub type Result<T> = eyre::Result<T>;

pub trait Compile {
    type Output;

    fn from_ast(ast: &Node, vars: &Vars) -> Self::Output;

    fn from_source(source: &str, vars: &Vars) -> Self::Output {
        // println!("Compiling the source: {}", source);
        let ast: Node = parser::parse(source).unwrap();
        // println!("ast => {:?}", ast);
        Self::from_ast(&ast, vars)
    }
}

fn prepare_equ(equ: &str) -> String {
    let re = regex::Regex::new(r"([\da-zA-Z])([a-zA-Z\(])").unwrap();
    let equ = re.replacen(equ, 0, "$1 * $2");

    equ.to_string()
}

pub fn solve_equ(equation: &str, vars: &Vars) -> eyre::Result<Num> {
    let equ = prepare_equ(equation);
    // println!("equ => {:?}", equ);
    Ok(Interpreter::from_source(&equ, vars)?)
}

/// solves a list of equations
#[pyfunction]
fn solve(equations: Vec<&str>) -> PyResult<Vec<Num>> {
    let vars = Vars::new();

    Ok(equations
        .into_iter()
        .map(|equ| {
            let res = solve_equ(equ, &vars);
            if let Ok(ans) = res {
                ans
            } else {
                println!("{res:?}");
                None
            }
        })
        .collect())
}

/// solves a single function, given a start and end of domain
#[pyfunction]
fn solve_func(function: &str, start: i64, stop: i64) -> Result<(String, (Vec<i64>, Vec<Num>))> {
    let Some((f_name, f_def)) = function.split_once("=") else { eyre::bail!("function definitions require and equals sign.") };
    let arg_name = f_name
        .split_once("(")
        .unwrap_or(("", "x)"))
        .1
        .replace(")", "");
    let ast = parser::parse(prepare_equ(&f_def).as_str())?;

    Ok((
        f_name.to_string(),
        (
            (start..=stop).collect(),
            (start..=stop)
                .map(|x| {
                    let mut vars = HashMap::new();
                    vars.insert(arg_name.trim().to_string(), x as f64);
                    let res = Interpreter::from_ast(&ast.clone(), &vars);
                    // println!("{vars:?}");

                    if let Ok(ans) = res {
                        ans
                    } else {
                        println!("{res:?}");
                        None
                    }
                })
                .collect(),
        ),
    ))
}

/// solves functions and returns a python dictionary that maps function name to (x_values, y_valiues)
#[pyfunction]
pub fn solve_funcs(
    functions: Vec<&str>,
    start: i64,
    stop: i64,
) -> PyResult<HashMap<String, (Vec<i64>, Vec<Option<f64>>)>> {
    let mut map = HashMap::new();

    for f in functions {
        let (f_def, ans) = solve_func(f, start, stop)?;
        map.insert(f_def.replace(" ", ""), ans);
    }

    Ok(map)
}

#[pymodule]
fn calc_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve, m)?)?;
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

        // [(-2, Some()), (-1, Some()), (0, Some()), (1, Some()), (2, Some())]
        // TODO: add a function that is undefined between x=-2 and x=2 to test that it returns None
        // at that point.
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

        // test_expr("", [(-2, Some()), (-1, Some()), (0, Some()), (1, Some()), (2, Some())])?;
        // test_expr("4/(10+4)^2", 0.02040816326530612)?;
        // test_expr("4(10+4)^2", 784)?;

        Ok(())
    }
}
