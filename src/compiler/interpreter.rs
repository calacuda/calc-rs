use std::collections::HashMap;

use eyre::bail;

use crate::ast::Num;
use crate::{Compile, Node, Operator, Result};

pub type Vars = HashMap<String, f64>;

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

    // fn solve(&self, node: &Node) -> Node {}

    fn add(&self, lhs: Num, rhs: Num) -> Num {
        lhs + rhs
        // match (lhs, rhs) {
        //     (Num::Int(i1), Num::Int(i2)) => Num::Int(i1 + i2),
        //     (Num::Float(f1), Num::Float(f2)) => Num::Float(f1 + f2),
        //     (Num::Int(i1), Num::Float(f2)) => Num::Float(i1 as f64 + f2),
        //     (Num::Float(f1), Num::Int(i2)) => Num::Float(f1 + i2 as f64),
        // }
    }

    fn sub(&self, lhs: Num, rhs: Num) -> Num {
        lhs - rhs
        // match (lhs, rhs) {
        //     (Num::Int(i1), Num::Int(i2)) => Num::Int(i1 - i2),
        //     (Num::Float(f1), Num::Float(f2)) => Num::Float(f1 - f2),
        //     (Num::Int(i1), Num::Float(f2)) => Num::Float(i1 as f64 - f2),
        //     (Num::Float(f1), Num::Int(i2)) => Num::Float(f1 - i2 as f64),
        // }
    }

    fn mul(&self, lhs: Num, rhs: Num) -> Num {
        lhs * rhs
        // match (lhs, rhs) {
        //     (Num::Int(i1), Num::Int(i2)) => Num::Int(i1 * i2),
        //     (Num::Float(f1), Num::Float(f2)) => Num::Float(f1 * f2),
        //     (Num::Int(i1), Num::Float(f2)) => Num::Float(i1 as f64 * f2),
        //     (Num::Float(f1), Num::Int(i2)) => Num::Float(f1 * i2 as f64),
        // }
    }

    fn div(&self, lhs: Num, rhs: Num) -> Num {
        lhs / rhs
        // match (lhs, rhs) {
        //     (Num::Int(i1), Num::Int(i2)) => Num::Int(i1 / i2),
        //     (Num::Float(f1), Num::Float(f2)) => Num::Float(f1 / f2),
        //     (Num::Int(i1), Num::Float(f2)) => Num::Float(i1 as f64 / f2),
        //     (Num::Float(f1), Num::Int(i2)) => Num::Float(f1 / i2 as f64),
        // }
    }

    fn exp(&self, lhs: Num, rhs: Num) -> Num {
        lhs.powf(rhs)
        // match (lhs, rhs) {
        //     (Num::Int(i1), Num::Int(i2)) => Num::Int(i1.pow(i2 as u32)),
        //     (Num::Float(f1), Num::Float(f2)) => Num::Float(f1.powf(f2)),
        //     (Num::Int(i1), Num::Float(f2)) => Num::Float((i1 as f64).powf(f2)),
        //     (Num::Float(f1), Num::Int(i2)) => Num::Float(f1.powf(i2 as f64)),
        // }
    }

    pub fn eval(&self, node: &Node, vars: &Vars) -> Result<Num> {
        match node {
            Node::Num(n) => Ok(n.clone()),
            Node::Var(var) => {
                if let Some(val) = vars.get(var) {
                    Ok(*val)
                } else {
                    eyre::bail!("unknown variable: {var}")
                }
            }
            Node::UnaryExpr(expr) => {
                let val = self.eval(expr, vars)?;
                println!("interpreter found the unary operator applied to {:?}", val);
                Ok(val)
                // if let Num::Int(n) = val {
                //     Ok(Num::Int(-1 * n))
                // } else if let Num::Float(n) = val {
                //     Ok(Num::Float(-1.0 * n))
                // } else {
                //     unreachable!("the negative sign can not be applied to operations")
                // }
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
