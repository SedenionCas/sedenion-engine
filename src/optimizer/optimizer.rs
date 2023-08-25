use anyhow::{bail, Result};
use log::trace;

use crate::{
    error::EvaluatorError,
    parser::{Expr, Op},
};

fn distribute_monomials(lhs: Expr, op: Op, rhs: Expr, target: String) -> Expr {
    match (lhs.clone(), rhs.clone()) {
        (
            Expr::Monomial {
                coefficient: left_coefficient,
                variable: left_variable,
                exponent: left_exponent,
            },
            Expr::Monomial {
                coefficient: right_coefficient,
                variable: right_variable,
                exponent: right_exponent,
            },
        ) => {
            if left_variable == right_variable {
                let new_exponent = match op {
                    Op::Add | Op::Subtract if left_exponent == right_exponent => left_exponent,
                    Op::Multiply => left_exponent + right_exponent,
                    _ => {
                        return Expr::BinOp {
                            lhs: Box::new(lhs),
                            op,
                            rhs: Box::new(rhs),
                        }
                    }
                };

                let new_coefficient = match op {
                    Op::Add => left_coefficient + right_coefficient,
                    Op::Subtract => left_coefficient - right_coefficient,
                    Op::Multiply => left_coefficient * right_coefficient,
                    _ => left_coefficient,
                };

                trace!("Merging monomials: op:={op:?} from={left_coefficient}{left_variable}{left_exponent}, {right_coefficient}{right_variable}{right_exponent} new={new_coefficient}{left_variable}^{new_exponent}");

                return Expr::Monomial {
                    coefficient: new_coefficient,
                    variable: left_variable,
                    exponent: new_exponent,
                };
            }
        }
        (
            Expr::BinOp {
                lhs: left_lhs,
                op: left_op,
                rhs: left_rhs,
            },
            Expr::Monomial { .. },
        ) => {
            if left_op.get_precedence() == op.get_precedence() {
                // I am so sorry for the variable names...
                if let Expr::Monomial {
                    variable: right_left_variable,
                    ..
                } = *left_rhs.clone()
                {
                    if right_left_variable == target {
                        trace!("Hoisting right");

                        return Expr::BinOp {
                            lhs: Box::new(Expr::BinOp {
                                lhs: left_lhs,
                                op,
                                rhs: Box::new(rhs),
                            }),
                            op: left_op,
                            rhs: left_rhs,
                        };
                    }
                }

                if let Expr::Monomial {
                    variable: left_left_variable,
                    ..
                } = *left_lhs.clone()
                {
                    if left_left_variable == target {
                        trace!("Hoisting left");

                        let mut rhs = rhs;
                        if op == Op::Subtract {
                            rhs = Expr::UnaryMinus(Box::new(rhs));
                        }

                        return Expr::BinOp {
                            lhs: Box::new(Expr::BinOp {
                                lhs: Box::new(rhs),
                                op: left_op,
                                rhs: left_rhs,
                            }),
                            op,
                            rhs: left_lhs,
                        };
                    }
                }
            }
        }
        _ => {}
    }

    Expr::BinOp {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
    }
}

impl Expr {
    pub fn optimize_expression(self, target: String) -> Expr {
        let mut old = self.clone();
        let mut latest = self.optimize_node(target.clone()).merge_numbers().unwrap();

        while old != latest {
            trace!("New cycle started...");
            old = latest.clone();
            latest = latest.optimize_node(target.clone());
        }

        latest
    }

    pub fn merge_numbers(&self) -> Result<Expr> {
        match self {
            Expr::BinOp { lhs, op, rhs } => {
                let merged_lhs = lhs.merge_numbers()?;
                let merged_rhs = rhs.merge_numbers()?;

                if let (Expr::Number(n_lhs), Expr::Number(n_rhs)) =
                    (merged_lhs.clone(), merged_rhs.clone())
                {
                    let res = match *op {
                        Op::Add => n_lhs + n_rhs,
                        Op::Subtract => n_lhs - n_rhs,
                        Op::Multiply => n_lhs * n_rhs,
                        Op::Divide => n_lhs / n_rhs,
                        Op::Modulo => n_lhs % n_rhs,
                        Op::Power => n_lhs.powf(n_rhs),
                        Op::Equals => bail!(EvaluatorError::EqualityInEval),
                    };

                    if res.fract() == 0.0 {
                        return Ok(Expr::Number(res));
                    } else {
                        return Ok(Expr::BinOp {
                            lhs: Box::new(merged_lhs),
                            op: *op,
                            rhs: Box::new(merged_rhs),
                        });
                    }
                }

                Ok(Expr::BinOp {
                    lhs: Box::new(merged_lhs),
                    op: *op,
                    rhs: Box::new(merged_rhs),
                })
            }
            Expr::Function { name, args } => todo!(),
            _ => Ok(self.clone()),
        }
    }

