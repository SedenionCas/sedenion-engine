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

impl ToString for Expr {
    fn to_string(&self) -> String {
        let mut out = String::new();
        match self {
            Expr::Number(val) => out.push_str(&val.to_string()),
            Expr::UnaryMinus(expr) => out.push_str(&format!("-({})", expr.to_string())),
            Expr::BinOp { lhs, op, rhs } => {
                let lhs = lhs.to_string();
                let rhs = rhs.to_string();
                let op = match op {
                    Op::Add => '+',
                    Op::Subtract => '-',
                    Op::Multiply => '*',
                    Op::Divide => '/',
                    Op::Modulo => '%',
                    Op::Power => '^',
                    Op::Equals => '=',
                };

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
