#![allow(clippy::approx_constant)]
use parser::{parse, parse_equation};
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate log;

mod error;
mod math;
pub mod numeric_evaluator;
mod optimizer;
mod parser;

#[cfg(test)]
mod tests;

#[wasm_bindgen]
pub fn evaluate(expression: &str) -> Result<f64, String> {
    match numeric_evaluator::evaluate(expression) {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string()),
    }
}

#[wasm_bindgen]
pub fn optimize(expression: &str) -> Result<String, String> {
    let parsed = match parse(expression) {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string()),
    }?;

    let optimized = parsed.optimize_expression(String::from("X"));
    Ok(optimized.as_latex())
}

#[wasm_bindgen]
pub fn optimize_equation(expression: &str, target: &str) -> Result<String, String> {
    let parsed = match parse_equation(expression) {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string()),
    }?;

    let optimized = parsed.optimize_equation(String::from(target));
    Ok(optimized.as_latex())
}
