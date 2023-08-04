use std::fmt;

pub type Num = f64;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
// ANCHOR: operator
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Negative,
    // Modulo,
}
// ANCHOR_END: operator

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus | Operator::Negative => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Exponent => write!(f, "^"),
            // Operator::Modulo => write!(f, "%"),
        }
    }
}

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub enum Num {
//     Int(i128),
//     Float(f64),
// }

#[derive(Debug, Clone, PartialEq, PartialOrd)]
// ANCHOR: node
pub enum Node {
    Var(String),
    Num(f64),
    UnaryExpr(Box<Node>),
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
// ANCHOR_END: node

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            // Node::Int(n) => write!(f, "{}", n),
            // Node::Float(n) => write!(f, "{}", n),
            // Node::Num(Num::Int(n)) => write!(f, "{}", n),
            Node::Num(n) => write!(f, "{}", n),
            // Node::NegNum(Num::Int(n)) => write!(f, "{}", n),
            // Node::NegNum(Num::Float(n)) => write!(f, "{}", n),
            Node::UnaryExpr(expr) => write!(f, "{}", expr),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
            Node::Var(var_name) => write!(f, "{}", var_name),
        }
    }
}
