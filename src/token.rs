#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Num(f64),
    OpenParen,
    CloseParen,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}