    pub fn optimize_node(&self, target: String) -> Expr {
        match self.clone() {
            Expr::BinOp { lhs, op, rhs } => {
                let optimized_lhs = lhs.optimize_node(target.clone());
                let optimized_rhs = rhs.optimize_node(target.clone());

                match op {
                    Op::Add => {
                        // ======== constants ========
                        // 0 + a = a
                        if let Expr::Number(n) = optimized_lhs {
                            if n == 0.0 {
                                trace!("0+a=a");
                                return optimized_rhs;
                            }
                        }

                        // a + 0 = a
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 0.0 {
                                trace!("a+0=a");
                                return optimized_lhs;
                            }
                        }

                        // ======== merges ========
                        // a + a = 2a
                        if optimized_lhs == optimized_rhs {
                            let (Expr::Monomial { .. }, Expr::Monomial { .. }) = (&optimized_lhs, &optimized_rhs) else {
                                trace!("a+a=a");
                                return Expr::BinOp {
                                    lhs: Box::new(Expr::Number(2.0)),
                                    op: Op::Multiply,
                                    rhs: Box::new(optimized_lhs),
                                };
                            };
                        }

                        // a + (-b) = a - b
                        if let Expr::UnaryMinus(inner) = optimized_rhs.clone() {
                            trace!("a+(-b)=a-b");
                            return Expr::BinOp {
                                lhs: Box::new(optimized_lhs),
                                op: Op::Subtract,
                                rhs: inner,
                            };
                        }
                    }
                    Op::Subtract => {
                        // ======== constants ========
                        // a - 0 = a
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 0.0 {
                                trace!("a-0 = a");
                                return optimized_lhs;
                            }
                        }

                        // 0 - a = -a
                        if let Expr::Number(n) = optimized_lhs {
                            if n == 0.0 {
                                trace!("0-a = -a");
                                return Expr::UnaryMinus(Box::new(optimized_rhs));
                            }
                        }

                        // ======== merges ========
                        // a - a = 0
                        if optimized_lhs == optimized_rhs {
                            trace!("a-a = 0");
                            return Expr::Number(0.0);
                        }

                        // a - (-b) = a + b
                        if let Expr::UnaryMinus(inner) = optimized_rhs.clone() {
                            trace!("a-(-b)=a+b");
                            return Expr::BinOp {
                                lhs: Box::new(optimized_lhs),
                                op: Op::Add,
                                rhs: inner,
                            };
                        }
                    }
                    Op::Multiply => {
                        // ======== constants ========
                        // 1 * a = a
                        if let Expr::Number(n) = optimized_lhs {
                            if n == 1.0 {
                                trace!("1*a=a");
                                return optimized_rhs;
                            }
                        }

                        // a * 1 = a
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 1.0 {
                                trace!("a*1=a");
                                return optimized_lhs;
                            }
                        }

                        // 0 * a = 0
                        if let Expr::Number(n) = optimized_lhs {
                            if n == 0.0 {
                                trace!("0*a=0");
                                return Expr::Number(0.0);
                            }
                        }

                        // a * 0 = 0
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 0.0 {
                                trace!("a*0=0");
                                return Expr::Number(0.0);
                            }
                        }

                        // ======== merges ========
                        // a * a = a^2
                        if optimized_lhs == optimized_rhs {
                            let (Expr::Monomial { .. }, Expr::Monomial { .. }) =
                                (&optimized_lhs, &optimized_rhs)
                            else {
                                trace!("a*a=a^2");
                                return Expr::BinOp {
                                    lhs: Box::new(optimized_lhs),
                                    op: Op::Power,
                                    rhs: Box::new(Expr::Number(2.0)),
                                };
                            };
                        }

                        // a^b * a^c = a^(b+c)
                        if let (
                            Expr::BinOp {
                                lhs: left_lhs,
                                op: left_op,
                                rhs: left_rhs,
                            },
                            Expr::BinOp {
                                lhs: right_lhs,
                                op: right_op,
                                rhs: right_rhs,
                            },
                        ) = (optimized_lhs.clone(), optimized_rhs.clone())
                        {
                            if left_lhs == right_lhs
                                && left_op == Op::Power
                                && right_op == Op::Power
                            {
                                trace!("a^b*a^c=a^(b+c)");
                                return Expr::BinOp {
                                    lhs: left_lhs,
                                    op: Op::Power,
                                    rhs: Box::new(Expr::BinOp {
                                        lhs: left_rhs,
                                        op: Op::Add,
                                        rhs: right_rhs,
                                    }),
                                };
                            }
                        }

                        // a * (b <op> c) = a * b <op> a * c
                        if let (
                            Expr::Number(..),
                            Expr::BinOp {
                                lhs: right_lhs,
                                op: right_op,
                                rhs: right_rhs,
                            },
                        ) = (optimized_lhs.clone(), optimized_rhs.clone())
                        {
                            trace!("a*(b <op> c) = a*b <op> a*c");
                            return Expr::BinOp {
                                lhs: Box::new(Expr::BinOp {
                                    lhs: Box::new(optimized_lhs.clone()),
                                    op: Op::Multiply,
                                    rhs: right_lhs,
                                }),
                                op: right_op,
                                rhs: Box::new(Expr::BinOp {
                                    lhs: Box::new(optimized_lhs),
                                    op: Op::Multiply,
                                    rhs: right_rhs,
                                }),
                            };
                        }

                        // ======== monomials ========
                        // a * bX^c = a*bX^c
                        if let (
                            Expr::Number(num),
                            Expr::Monomial {
                                coefficient,
                                variable,
                                exponent,
                            },
                        ) = (optimized_lhs.clone(), optimized_rhs.clone())
                        {
                            trace!("a*bX^c = (a*b)X^c");
                            return Expr::Monomial {
                                coefficient: num * coefficient,
                                variable,
                                exponent,
                            };
                        }
                    }
                    Op::Divide => {
                        // ======== constants ========
                        // a / 1 = a
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 1.0 {
                                trace!("a/1=a");
                                return optimized_lhs;
                            }
                        }

