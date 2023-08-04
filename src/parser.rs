#![allow(clippy::upper_case_acronyms)]

use crate::ast::{Node, Num, Operator};
use crate::Result;
use lazy_static::lazy_static;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::{self, Parser};

// ANCHOR: parser
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;
// ANCHOR_END: parser

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(power, Right))
    };
}

fn parse_expr(pairs: Pairs<Rule>, pratt_parser: &PRATT_PARSER) -> Node {
    // println!("pairs -> {:?}", pairs);

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
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            // println!("infix => {}{}{}", lhs, op, rhs);
            Node::BinaryExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| {
            // println!("prefix => {} {}", op, rhs);
            match op.as_rule() {
                Rule::unary_minus => Node::UnaryExpr(Box::new(rhs)),
                _ => unreachable!(),
            }
        })
        .parse(pairs)
}

pub fn parse(source: &str) -> Result<Node> {
    let tokens = CalcParser::parse(Rule::equation, source)?;
    // println!("tokens  => {:?}", tokens);
    // println!("source  => {:?}", source);
    let parsed = parse_expr(tokens, &PRATT_PARSER);
    // println!("parsed  => {:?}", parsed);
    // println!();

    Ok(parsed)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn nested_expr() {
//         fn test_expr(expected: &str, src: &str) {
//             assert_eq!(
//                 expected,
//                 parse(src)
//                     .unwrap()
//                     .iter()
//                     .fold(String::new(), |acc, arg| acc + &format!("{}", &arg))
//             );
//         }
//
//         test_expr("1 + 2 + 3", "(1 + 2) + 3");
//         test_expr("1 + 2 + 3", "1 + (2 + 3)");
//         test_expr("1 + 2 + 3 + 4", "1 + (2 + (3 + 4))");
//         test_expr("1 + 2 + 3 - 4", "(1 + 2) + (3 - 4)");
//     }
// }
