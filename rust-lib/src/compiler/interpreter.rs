// use crate::ast::Num;
use crate::{Compile, Node, Operator, Result};
use std::collections::HashMap;

pub type Vars = HashMap<String, f64>;
pub type Num = Option<crate::ast::Number>;

pub struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<Num>;

    fn from_ast(ast: &Node, vars: &Vars) -> Self::Output {
        let evaluator = Eval::new();

        evaluator.eval(ast, &vars)
    }
}

struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self
    }

    fn add(&self, lhs: Num, rhs: Num) -> Num {
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => Some(lhs + rhs),
            _ => None,
        }
    }

    fn sub(&self, lhs: Num, rhs: Num) -> Num {
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => Some(lhs - rhs),
            _ => None,
        }
    }

    fn mul(&self, lhs: Num, rhs: Num) -> Num {
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => Some(lhs * rhs),
            _ => None,
        }
    }

    fn div(&self, lhs: Num, rhs: Num) -> Num {
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) if rhs != 0.0 => Some(lhs / rhs),
            _ => None,
        }
    }

    fn exp(&self, lhs: Num, rhs: Num) -> Num {
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => Some(lhs.powf(rhs)),
            _ => None,
        }
    }

    pub fn eval(&self, node: &Node, vars: &Vars) -> Result<Num> {
        match node {
            Node::Num(n) => Ok(Some(n.clone())),
            Node::Var(var) => {
                if let Some(val) = vars.get(var) {
                    Ok(Some(*val))
                } else {
                    crate::bail!("unknown variable: {var}")
                }
            }
            Node::UnaryExpr(expr) => {
                let val = self.eval(expr, vars)?;
                // println!("interpreter found a unary operator applied to {:?}", val);
                Ok(val)
            }
            Node::BinaryExpr { op, lhs, rhs } => match op {
                Operator::Exponent => Ok(self.exp(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Divide => Ok(self.div(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Multiply => Ok(self.mul(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Plus => Ok(self.add(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Minus => Ok(self.sub(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Negative => unreachable!("negative numbers can only have one operand"),
            },
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn basics() {
//         assert_eq!(Interpreter::from_source("1 + 2").unwrap() as i32, 3);
//         // assert_eq!(Interpreter::source("(1 + 2)").unwrap() as i32, 3);
//         assert_eq!(Interpreter::from_source("2 + (2 - 1)").unwrap() as i32, 3);
//         assert_eq!(Interpreter::from_source("(2 + 3) - 1").unwrap() as i32, 4);
//         assert_eq!(
//             Interpreter::from_source("1 + ((2 + 3) - (2 + 3))").unwrap() as i32,
//             1
//         );
//     }
// }
