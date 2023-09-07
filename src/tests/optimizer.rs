#[cfg(test)]
mod test {
    use crate::parser::{parse, parse_equation};
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup_single(expression: &str) -> String {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        parse(expression)
            .unwrap()
            .optimize_node(String::new())
            .to_string()
    }

    fn setup_merge(expression: &str) -> String {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        parse(expression)
            .unwrap()
            .merge_numbers()
            .unwrap()
            .to_string()
    }

    fn setup_multi(expression: &str) -> String {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        parse(expression)
            .unwrap()
            .optimize_expression(String::new())
            .to_string()
    }

    fn setup_equation(expression: &str, target: &str) -> String {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        parse_equation(expression)
            .unwrap()
            .optimize_equation(target.to_string())
            .to_string()
    }

    #[test]
    fn can_optimize_double_unary() {
        assert_eq!("25", setup_single("-(-25)"));
    }

    #[test]
    fn can_optimize_double_unary_in_expression() {
        assert_eq!("((3*5)+25)", setup_single("3*5+(-(-25))"));
    }

    #[test]
    fn can_optimize_zero_addition() {
        assert_eq!("645", setup_single("0+645"));
        assert_eq!("645", setup_single("645+0"));
    }

    #[test]
    fn can_optimize_zero_addition_in_expression() {
        assert_eq!("(55*645)", setup_single("55*(0+645)"));
        assert_eq!("(24*645)", setup_single("24*645+0"));
    }

    #[test]
    fn can_optimize_zero_subtraction() {
        assert_eq!("645", setup_single("645-0"));
    }

    #[test]
    fn can_optimize_zero_subtraction_in_expression() {
        assert_eq!("(24*645)", setup_single("24*645-0"));
    }

    #[test]
    fn can_optimize_double_subtraction() {
        assert_eq!("0", setup_single("112-112"));
    }

    #[test]
    fn can_optimize_double_subtraction_in_expression() {
        assert_eq!("0", setup_single("(32894/132)-(32894/132)"));
    }

    #[test]
    fn can_optimize_one_multiplication() {
        assert_eq!("645", setup_single("1*645"));
        assert_eq!("645", setup_single("645*1"));
    }

    #[test]
    fn can_optimize_one_multiplication_in_expression() {
        assert_eq!("(55*645)", setup_single("55*1*645"));
        assert_eq!("(24*645)", setup_single("24*645*1"));
    }

    #[test]
    fn can_optimize_one_division() {
        assert_eq!("645", setup_single("645/1"));
    }

    #[test]
    fn can_optimize_one_division_in_expression() {
        assert_eq!("(24*645)", setup_single("24*645/1"));
    }

    #[test]
    fn can_optimize_double_division() {
        assert_eq!("1", setup_single("112/112"));
    }

    #[test]
    fn can_optimize_double_division_in_expression() {
        assert_eq!("1", setup_single("(32894-132)/(32894-132)"));
    }

    #[test]
    fn can_optimize_double_powers() {
        assert_eq!("(3^(5+10))", setup_single("3^5*3^10"));
    }

    #[test]
    fn can_optimize_double_powers_in_expression() {
        assert_eq!(
            "(3^((3213*2)+(421*23)))",
            setup_single("3^(3213*2)*3^(421*23)")
        );
    }

    #[test]
    fn can_optimize_power_of_one() {
        assert_eq!("3", setup_single("3^1"));
    }

    #[test]
    fn can_optimize_power_of_one_in_expression() {
        assert_eq!("(3213*2)", setup_single("(3213*2)^1"));
    }

    #[test]
    fn can_optimize_power_of_negative_one() {
        assert_eq!("(1/(3^1))", setup_single("3^(-1)"));
    }

    #[test]
    fn can_optimize_power_of_negative_one_in_expression() {
        assert_eq!("(1/((3213*2)^1))", setup_single("(3213*2)^(-1)"));
    }

    #[test]
    fn can_optimize_multiple_layers() {
        assert_eq!("(1/6426)", setup_multi("(3213*2)^(-1)"));
        assert_eq!("(1/0)", setup_multi("(53*88*(52-52))^(-(125/125))"));
    }

    #[test]
    fn can_optimize_monomial_plus() {
        assert_eq!("8x^(8)", setup_single("2x^8+6x^8"));
        assert_eq!("2x^(1)", setup_single("x+x"));
    }

    #[test]
    fn can_optimize_monomial_multiply() {
        assert_eq!("12x^(10)", setup_single("2x^8*6x^2"));
        assert_eq!("1x^(2)", setup_single("x*x"));
    }

    #[test]
    fn can_optimize_equation_per_side() {
        assert_eq!("(1x^(2)=2x^(1))", setup_equation("x*x=x+x", "x"));
    }

    #[test]
    fn can_optimize_equation_cross_equal_sign() {
        assert_eq!("(1y^(1)=0)", setup_equation("x+y=x", "y"));
        assert_eq!("(1y^(1)=-(1x^(1)))", setup_equation("x+y+x=x", "y"));
    }

    #[test]
    fn can_optimize_cross_equal_sign_negative() {
        assert_eq!("(1y^(1)=-(1x^(1)))", setup_equation("x-y-x=x", "y"));
    }

    #[test]
    fn can_optimize_hoisting() {
        assert_eq!("(1y^(1)=1x^(1))", setup_equation("y-x+x=x", "y"));
        assert_eq!("(1y^(1)=4x^(1))", setup_equation("-(3x)-4y=5x-6y", "y"));
    }

    #[test]
    fn can_optimize_mixed() {
        assert_eq!("(4x^(3)+6)", setup_multi("2*(2x^3+3)"))
    }

    #[test]
    fn can_merge_simple() {
        assert_eq!("(1*1x^(1))", setup_merge("(1+1)/2*x"));
    }

    #[test]
    fn dont_merge_with_fraction() {
        assert_eq!("((3/2)*1x^(1))", setup_merge("(1.5+1.5)/2*x"));
    }
}
