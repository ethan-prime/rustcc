use std::{fs::File, io::Read};

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Integer(i32),
    Void,
    Return,
    Semicolon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Int,
    EOF,
    Tilde,
    Hyphen,
    Decrement,
}
pub struct Lexer {
    input: String,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(filename: &str) -> Self {
        let mut l = Lexer { input: String::new(), pos: 0, line: 0 };
        let mut f = File::open(filename).expect("Failed to open file.");
        f.read_to_string(&mut l.input).unwrap();
        l
    }

    pub fn error(&self, msg: &'static str) -> String {
        let s = format!("[lexer] line {}: {}", self.line, msg);
        s
    }

    fn char_at(&self, n: usize) -> Option<char> {
       self.input.chars().nth(n)
    }

    fn cur_char(&self) -> Result<char, String> {
        match self.char_at(self.pos) {
            Some(c) => Ok(c),
            None => Err("exceeded bounds trying to read current character".to_string()),
        }
    }

    fn peek_char(&self) -> Result<char, String> {
        match self.char_at(self.pos + 1) {
            Some(c) => Ok(c),
            None => Err("exceeded bounds trying to read current character".to_string()),
        }
    }

    // parse ends at last character collected.
    pub fn collect_number(&mut self) -> Result<i32, String> {
        let mut c = self.cur_char()?;
        if !c.is_numeric() {
            return Err(self.error("expected a numeric char"));
        } 

        let mut res_string = String::new();
        while c.is_numeric() {
            res_string.push(c);
            self.pos += 1;
            c = match self.cur_char() {
                Ok(c) => c,
                Err(_) => {
                    break;
                }
            }

        }

        self.pos -= 1;
        Ok(res_string.parse::<i32>().unwrap())
    }

    // parse ends at last character collected.
    pub fn collect_identifier_str(&mut self) -> Result<String, String> {
        let mut c = self.cur_char()?;
        
        if !(c.is_ascii_alphanumeric() || c == '_') {
            return Err(self.error("expected an alphanumeric char"));
        } 

        let mut res_string = String::new();
        while c.is_ascii_alphanumeric() || c == '_' {
            res_string.push(c);
            self.pos += 1;
            c = match self.cur_char() {
                Ok(c) => c,
                Err(_) => {
                    break;
                }
            }
        }

        self.pos -= 1;
        Ok(res_string)
    }

    fn skip_whitespace(&mut self) {
        while let Ok(c) = self.cur_char() {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let match_identifier = | x: String | -> Token {
            match x.as_str() {
                "int" => Token::Int,
                "return" => Token::Return,
                "void" => Token::Void,
                _ => Token::Identifier(x),
            }
        };

        let mut res: Vec<Token> = Vec::new();

        while let Ok(c) = self.cur_char() {
            if c.is_whitespace() { 
                self.skip_whitespace(); 
                continue; 
            };
            let tok = match c {
                ';' => Token::Semicolon,
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '{' => Token::OpenBrace,
                '}' => Token::CloseBrace,
                '~' => Token::Tilde,
                '-' => {
                    if let Ok(c) = self.peek_char() && c == '-' {
                        self.pos += 1; 
                        Token::Decrement
                    } else {
                        Token::Hyphen
                    }
                }
                _ => {
                    if c.is_numeric() {
                        Token::Integer(self.collect_number().unwrap())
                    } else {
                        match_identifier(self.collect_identifier_str().unwrap())
                    }
                },
            };

            res.push(tok);
            self.pos += 1;
        }
        res
    }

}