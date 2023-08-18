use std::io::{self, BufRead};

use anyhow::Result;
use sedenion_engine::numeric_evaluator::evaluate;

fn main() -> Result<()> {
    let stdin = io::stdin();
    loop {
        let expression;
        {
            expression = stdin.lock().lines().next().unwrap().unwrap();
        }

        if expression == "exit" {
            break;
        }

        let result = evaluate(&expression);
        println!("= {:?}", result);
    }

    Ok(())
}
