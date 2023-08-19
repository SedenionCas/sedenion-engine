#[cfg(test)]
mod test {
    use crate::numeric_evaluator::evaluate;

    #[test]
    fn can_eval_plus() {
        assert_eq!(7.0, evaluate("2+5").unwrap());
        assert_eq!(-7.0, evaluate("-2+-5").unwrap());
        assert_eq!(14.0, evaluate("2+5+7").unwrap());
    }

    #[test]
    fn can_eval_minus() {
        assert_eq!(-4.0, evaluate("3-7").unwrap());
        assert_eq!(4.0, evaluate("-3--7").unwrap());
        assert_eq!(-8.0, evaluate("3-7-4").unwrap());
    }

    #[test]
    fn can_eval_multiply() {
        assert_eq!(18.0, evaluate("6*3").unwrap());
        assert_eq!(18.0, evaluate("-6*-3").unwrap());
        assert_eq!(144.0, evaluate("6*3*8").unwrap());
    }

    #[test]
    fn can_eval_divide() {
        assert_eq!(0.1, evaluate("1/10").unwrap());
        assert_eq!(0.1, evaluate("-1/-10").unwrap());
        assert_eq!(0.02, evaluate("1/10/5").unwrap());
    }

    #[test]
    fn can_eval_modulus() {
        assert_eq!(1.0, evaluate("3%2").unwrap());
        assert_eq!(1.0, evaluate("-3%-2").unwrap());
        assert_eq!(1.0, evaluate("3%2%3").unwrap());
    }

    #[test]
    fn can_eval_power() {
        assert_eq!(9.0, evaluate("3^2").unwrap());
        assert_eq!(0.0625, evaluate("-4^-2").unwrap());
        assert_eq!(43046721.0, evaluate("3^2^4").unwrap());
    }

    #[test]
    fn can_eval_decimal() {
        assert_eq!(3.2, evaluate("3.2").unwrap());
        assert_eq!(-3.2, evaluate("-3.2").unwrap());
    }

    #[test]
    fn can_eval_order_of_operations() {
        assert_eq!(14.0, evaluate("2+4*3").unwrap());
        assert_eq!(18.0, evaluate("(2+4)*3").unwrap());

        assert_eq!(-10.0, evaluate("2-4*3").unwrap());
        assert_eq!(-6.0, evaluate("(2-4)*3").unwrap());

        assert_eq!(4.0, evaluate("2+4/2").unwrap());
        assert_eq!(3.0, evaluate("(2+4)/2").unwrap());

        assert_eq!(0.0, evaluate("2-4/2").unwrap());
        assert_eq!(-1.0, evaluate("(2-4)/2").unwrap());

        assert_eq!(55.0, evaluate("1+2*3^3").unwrap());
        assert_eq!(217.0, evaluate("1+(2*3)^3").unwrap());
    }

    #[test]
    fn can_eval_tests_wikipedia() {
        assert_eq!(3.0001220703125, evaluate("3+4*2/(1-5)^2^3").unwrap());
    }

    #[test]
    fn can_eval_functions() {
        assert_eq!(0.5, evaluate("cos(60)").unwrap());
        assert_eq!(0.5, evaluate("sin(30)").unwrap());
        assert_eq!(1.0, evaluate("tan(45)").unwrap());
        assert_eq!(1.0, evaluate("tan(45)").unwrap());
        assert_eq!(4.0, evaluate("floor(4.5)").unwrap());
        assert_eq!(5.0, evaluate("ceil(4.5)").unwrap());
        assert_eq!(5.0, evaluate("round(4.6)").unwrap());
        assert_eq!(1.0, evaluate("trunc(1.128)").unwrap());
        assert_eq!(0.128, evaluate("fract(1.128)").unwrap());
        assert_eq!(2.0, evaluate("sqrt(4)").unwrap());
        assert_eq!(16.0, evaluate("pow(4, 2)").unwrap());
        assert_eq!(2.0, evaluate("min(4, 2)").unwrap());
        assert_eq!(4.0, evaluate("max(4, 2)").unwrap());

        assert_eq!(6.0, evaluate("max(1, 2) + 4").unwrap());
        assert_eq!(8.0, evaluate("4 + min(5, 4)").unwrap());
        assert_eq!(
            29.0,
            evaluate("7 + max(2, min(47.94, trunc(22.54)))").unwrap()
        );
    }

    #[test]
    fn can_parse_constants() {
        assert_eq!(3.141592653589793, evaluate("pi").unwrap());
        assert_eq!(6.283185307179586, evaluate("tau").unwrap());
        assert_eq!(1.618033988749895, evaluate("phi").unwrap());
        assert_eq!(2.718281828459045, evaluate("e").unwrap());
    }
}
