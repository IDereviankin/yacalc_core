use std::iter::Peekable;

use crate::token::Token;

pub fn lex(text: &str) -> Vec<Token> {
    let mut i: Peekable<_> = text.chars().filter(|x| !(x.is_whitespace())).peekable();
    let mut output = vec![];

    loop {
        let mut a = lex_until_number(&mut i);
        let b = lex_number(&mut i);
        let is_empty = a.is_empty();

        output.append(&mut a);
        match b {
            Some(y) => output.push(y),
            None if is_empty => break,
            _ => {}
        }
    }

    output
}

fn lex_until_number<T>(i: &mut Peekable<T>) -> Vec<Token>
where
    T: Iterator<Item = char>,
{
    let mut output = vec![];

    while let Some(c) = i.peek().map(lex_char).unwrap_or(None) {
        output.push(c);
        i.next();
    }

    output
}

fn lex_char(c: &char) -> Option<Token> {
    match c {
        '(' => Some(Token::OpenParen),
        ')' => Some(Token::CloseParen),
        '+' => Some(Token::Add),
        '-' => Some(Token::Sub),
        '*' => Some(Token::Mul),
        '/' => Some(Token::Div),
        '^' => Some(Token::Pow),
        _ => None,
    }
}

fn lex_number<T>(i: &mut Peekable<T>) -> Option<Token>
where
    T: Iterator<Item = char>,
{
    let mut s = vec![];

    while let Some(x) = i.peek() {
        if belongs_to_number(x) {
            s.push(*x);
        } else {
            break;
        }

        i.next();
    }

    if let Ok(x) = s.iter().collect::<String>().parse::<f64>() {
        Some(Token::Num(x))
    } else {
        None
    }
}

#[inline]
fn belongs_to_number(x: &char) -> bool {
    *x == '.' || ('0'..='9').contains(x)
}