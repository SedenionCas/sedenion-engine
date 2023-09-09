#[cfg(test)]
mod test {
    use crate::parser::{parse, parse_equation};

    fn setup_basic(expression: &str) -> String {
        parse(expression).unwrap().to_string()
    }

    fn setup_equation(expression: &str) -> String {
        parse_equation(expression).unwrap().to_string()
    }

    #[test]
    fn can_parse_plus() {
        assert_eq!("(2+5)", setup_basic("2+5"));
        assert_eq!("(-(2)+-(5))", setup_basic("-2+-5"));
        assert_eq!("((2+5)+7)", setup_basic("2+5+7"));
    }

    #[test]
    fn can_parse_minus() {
        assert_eq!("(3-7)", setup_basic("3-7"));
        assert_eq!("(-(3)--(7))", setup_basic("-3--7"));
        assert_eq!("((3-7)-4)", setup_basic("3-7-4"));
    }

    #[test]
    fn can_parse_multiply() {
        assert_eq!("(6*3)", setup_basic("6*3"));
        assert_eq!("(-(6)*-(3))", setup_basic("-6*-3"));
        assert_eq!("((6*3)*8)", setup_basic("6*3*8"));
    }

    #[test]
    fn can_parse_divide() {
        assert_eq!("(1/9)", setup_basic("1/9"));
        assert_eq!("(-(1)/-(9))", setup_basic("-1/-9"));
        assert_eq!("((1/9)/5)", setup_basic("1/9/5"));
    }

    #[test]
    fn can_parse_modulus() {
        assert_eq!("(3%2)", setup_basic("3%2"));
        assert_eq!("(-(3)%-(2))", setup_basic("-3%-2"));
        assert_eq!("((3%2)%3)", setup_basic("3%2%3"));
    }

    #[test]
    fn can_parse_power() {
        assert_eq!("(3^2)", setup_basic("3^2"));
        assert_eq!("(-(3)^-(2))", setup_basic("-3^-2"));
        assert_eq!("(3^(2^4))", setup_basic("3^2^4"));
    }

    #[test]
    fn can_parse_decimal() {
        assert_eq!("3.2", setup_basic("3.2"));
        assert_eq!("-(3.2)", setup_basic("-3.2"));
    }

    #[test]
    fn can_parse_order_of_operations() {
        assert_eq!("(2+(4*3))", setup_basic("2+4*3"));
        assert_eq!("((2+4)*3)", setup_basic("(2+4)*3"));

        assert_eq!("(2-(4*3))", setup_basic("2-4*3"));
        assert_eq!("((2-4)*3)", setup_basic("(2-4)*3"));

        assert_eq!("(2+(4/3))", setup_basic("2+4/3"));
        assert_eq!("((2+4)/3)", setup_basic("(2+4)/3"));

        assert_eq!("(2-(4/3))", setup_basic("2-4/3"));
        assert_eq!("((2-4)/3)", setup_basic("(2-4)/3"));

        assert_eq!("(1+(2*(3^3)))", setup_basic("1+2*3^3"));
        assert_eq!("(1+((2*3)^3))", setup_basic("1+(2*3)^3"));
    }

    #[test]
    fn can_parse_tests_wikipedia() {
        assert_eq!("(3+((4*2)/((1-5)^(2^3))))", setup_basic("3+4*2/(1-5)^2^3"));
        assert_eq!(
            "sin(((max(2, 3)/3)*3.1415))",
            setup_basic("sin(max(2, 3) / 3 * 3.1415)")
        );
    }

    #[test]
    fn can_parse_functions() {
        assert_eq!("(max(1, 2)+4)", setup_basic("max(1, 2) + 4"));
        assert_eq!("(4+min(5, 4))", setup_basic("4 + min(5, 4)"));
        assert_eq!(
            "(7+max(2, min(47.94, trunc(22.54))))",
            setup_basic("7 + max(2, min(47.94, trunc(22.54)))")
        );
    }

    #[test]
    fn can_parse_monomials() {
        assert_eq!("3x^(2)", setup_basic("3x^2"));
        assert_eq!("312a^(221)", setup_basic("312a^221"));
        assert_eq!("1b^(1)", setup_basic("b"));
    }

    #[test]
    fn can_parse_equations() {
        assert_eq!("((1+1)=(4-2))", setup_equation("1+1=4-2"))
    }

    #[test]
    fn can_parse_constants() {
        assert_eq!("PI", setup_basic("PI"));
        assert_eq!("TAU", setup_basic("TAU"));
        assert_eq!("PHI", setup_basic("PHI"));
        assert_eq!("E", setup_basic("E"));
    }
}
