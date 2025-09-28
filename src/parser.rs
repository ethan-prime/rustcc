use crate::lexer::Token;
use crate::ast::{self, FunctionDefinition, ProgramNode, StatementNode};
use std::mem;

macro_rules! expect {
    ($p:expr, $pat:pat $(if $guard:expr)? $(,)?) => {{
        let t = $p.curr_tok()?;
        if matches!(t, $pat $(if $guard)?) {
            $p.advance()?; // only advance on success
            Ok(())
        } else {
            Err(format!("expected {}, found {:?}", stringify!($pat), t))
        }
    }};
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn nth_tok(&self, n: usize) -> Result<&Token, String> {
        self.tokens.get(n).ok_or_else(|| "tried to read token out of bounds".to_string())
    }

    pub fn curr_tok(&self) -> Result<&Token, String> {
        self.nth_tok(self.pos)
    }

    pub fn peek_tok(&self) -> Result<&Token, String> {
        self.nth_tok(self.pos + 1)
    }

    fn advance(&mut self) -> Result<(), String> {
        if self.pos < self.tokens.len() { self.pos += 1; Ok(()) }
        else { Err("unexpected end of tokens".into()) }
    }

    fn consume_ident(&mut self) -> Result<String, String> {
        let t = self.curr_tok()?;
        if let Token::Identifier(s) = t {
            let name = s.clone();
            self.advance()?;
            Ok(name)
        } else {
            Err(format!("expected Identifier, found {:?}", t))
        }
    }

    fn parse_return_statement(&mut self) -> Result<StatementNode, String> {
        expect!(self, Token::Return)?;
        let curr_tok = self.curr_tok()?;
        match curr_tok {
            &Token::Integer(i) => {
                self.advance()?;
                Ok(StatementNode::Return(i))
            },
            _ => Err(format!("expected intger, found {:?}", curr_tok)),
        }
    }

    pub fn parse_function_definition(&mut self) -> Result<FunctionDefinition, String> {
        // int <ident>(void) { ... }
        expect!(self, Token::Int)?;                    // 'int'
        let name = self.consume_ident()?;              // function name
        expect!(self, Token::OpenParen)?;              // '('
        expect!(self, Token::Void)?;                   // 'void'
        expect!(self, Token::CloseParen)?;             // ')'
        expect!(self, Token::OpenBrace)?;              // '{'

        // ... parse body items until '}' ...
        
        let return_statement = self.parse_return_statement()?;

        expect!(self, Token::Semicolon)?;
        expect!(self, Token::CloseBrace)?;             // '}'
        Ok(FunctionDefinition::new(name, return_statement))
    }
}
