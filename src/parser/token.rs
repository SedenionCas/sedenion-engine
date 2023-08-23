use std::fmt::format;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Function {
        name: String,
        args: Vec<Box<Expr>>,
    },
    Monomial {
        coefficient: f64,
        variable: String,
        exponent: f64,
    },
    Constant {
        name: String,
        value: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equals,
}

pub trait Optimize {
    fn optimize_expression(self) -> Expr;
    fn optimize_node(&self) -> Expr;
    fn optimize_equation(self) -> Expr;
}

impl Expr {
    pub fn as_latex(&self) -> String {
        match self {
            Expr::Number(num) => num.to_string(),
            Expr::UnaryMinus(inner) => format!("-{}", inner.as_latex()),
            Expr::BinOp { lhs, op, rhs } => match op {
                Op::Power => format!("^{{{}}}", rhs.as_latex()),
                Op::Divide => format!("\\frac{{{}}}{{{}}}", lhs.as_latex(), rhs.as_latex()),
                Op::Multiply => format!("{}\\cdot{}", lhs.as_latex(), rhs.as_latex()),
                _ => format!("{}{}{}", lhs.as_latex(), op.to_string(), rhs.as_latex()),
            },
            Expr::Function { name, args } => {
                let args = args
                    .iter()
                    .map(|arg| arg.as_latex())
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("{}({})", name, args)
            }
            Expr::Monomial {
                coefficient,
                variable,
                exponent,
            } => {
                let mut coefficient = coefficient.to_string();
                if coefficient == "1.0" {
                    coefficient = String::new();
                }

                let mut exponent = format!("^{{{}}}", exponent);
                if exponent == "^{1.0}" {
                    exponent = String::new();
                }

                format!("{}{}{}", coefficient, variable, exponent)
            }
            Expr::Constant { name, .. } => name.to_owned(),
        }
    }

    pub fn print_expr(&self, indent: usize) {
        match self {
            Expr::Number(num) => println!("{:indent$}Number: {}", "", num, indent = indent),
            Expr::UnaryMinus(expr) => {
                println!("{:indent$}UnaryMinus", "", indent = indent);
                expr.print_expr(indent + 2);
            }
            Expr::BinOp { lhs, op, rhs } => {
                println!("{:indent$}BinOp: {:?}", "", op, indent = indent);
                lhs.print_expr(indent + 2);
                rhs.print_expr(indent + 2);
            }
            Expr::Function { name, args } => {
                println!("{:indent$}Function: {}", "", name, indent = indent);
                for arg in args {
                    arg.print_expr(indent + 2);
                }
            }
            Expr::Monomial {
                coefficient,
                variable,
                exponent,
            } => println!(
                "{:indent$}Monomial: {} {}^{}",
                "",
                coefficient,
                variable,
                exponent,
                indent = indent
            ),
            Expr::Constant { name, .. } => {
                println!("{:indent$}Constant: {}", "", name, indent = indent)
            }
        }
    }
}

impl Op {
    pub fn get_precedence(&self) -> Option<u8> {
        match self {
            Op::Add | Op::Subtract => Some(1),
            Op::Multiply | Op::Divide | Op::Modulo => Some(2),
            Op::Power => Some(3),
            Op::Equals => None,
        }
    }
}

impl Expr {
    pub fn get_bin_op(self) -> Option<(Expr, Op, Expr)> {
        match self {
            Expr::BinOp { lhs, op, rhs } => Some((*lhs, op, *rhs)),
            _ => None,
        }
    }
}

impl ToString for Op {
    fn to_string(&self) -> String {
        String::from(match self {
            Op::Add => '+',
            Op::Subtract => '-',
            Op::Multiply => '*',
            Op::Divide => '/',
            Op::Modulo => '%',
            Op::Power => '^',
            Op::Equals => '=',
        })
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        let mut out = String::new();
        match self {
            Expr::Number(val) => out.push_str(&val.to_string()),
            Expr::UnaryMinus(expr) => out.push_str(&format!("-({})", expr.to_string())),
            Expr::BinOp { lhs, op, rhs } => {
                let lhs = lhs.to_string();
                let rhs = rhs.to_string();
                let op = op.to_string();

                out.push_str(&format!("({lhs}{op}{rhs})"));
            }
            Expr::Function { name, args } => {
                let args = args
                    .into_iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                out.push_str(&format!("{name}({args})"));
            }
            Expr::Monomial {
                coefficient,
                variable,
                exponent,
            } => out.push_str(&format!("{coefficient}{variable}^({exponent})")),
            Expr::Constant { name, .. } => out.push_str(&name),
        }
        return out;
    }
}
