#[derive(Debug)]
pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
    Pow,
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Pow => "^",
        }
        .to_string()
    }
}

impl Op {
    pub fn to_fn(&self) -> fn(f64, f64) -> f64 {
        match self {
            Op::Add => |a, b| a + b,
            Op::Sub => |a, b| a - b,
            Op::Mul => |a, b| a * b,
            Op::Div => |a, b| a / b,
            Op::Pow => |a, b| a.powf(b),
        }
    }
}

// #[derive(Debug)]
// pub enum MonoOp {
//     Abs,
// }

// impl ToString for MonoOp {
//     fn to_string(&self) -> String {
//         match self {
//             MonoOp::Abs => "abs",
//         }
//         .to_string()
//     }
// }

// impl MonoOp {
//     pub fn to_fn(&self) -> fn(f64) -> f64 {
//         match self {
//             MonoOp::Abs => |a| a.abs(),
//         }
//     }
// }
