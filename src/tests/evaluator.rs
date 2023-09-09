#[cfg(test)]
mod test {
    use crate::{math::round, numeric_evaluator::evaluate};

    fn setup(expr: &str) -> f64 {
        round(evaluate(expr).unwrap(), 14)
    }

    #[test]
    fn can_eval_plus() {
        assert_eq!(7.0, setup("2+5"));
        assert_eq!(-7.0, setup("-2+-5"));
        assert_eq!(14.0, setup("2+5+7"));
    }

    #[test]
    fn can_eval_minus() {
        assert_eq!(-4.0, setup("3-7"));
        assert_eq!(4.0, setup("-3--7"));
        assert_eq!(-8.0, setup("3-7-4"));
    }

    #[test]
    fn can_eval_multiply() {
        assert_eq!(18.0, setup("6*3"));
        assert_eq!(18.0, setup("-6*-3"));
        assert_eq!(144.0, setup("6*3*8"));
    }

    #[test]
    fn can_eval_divide() {
        assert_eq!(0.1, setup("1/10"));
        assert_eq!(0.1, setup("-1/-10"));
        assert_eq!(0.02, setup("1/10/5"));
    }

    #[test]
    fn can_eval_modulus() {
        assert_eq!(1.0, setup("3%2"));
        assert_eq!(1.0, setup("-3%-2"));
        assert_eq!(1.0, setup("3%2%3"));
    }

    #[test]
    fn can_eval_power() {
        assert_eq!(9.0, setup("3^2"));
        assert_eq!(0.0625, setup("-4^-2"));
        assert_eq!(43046721.0, setup("3^2^4"));
    }

    #[test]
    fn can_eval_decimal() {
        assert_eq!(3.2, setup("3.2"));
        assert_eq!(-3.2, setup("-3.2"));
    }

    #[test]
    fn can_eval_order_of_operations() {
        assert_eq!(14.0, setup("2+4*3"));
        assert_eq!(18.0, setup("(2+4)*3"));

        assert_eq!(-10.0, setup("2-4*3"));
        assert_eq!(-6.0, setup("(2-4)*3"));

        assert_eq!(4.0, setup("2+4/2"));
        assert_eq!(3.0, setup("(2+4)/2"));

        assert_eq!(0.0, setup("2-4/2"));
        assert_eq!(-1.0, setup("(2-4)/2"));

        assert_eq!(55.0, setup("1+2*3^3"));
        assert_eq!(217.0, setup("1+(2*3)^3"));
    }

    #[test]
    fn can_eval_tests_wikipedia() {
        assert_eq!(3.0001220703125, setup("3+4*2/(1-5)^2^3"));
    }

    #[test]
    fn can_eval_functions() {
        assert_eq!(0.5, setup("cos(60)"));
        assert_eq!(0.5, setup("sin(30)"));
        assert_eq!(1.0, setup("tan(45)"));
        assert_eq!(1.0, setup("tan(45)"));
        assert_eq!(4.0, setup("floor(4.5)"));
        assert_eq!(5.0, setup("ceil(4.5)"));
        assert_eq!(5.0, setup("round(4.6)"));
        assert_eq!(1.0, setup("trunc(1.128)"));
        assert_eq!(0.128, setup("fract(1.128)"));
        assert_eq!(2.0, setup("sqrt(4)"));
        assert_eq!(16.0, setup("pow(4, 2)"));
        assert_eq!(2.0, setup("min(4, 2)"));
        assert_eq!(4.0, setup("max(4, 2)"));

        assert_eq!(6.0, setup("max(1, 2) + 4"));
        assert_eq!(8.0, setup("4 + min(5, 4)"));
        assert_eq!(29.0, setup("7 + max(2, min(47.94, trunc(22.54)))"));

        assert_eq!(60.00000000000001, setup("arccos(0.5)"));
        assert_eq!(30.00000000000001, setup("arcsin(0.5)"));
        assert_eq!(45.0, setup("arctan(1)"));

        assert_eq!(4.0, setup("abs(-4)"));
        assert_eq!(3.0, setup("abs(3)"));

        assert_eq!(3.0, setup("avg(1, 2, 3, 4, 5)"))
    }

    #[test]
    fn can_parse_constants() {
        assert_eq!(3.14159265358979, setup("PI"));
        assert_eq!(6.28318530717959, setup("TAU"));
        assert_eq!(1.61803398874990, setup("PHI"));
        assert_eq!(2.71828182845905, setup("E"));
    }
}
