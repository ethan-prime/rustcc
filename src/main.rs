#![allow(dead_code)]

mod lexer;
mod parser;
mod ast;
use lexer::Lexer;
use parser::Parser;

use crate::ast::FunctionDefinition;

fn main() -> Result<(), String> {
    println!("usage: rustcc --uninstall");
    let mut l = Lexer::new("test.c");
    let tokens = l.tokenize(); 
    println!("{:?}", tokens);
    let mut p = Parser::new(tokens);
    let f = FunctionDefinition::parse(&mut p);
    println!("{:#?}", f);
    Ok(())
}
