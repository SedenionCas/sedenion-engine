use std::f64::consts::{E, PI, TAU};

use phf::phf_map;

pub static CONSTANTS_DATABASE: phf::Map<&'static str, f64> = phf_map! {
    "pi" => PI,
    "tau" => TAU,
    "e" => E,
    "phi" => 1.61803398874989484820,
};
