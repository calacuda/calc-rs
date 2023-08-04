pub mod ast;
pub mod compiler;
pub mod parser;
pub use crate::ast::{Node, Num, Operator};
pub use crate::compiler::interpreter::Interpreter;
pub use compiler::interpreter::Vars;
use eyre;
use pyo3::prelude::*;
use regex;
use std::collections::HashMap;
use std::sync::Arc;

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

pub fn solve_equ(equation: &str, vars: &Vars) -> eyre::Result<f64> {
    // Ok(match Interpreter::from_source(equation)? {
    //     Num::Int(n) => n as f64,
    //     Num::Float(n) => n,
    // })
    let equ = prepare_equ(equation);
    println!("equ => {:?}", equ);
    Ok(Interpreter::from_source(&equ, vars)?)
}

/// solves a list of equations
#[pyfunction]
fn solve(equations: Vec<&str>) -> PyResult<Vec<Option<f64>>> {
    let vars = Vars::new();

    Ok(equations
        .into_iter()
        .map(|equ| {
            let res = solve_equ(equ, &vars);
            if let Ok(ans) = res {
                Some(ans)
            } else {
                // println!("{res:?}");
                None
            }
        })
        .collect())
}

fn solve_for_domain(
    function: &str,
    start: i64,
    stop: i64,
) -> Result<(String, (Vec<i64>, Vec<Option<f64>>))> {
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
            (start..stop).collect(),
            (start..stop)
                .map(|x| {
                    let mut vars = HashMap::new();
                    vars.insert(arg_name.trim().to_string(), x as f64);
                    let res = Interpreter::from_ast(&ast.clone(), &vars);
                    // println!("{vars:?}");

                    if let Ok(ans) = res {
                        Some(ans)
                    } else {
                        // println!("{res:?}");
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
        let (f_def, ans) = solve_for_domain(f, start, stop)?;
        map.insert(f_def.replace(" ", ""), ans);
    }

    Ok(map)
}

#[pymodule]
fn calc_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve, m)?)?;
    m.add_function(wrap_pyfunction!(solve_funcs, m)?)?;

    Ok(())
}
