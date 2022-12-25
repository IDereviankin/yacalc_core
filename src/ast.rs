use crate::op::Op;

#[derive(Debug)]
pub enum Ast {
    BiNode {
        op: Op,
        left: Box<Ast>,
        right: Box<Ast>,
    },
    // MonoNode {
    //     op: MonoOp,
    //     expr: Box<Ast>,
    // },
    Leaf(f64),
}

impl Ast {
    pub fn eval(&self) -> f64 {
        match self {
            Ast::BiNode { op, left, right } => op.to_fn()(left.eval(), right.eval()),
            // Ast::MonoNode { op, expr } => op.to_fn()(expr.eval()),
            Ast::Leaf(x) => *x,
        }
    }
}

impl ToString for Ast {
    fn to_string(&self) -> String {
        match self {
            Ast::BiNode { op, left, right } => format!(
                "{} {} {}",
                left.to_string(),
                right.to_string(),
                op.to_string()
            ),
            // Ast::MonoNode { op, expr } => format!("{}({})", op.to_string(), expr.to_string()),
            Ast::Leaf(x) => x.to_string(),
        }
    }
}