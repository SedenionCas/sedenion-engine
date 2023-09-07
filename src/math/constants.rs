use std::f64::consts::{E, PI, TAU};

use phf::phf_map;

pub static CONSTANTS_DATABASE: phf::Map<&'static str, f64> = phf_map! {
    "PI" => PI,
    "TAU" => TAU,
    "E" => E,
    "PHI" => 1.618_033_988_749_895,
};
