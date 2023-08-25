use anyhow::{bail, Result};
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;

use crate::error::ParserError;
use crate::math::CONSTANTS_DATABASE;

use super::{Expr, Op};

#[derive(pest_derive::Parser)]
#[grammar = "grammar/sedenion.pest"]
pub(crate) struct CalculatorParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::infix(power, Right))
            .op(Op::prefix(unary_minus))
            .op(Op::infix(equals, Left))
        };
}

fn parse_function(pairs: Pairs<Rule>) -> Result<Expr> {
    let mut name = String::new();
    let mut args: Vec<Expr> = Vec::new();
    println!("{:#?}", pairs);

    for pair in pairs {
        match pair.as_rule() {
            Rule::function_name => name = String::from(pair.as_str()),
            Rule::function_args => {
                args = pair
                    .into_inner()
                    .map(|arg| parse_expr(arg.into_inner()))
                    .map(|arg| arg.unwrap())
                    .collect()
            }
            rule => {
                bail!(ParserError::InvalidToken(format!("{:?}", rule)))
            }
        }
    }

    if !name.is_empty() {
        Ok(Expr::Function { name, args })
    } else {
        bail!(ParserError::NoFunctionName)
    }
}

fn parse_monomial(pairs: Pairs<Rule>) -> Result<Expr> {
    let mut coefficient: Option<f64> = None;
    let mut exponent: Option<f64> = None;
    let mut variable: Option<String> = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::coefficient => coefficient = Some(pair.as_str().parse::<f64>()?),
            Rule::variable => variable = Some(pair.as_str().to_string()),
            Rule::exponent => {
                let pair = match pair.as_str().strip_prefix('^') {
                    Some(val) => val,
                    None => bail!(ParserError::InvalidToken(format!("{:?}", pair.as_str()))),
                };
                exponent = Some(pair.parse::<f64>()?);
            }
            rule => bail!(ParserError::InvalidToken(format!("{:?}", rule))),
        }
    }

    Ok(Expr::Monomial {
        coefficient: coefficient.unwrap_or(1.0),
        variable: variable.unwrap(),
        exponent: exponent.unwrap_or(1.0),
    })
}

fn parse_expr(pairs: Pairs<Rule>) -> Result<Expr> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Ok(Expr::Number(primary.as_str().parse::<f64>().unwrap())),
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::function => parse_function(primary.into_inner()),
            Rule::monomial => parse_monomial(primary.into_inner()),
            Rule::constant => {
                let name = primary.as_str().to_string();
                let value = *match CONSTANTS_DATABASE.get(&name) {
                    Some(val) => val,
                    None => bail!(ParserError::UnknownConstant(name.to_string())),
                };

                Ok(Expr::Constant { name, value })
            }
            rule => bail!(ParserError::InvalidToken(format!("{:?}", rule))),
        })
        .map_infix(|lhs, op, rhs| {
            let op: Result<Op> = match op.as_rule() {
                Rule::add => Ok(Op::Add),
                Rule::subtract => Ok(Op::Subtract),
                Rule::multiply => Ok(Op::Multiply),
                Rule::divide => Ok(Op::Divide),
                Rule::modulo => Ok(Op::Modulo),
                Rule::power => Ok(Op::Power),
                rule => bail!(ParserError::InvalidOperator(format!("{:?}", rule))),
            };
            let op = op?;
            Ok(Expr::BinOp {
                lhs: Box::new(lhs?),
                op,
                rhs: Box::new(rhs?),
            })
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Ok(Expr::UnaryMinus(Box::new(rhs?))),
            rule => bail!(ParserError::InvalidToken(format!("{:?}", rule))),
        })
        .parse(pairs)
}

pub fn parse(expression: &str) -> Result<Expr> {
    let mut pairs = CalculatorParser::parse(Rule::equation, expression)?;
    parse_expr(pairs.next().unwrap().into_inner())
}

pub fn parse_equation(expression: &str) -> Result<Expr> {
    if !expression.contains('=') {
        bail!(ParserError::NoEquals);
    }

    let expression: Vec<&str> = expression.split('=').collect();
    if expression.len() != 2 {
        bail!(ParserError::EqualsCount);
    }

    let lhs = parse(expression[0])?;
    let rhs = parse(expression[1])?;

    Ok(Expr::BinOp {
        lhs: Box::new(lhs),
        op: Op::Equals,
        rhs: Box::new(rhs),
    })
}
