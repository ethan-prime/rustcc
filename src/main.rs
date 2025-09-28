mod lexer;
mod parser;
mod ast;
use lexer::Lexer;
use parser::Parser;

fn main() -> Result<(), String> {
    println!("usage: rustcc --uninstall");
    let mut l = Lexer::new("test.c");
    let tokens = l.tokenize(); 
    println!("{:?}", tokens);
    let mut p = Parser::new(tokens);
    let _ = p.parse_function_definition()?;
    Ok(())
}
