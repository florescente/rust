mod lexer;

pub enum Token {
    LPar,
    Rpar,
    Identifier(String),
    Number(u64),
}

pub enum Expr {
    Number(u64),
    Identifier(String),
    Call(String, Vec<Expr>),
    Let(String, Box<Expr>)),
    Print(String, Box<Expr>),
}

fn main() {
    let str ="(let x 5)".to_string();

    for x in str.chars()
}
