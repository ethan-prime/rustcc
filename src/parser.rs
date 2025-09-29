use crate::lexer::Token;
use crate::ast::{BinaryOperator, ExprNode, FactorNode, FunctionDefinition, StatementNode, UnaryOperator};

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

    fn parse_expr(&mut self, min_prec: i32) -> Result<Box<ExprNode>, String> {
        let mut left = match self.parse_factor()? {
            FactorNode::Expr(expr) => expr,
            FactorNode::Unary{unary_op:op,expr:e} => Box::new(ExprNode::Unary { unary_op: op, expr: e }),
            FactorNode::Integer(i) => Box::new(ExprNode::Integer(i)),
        };

        while let Ok(tok) = self.curr_tok() {
            if let Some(binary_op) = tok.is_binary_operator() && binary_op.precedence() >= min_prec {
                self.advance()?;
                let right = self.parse_expr(binary_op.precedence() + 1)?;
                left = Box::new(ExprNode::Binary{ lhs: left, rhs: right, binary_op: binary_op });
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<FactorNode, String> {
        let t = self.curr_tok()?;
        match t {
            &Token::Integer(i) => {
                self.advance()?;
                Ok(FactorNode::Integer(i))
            },
            &Token::Hyphen | &Token::Tilde => {
                let unary_op = match t {&Token::Hyphen => UnaryOperator::Negate, _ => UnaryOperator::Complement};
                self.advance()?;
                let expr: Box<ExprNode> = self.parse_expr(0)?;
                Ok(FactorNode::Unary { unary_op, expr })
            },
            &Token::OpenParen => {
                self.advance()?;
                let expr = self.parse_expr(0)?;
                expect!(self, Token::CloseParen)?;
                Ok(FactorNode::Expr(expr))
            },
            _ => {
                Err(format!("expeceted an expression, found {:?}", t))
            },
        }
    }
 
    fn parse_return_statement(&mut self) -> Result<StatementNode, String> {
        expect!(self, Token::Return)?;
        Ok(StatementNode::Return(*self.parse_expr(0)?))
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

impl Token {
    pub fn is_binary_operator(&self) -> Option<BinaryOperator> {
        match self {
            &Token::Plus => Some(BinaryOperator::Add),
            &Token::Hyphen => Some(BinaryOperator::Subtract),
            &Token::Asterisk => Some(BinaryOperator::Multiply),
            &Token::Percent => Some(BinaryOperator::Mod),
            &Token::Backslash => Some(BinaryOperator::Divide),
            _ => None
        }
    }
}