use std::f64::consts::PI;

pub fn deg_to_rad(a: f64) -> f64 {
    a*(PI/180.0)
}

pub fn rad_to_deg(x: f64) -> f64 {
    x*(180.0/PI)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::math::{deg_to_rad, angle::rad_to_deg};

    #[test]
    fn can_convert_degrees_to_radians() {
        assert_eq!(PI/2.0, deg_to_rad(90.0));
        assert_eq!(PI, deg_to_rad(180.0));
        assert_eq!(2.0*PI, deg_to_rad(360.0));
    }

    #[test]
    fn can_convert_radians_to_degrees() {
        assert_eq!(90.0, rad_to_deg(PI/2.0));
        assert_eq!(180.0, rad_to_deg(PI));
        assert_eq!(360.0, rad_to_deg(2.0*PI));
    }
}