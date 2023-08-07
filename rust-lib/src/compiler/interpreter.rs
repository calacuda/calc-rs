use crate::{debug, Compile, Node, Operator, Result};
use eyre::bail;
use regex::Regex;
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

    fn modulo(&self, lhs: Num, rhs: Num) -> Num {
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => Some(lhs % rhs),
            _ => None,
        }
    }

    fn riemann_sum(&self, lhs: &Node, rhs: &Node, vars: &Vars) -> Result<Num> {
        let left = if let Node::Range(range) = lhs {
            range
        } else {
            bail!("none range before Rieman sum");
        };

        let re = Regex::new(r"([a-zA-Z])=(\d*)_(\d*)").unwrap();
        // get range and var name
        let (name, start, stop) = if let Some(cap) = re.captures(left) {
            (
                cap[1].to_string(),
                cap[2].parse::<i64>().unwrap(),
                cap[3].parse::<i64>().unwrap(),
            )
        } else {
            bail!("improperly formatted index variable range.");
        };

        let mut loc_vars = vars.clone();
        let mut total = 0.0;

        for i in start..=stop {
            loc_vars.insert(name.clone(), i as f64);

            if let Some(n) = self.eval(rhs, &loc_vars)? {
                total += n;
            } else {
                return Ok(None);
            };
        }

        Ok(Some(total))
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
                debug!("interpreter found a unary operator applied to {:?}", val);
                Ok(val)
            }
            Node::BinaryExpr { op, lhs, rhs } => match op {
                Operator::Exponent => Ok(self.exp(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Divide => Ok(self.div(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Multiply => Ok(self.mul(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Plus => Ok(self.add(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Minus => Ok(self.sub(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Modulo => Ok(self.modulo(self.eval(lhs, vars)?, self.eval(rhs, vars)?)),
                Operator::Riemann => Ok(self.riemann_sum(lhs, rhs, vars)?),
                Operator::Negative => unreachable!("negative numbers can only have one operand"),
            },
            Node::Range(_) => unreachable!("tryed to eval a range which don't get evaluated."),
        }
    }
}
