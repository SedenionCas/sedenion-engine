#![allow(clippy::module_inception)]
mod parser;
mod token;

pub use parser::{parse, parse_equation};
pub use token::{Expr, Op, Optimize};
