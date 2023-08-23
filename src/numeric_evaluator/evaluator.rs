use anyhow::{bail, Result};

use crate::error::EvaluatorError;
use crate::math::{deg_to_rad, rad_to_deg, round};
use crate::parser::{parse, Expr, Op};

fn evaluate_expr(expr: Expr) -> Result<f64> {
    match expr {
        Expr::BinOp { lhs, op, rhs } => match op {
            Op::Add => Ok(evaluate_expr(*lhs)? + evaluate_expr(*rhs)?),
            Op::Subtract => Ok(evaluate_expr(*lhs)? - evaluate_expr(*rhs)?),
            Op::Multiply => Ok(evaluate_expr(*lhs)? * evaluate_expr(*rhs)?),
            Op::Divide => Ok(evaluate_expr(*lhs)? / evaluate_expr(*rhs)?),
            Op::Modulo => Ok((evaluate_expr(*lhs)? % evaluate_expr(*rhs)?).abs()),
            Op::Power => Ok(evaluate_expr(*lhs)?.powf(evaluate_expr(*rhs)?)),
            Op::Equals => bail!(EvaluatorError::EqualityInEval),
        },
        Expr::Number(val) => Ok(val),
        Expr::Constant { value, .. } => Ok(value),
        Expr::UnaryMinus(op) => Ok(-1.0 * evaluate_expr(*op)?),
        Expr::Function { name, args } => match name.as_str() {
            "cos" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(deg_to_rad(evaluate_expr(arg)?).cos())
            }
            "sin" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(deg_to_rad(evaluate_expr(arg)?).sin())
            }
            "tan" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(deg_to_rad(evaluate_expr(arg)?).tan())
            }
            "arccos" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(rad_to_deg(evaluate_expr(arg)?.acos()))
            }
            "arcsin" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(rad_to_deg(evaluate_expr(arg)?.asin()))
            }
            "arctan" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(rad_to_deg(evaluate_expr(arg)?.atan()))
            }
            "abs" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(evaluate_expr(arg)?.abs())
            }
            "floor" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(evaluate_expr(arg)?.floor())
            }
            "ceil" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(evaluate_expr(arg)?.ceil())
            }
            "round" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(evaluate_expr(arg)?.round())
            }
            "trunc" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(evaluate_expr(arg)?.trunc())
            }
            "fract" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(evaluate_expr(arg)?.fract())
            }
            "sqrt" => {
                assert_eq!(args.len(), 1);
                let arg = args.first().unwrap().to_owned();
                Ok(evaluate_expr(arg)?.sqrt())
            }
            "pow" => {
                assert_eq!(args.len(), 2);
                let mut args = args.iter();
                let arg1 = args.next().unwrap().to_owned();
                let arg2 = args.next().unwrap().to_owned();
                Ok(evaluate_expr(arg1)?.powf(evaluate_expr(arg2)?))
            }
            "min" => {
                assert_eq!(args.len(), 2);
                let mut args = args.iter();
                let arg1 = args.next().unwrap().to_owned();
                let arg2 = args.next().unwrap().to_owned();
                Ok(evaluate_expr(arg1)?.min(evaluate_expr(arg2)?))
            }
            "max" => {
                assert_eq!(args.len(), 2);
                let mut args = args.iter();
                let arg1 = args.next().unwrap().to_owned();
                let arg2 = args.next().unwrap().to_owned();
                Ok(evaluate_expr(arg1)?.max(evaluate_expr(arg2)?))
            }
            "avg" => {
                let len = args.len() as f64;
                let sum = args
                    .iter()
                    .fold(0.0, |acc, n| acc + evaluate_expr(n.to_owned()).unwrap());

                Ok(sum / len)
            }
            name => bail!("Syntax error: no function name '{name}' found"),
        },
        _ => todo!(),
    }
}

pub fn evaluate(expression: &str) -> Result<f64> {
    Ok(round(evaluate_expr(parse(expression)?)?, 15))
}
