pub mod ast;
pub mod compiler;
pub mod parser;
pub use crate::ast::{Node, Operator};
pub use crate::compiler::interpreter::Interpreter;
pub use crate::compiler::interpreter::Num;
pub use compiler::interpreter::Vars;
pub use eyre::{bail, Result};
use rayon::prelude::*;
use regex;
use std::collections::HashMap;

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

fn prepare_equ(equation: &str) -> String {
    let mut re = regex::Regex::new(r"([\da-zA-Z])[ ]{0,1}([a-zA-Z\(])").unwrap();
    let mut equ = re.replacen(equation, 0, "$1 * $2").to_string();

    re = regex::Regex::new(r"\)[ ]{0,1}([\da-zA-Z])").unwrap();
    equ = re.replacen(&equ, 0, "$1 * $2").to_string();

    equ
}

pub fn solve_equ(equation: &str, vars: &Vars) -> Result<Num> {
    let equ = prepare_equ(equation);
    // println!("equ => {:?}", equ);
    Ok(Interpreter::from_source(&equ, vars)?)
}

/// solves a list of equations
pub fn solve_equs(equations: Vec<&str>) -> Result<Vec<Num>> {
    let vars = Vars::new();

    Ok(equations
        .par_iter()
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
pub fn solve_func(function: &str, start: i64, stop: i64) -> Result<(String, (Vec<i64>, Vec<Num>))> {
    let Some((f_name, f_def)) = function.split_once("=") else { bail!("function definitions require and equals sign.") };
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
                .into_par_iter()
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
pub fn solve_funcs(
    functions: Vec<&str>,
    start: i64,
    stop: i64,
) -> Result<HashMap<String, (Vec<i64>, Vec<Num>)>> {
    let mut map = HashMap::new();

    for f in functions {
        let (f_def, ans) = solve_func(f, start, stop)?;
        map.insert(f_def.trim().to_string(), ans);
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use crate::Result;
    use std::collections::HashMap;

    #[test]
    fn equation_solver() -> Result<()> {
        use crate::{solve_equ, solve_equs};

        fn test_expr(equation: &str, answer: Option<f64>) -> Result<()> {
            assert_eq!(solve_equs(vec![equation])?, vec![answer]);
            assert_eq!(solve_equ(equation, &HashMap::new())?, answer);
            Ok(())
        }

        test_expr("1 + 2 + 3", Some(6.0))?;
        test_expr("1 + 2 + 3 + 4", Some(10.0))?;
        test_expr("1 + 2 + 3 - 4", Some(2.0))?;
        test_expr("4/(10+4)^2", Some(0.02040816326530612))?;
        test_expr("4(10+4)^2", Some(784.0))?;
        test_expr("5%2", Some(1.0))?;
        test_expr("15%4", Some(3.0))?;

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
