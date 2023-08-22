use parser::parse;
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
    let optimized = parsed.optimize_expression(String::from("X")).as_latex();
    Ok(optimized)
}