                        // ======== reduces ========
                        if let (
                            Expr::Monomial {
                                coefficient,
                                variable,
                                exponent,
                            },
                            Expr::Number(num),
                        ) = (optimized_lhs.clone(), optimized_rhs.clone())
                        {
                            return Expr::Monomial {
                                coefficient: coefficient / num,
                                variable,
                                exponent,
                            };
                        }

                        // ======== merges ========
                        // a / a = 1
                        if optimized_lhs == optimized_rhs {
                            trace!("a/a=1");
                            return Expr::Number(1.0);
                        }
                    }
                    Op::Power => {
                        // ======== constants ========
                        // a^0 = 1
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 0.0 {
                                trace!("a^0=1");
                                return Expr::Number(1.0);
                            }
                        }

                        // a^1 = a
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 1.0 {
                                trace!("a^1=a");
                                return optimized_lhs;
                            }
                        }

                        // a^(-n) = 1/a^n
                        if let Expr::UnaryMinus(inner) = optimized_rhs.clone() {
                            if let Expr::Number(n) = *inner {
                                trace!("a^(-n) = 1/a^n");
                                return Expr::BinOp {
                                    lhs: Box::new(Expr::Number(1.0)),
                                    op: Op::Divide,
                                    rhs: Box::new(Expr::BinOp {
                                        lhs: Box::new(optimized_lhs),
                                        op: Op::Power,
                                        rhs: Box::new(Expr::Number(n)),
                                    }),
                                };
                            }
                        }
                    }
                    operator => todo!("{operator:?}"),
                }
                distribute_monomials(optimized_lhs, op, optimized_rhs, target)
            }
            Expr::Monomial {
                coefficient,
                variable,
                exponent,
            } => {
                if coefficient == 0.0 {
                    trace!("Collapsing monomial due to coefficient");
                    return Expr::Number(0.0);
                }

                if exponent == 0.0 {
                    trace!("Collapsing monomial due to exponent");
                    return Expr::Number(1.0);
                }

                if coefficient.is_sign_negative() {
                    trace!("Applying unary to coefficient");
                    return Expr::UnaryMinus(Box::new(Expr::Monomial {
                        coefficient: coefficient.abs(),
                        variable,
                        exponent,
                    }));
                }

                self.clone()
            }
            Expr::UnaryMinus(inner) => {
                let inner = inner.optimize_node(target);

                // --a = a
                if let Expr::UnaryMinus(inner_inner) = inner {
                    trace!("--a = a");
                    return *inner_inner;
                }

                // -(a <op> b) = -a <op> -b
                if let Expr::BinOp { lhs, op, rhs } = inner {
                    trace!("-(a<op>b) = -a<op>-b");
                    return Expr::BinOp {
                        lhs: Box::new(Expr::UnaryMinus(lhs)),
                        op,
                        rhs: Box::new(Expr::UnaryMinus(rhs)),
                    };
                }

                Expr::UnaryMinus(Box::new(inner))
            }
            Expr::Number(_) => self.clone(),
            node => todo!("optimization: {node:?}"),
        }
    }

    fn apply_equation_rule(self, target: String) -> Expr {
        if let Expr::BinOp {
            lhs,
            op: Op::Equals,
            rhs,
        } = self.clone()
        {
            let lhs = *lhs;
            let rhs = *rhs;

            // ======== addition ========

            // T + a = b => T = b - a
            if let Expr::BinOp {
                lhs: left_lhs,
                op: Op::Add,
                rhs: left_rhs,
            } = lhs.clone()
            {
                if let Expr::Monomial { variable, .. } = *left_lhs.clone() {
                    if variable == target {
                        trace!("T+a = b => T = b-a");
                        return Expr::BinOp {
                            lhs: left_lhs,
                            op: Op::Equals,
                            rhs: Box::new(Expr::BinOp {
                                lhs: Box::new(rhs),
                                op: Op::Subtract,
                                rhs: left_rhs,
                            }),
                        };
                    }
                }
            }

            // a + T = b => T = b - a
            if let Expr::BinOp {
                lhs: left_lhs,
                op: Op::Add,
                rhs: left_rhs,
            } = lhs.clone()
            {
                if let Expr::Monomial { variable, .. } = *left_rhs.clone() {
                    if variable == target {
                        trace!("a+T = b => T = b-a");
                        return Expr::BinOp {
                            lhs: left_rhs,
                            op: Op::Equals,
                            rhs: Box::new(Expr::BinOp {
                                lhs: Box::new(rhs),
                                op: Op::Subtract,
                                rhs: left_lhs,
                            }),
                        };
                    }
                }
            }

            if let Expr::BinOp {
                lhs: right_lhs,
                op: Op::Add,
                rhs: right_rhs,
            } = rhs.clone()
            {
                debug!("{}", self.to_string());
                if let Expr::Monomial { variable, .. } = *right_lhs.clone() {
                    if variable == target {
                        trace!("a = T+b => a-T = b");
                        return Expr::BinOp {
                            lhs: Box::new(Expr::BinOp {
                                lhs: Box::new(lhs),
                                op: Op::Subtract,
                                rhs: right_rhs,
                            }),
                            op: Op::Equals,
                            rhs: right_lhs,
                        };
                    }
                }
            }

            // a = b + T => a - T = b
            if let Expr::BinOp {
                lhs: right_lhs,
                op: Op::Add,
                rhs: right_rhs,
            } = rhs.clone()
            {
                if let Expr::Monomial { variable, .. } = *right_rhs.clone() {
                    if variable == target {
                        trace!("a = b+T => a-T = b");
                        return Expr::BinOp {
                            lhs: Box::new(Expr::BinOp {
                                lhs: Box::new(lhs),
                                op: Op::Subtract,
                                rhs: right_rhs,
                            }),
                            op: Op::Equals,
                            rhs: right_lhs,
                        };
                    }
                }
            }

            // ======== subtraction ========
            // T - a = b => T = b + a
            if let Expr::BinOp {
                lhs: left_lhs,
                op: Op::Subtract,
                rhs: left_rhs,
            } = lhs.clone()
            {
                if let Expr::Monomial { variable, .. } = *left_lhs.clone() {
                    if variable == target {
                        trace!("T-a = b => T = b+a");
                        return Expr::BinOp {
                            lhs: left_lhs,
                            op: Op::Equals,
                            rhs: Box::new(Expr::BinOp {
                                lhs: Box::new(rhs),
                                op: Op::Add,
                                rhs: left_rhs,
                            }),
                        };
                    }
                }
            }

            // a - T = b => T = -b + a
            if let Expr::BinOp {
                lhs: left_lhs,
                op: Op::Subtract,
                rhs: left_rhs,
            } = lhs.clone()
            {
                if let Expr::Monomial { variable, .. } = *left_rhs.clone() {
                    if variable == target {
                        trace!("a-T = b => T = -b+a");
                        return Expr::BinOp {
                            lhs: left_rhs,
                            op: Op::Equals,
                            rhs: Box::new(Expr::BinOp {
                                lhs: Box::new(Expr::UnaryMinus(Box::new(rhs))),
                                op: Op::Add,
                                rhs: left_lhs,
                            }),
                        };
                    }
                }
            }

            // a = T - b => a - T = - b
            if let Expr::BinOp {
                lhs: right_lhs,
                op: Op::Subtract,
                rhs: right_rhs,
            } = rhs.clone()
            {
                if let Expr::Monomial { variable, .. } = *right_lhs.clone() {
                    if variable == target {
                        trace!("a = T-b => a-T = -b");
                        return Expr::BinOp {
                            lhs: Box::new(Expr::BinOp {
                                lhs: Box::new(lhs),
                                op: Op::Subtract,
                                rhs: right_lhs,
                            }),
                            op: Op::Equals,
                            rhs: Box::new(Expr::UnaryMinus(right_rhs)),
                        };
                    }
                }
            }

            // a = b - T => a + T = b
            if let Expr::BinOp {
                lhs: right_lhs,
                op: Op::Subtract,
                rhs: right_rhs,
            } = rhs.clone()
            {
                if let Expr::Monomial { variable, .. } = *right_rhs.clone() {
                    if variable == target {
                        trace!("a = b-T => T = -a+b");
                        return Expr::BinOp {
                            lhs: Box::new(Expr::BinOp {
                                lhs: Box::new(lhs),
                                op: Op::Add,
                                rhs: right_rhs,
                            }),
                            op: Op::Equals,
                            rhs: right_lhs,
                        };
                    }
                }
            }

            // ======== unary ========

            // -(T) = a => T = -(a)
            if let Expr::UnaryMinus(inner) = lhs.clone() {
                if let Expr::Monomial { variable, .. } = *inner.clone() {
                    if variable == target {
                        trace!("-(T) = a => T = -(a)");
                        return Expr::BinOp {
                            lhs: inner,
                            op: Op::Equals,
                            rhs: Box::new(Expr::UnaryMinus(Box::new(rhs))),
                        };
                    }
                }
            }

            // ======== monomial ========
            // reduce coefficient
            if let Expr::Monomial {
                coefficient,
                variable,
                exponent,
            } = lhs.clone()
            {
                trace!("Reducing coefficient");
                if variable == target {
                    return Expr::BinOp {
                        lhs: Box::new(Expr::Monomial {
                            coefficient: 1.0,
                            variable,
                            exponent,
                        }),
                        op: Op::Equals,
                        rhs: Box::new(Expr::BinOp {
                            lhs: Box::new(rhs),
                            op: Op::Divide,
                            rhs: Box::new(Expr::Number(coefficient)),
                        }),
                    };
                }
            }

            Expr::BinOp {
                lhs: Box::new(lhs),
                op: Op::Equals,
                rhs: Box::new(rhs),
            }
        } else {
            panic!("Not an equation!");
        }
    }

    pub fn optimize_equation(self, target: String) -> Expr {
        if let Expr::BinOp {
            lhs,
            op: Op::Equals,
            rhs,
        } = self
        {
            let mut old = Expr::BinOp {
                lhs: Box::new(lhs.optimize_expression(target.clone()).merge_numbers().unwrap()),
                op: Op::Equals,
                rhs: Box::new(rhs.optimize_expression(target.clone()).merge_numbers().unwrap()),
            };

            loop {
                let expression = old.clone().apply_equation_rule(target.clone());
                let (lhs, op, rhs) = expression.get_bin_op().unwrap();
                let expression = Expr::BinOp {
                    lhs: Box::new(lhs.optimize_expression(target.clone())),
                    op: Op::Equals,
                    rhs: Box::new(rhs.optimize_expression(target.clone())),
                };

                if expression == old {
                    return expression;
                } else {
                    old = expression;
                }
            }
        } else {
            panic!("Not an equation!");
        }
    }
}
