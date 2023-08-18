use wasm_bindgen::prelude::*;

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
