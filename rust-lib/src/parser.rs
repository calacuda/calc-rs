#![allow(clippy::upper_case_acronyms)]
use crate::ast::{Node, Operator};
use crate::Result;
use crate::{debug, info};
use lazy_static::lazy_static;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::{self, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::infix(power, Right))
            .op(Op::infix(riemann, Left))
    };
}

fn parse_expr(pairs: Pairs<Rule>, pratt_parser: &PRATT_PARSER) -> Node {
    debug!("pairs -> {:?}", pairs);

    pratt_parser
        .map_primary(|primary| match primary.as_rule() {
            Rule::expr => parse_expr(primary.into_inner(), pratt_parser),
            Rule::num => Node::Num(primary.as_str().parse::<f64>().unwrap()),
            Rule::negative => Node::BinaryExpr {
                op: Operator::Multiply,
                lhs: Box::new(Node::Num(-1.0)),
                rhs: Box::new(parse_expr(primary.into_inner(), pratt_parser)),
            },
            Rule::unary_minus => Node::UnaryExpr(Box::new(parse_expr(
                primary.clone().into_inner(),
                pratt_parser,
            ))),
            Rule::var => Node::Var(primary.as_str().to_string()),
            Rule::range => Node::Range(primary.as_str().to_string()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Operator::Plus,
                Rule::subtract => Operator::Minus,
                Rule::multiply => Operator::Multiply,
                Rule::divide => Operator::Divide,
                Rule::power => Operator::Exponent,
                Rule::unary_minus => Operator::Negative,
                Rule::modulo => Operator::Modulo,
                Rule::riemann => Operator::Riemann,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            debug!("infix => {}{}{}", lhs, op, rhs);
            Node::BinaryExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| {
            debug!("prefix => {} {}", op, rhs);
            match op.as_rule() {
                Rule::unary_minus => Node::UnaryExpr(Box::new(rhs)),
                _ => unreachable!(),
            }
        })
        .parse(pairs)
}

pub fn parse(source: &str) -> Result<Node> {
    let tokens = CalcParser::parse(Rule::equation, source)?;
    debug!("tokens  => {:?}", tokens);
    info!("source  => {:?}", source);
    let parsed = parse_expr(tokens, &PRATT_PARSER);
    info!("parsed  => {:?}", parsed);
    info!("");

    Ok(parsed)
}
