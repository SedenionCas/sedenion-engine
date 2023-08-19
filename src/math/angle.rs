use std::f64::consts::PI;

pub fn deg_to_rad(a: f64) -> f64 {
    a*(PI/180.0)
}

pub fn rad_to_deg(x: f64) -> f64 {
    x*(180.0/PI)
}
