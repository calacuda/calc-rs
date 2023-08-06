use std::fmt;

pub type Number = f64;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Negative,
    Modulo,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus | Operator::Negative => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Exponent => write!(f, "^"),
            Operator::Modulo => write!(f, "%"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Node {
    Var(String),
    Num(Number),
    UnaryExpr(Box<Node>),
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Num(n) => write!(f, "{}", n),
            Node::UnaryExpr(expr) => write!(f, "{}", expr),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
            Node::Var(var_name) => write!(f, "{}", var_name),
        }
    }
}
