use crate::{ast::Ast, op::Op, token::Token};

pub fn parse(tokens: &[Token]) -> Ast {
    let root = find_last_op(tokens, |t| match t {
        Token::Add | Token::Sub => true,
        _ => false,
    })
    .or(find_last_op(tokens, |t| match t {
        Token::Mul | Token::Div => true,
        _ => false,
    }))
    .or(find_last_op(tokens, |t| match t {
        Token::Pow => true,
        _ => false,
    }));

    if let Some((index, tok)) = root {
        return Ast::BiNode {
            op: match tok {
                Token::Add => Op::Add,
                Token::Sub => Op::Sub,
                Token::Mul => Op::Mul,
                Token::Div => Op::Div,
                Token::Pow => Op::Pow,
                _ => unreachable!(),
            },
            left: Box::new(parse(&tokens[..index])),
            right: Box::new(parse(&tokens[(index + 1)..])),
        };
    }

    let end = tokens.len() - 1;

    if tokens[0] == Token::OpenParen && tokens[end] == Token::CloseParen {
        return parse(&tokens[1..end]);
    }

    if tokens.len() == 1 {
        if let Token::Num(x) = tokens[0] {
            return Ast::Leaf(x);
        } else {
            unreachable!();
        }
    }

    unreachable!()
}

fn find_last_op(
    tokens: &[Token],
    mut predicate: impl FnMut(Token) -> bool,
) -> Option<(usize, &Token)> {
    let mut counter = 0;

    tokens.iter().enumerate().rev().find(|(_, &t)| {
        if t == Token::CloseParen {
            counter += 1
        };
        if t == Token::OpenParen {
            counter -= 1
        };
        predicate(t) && counter == 0
    })
}
