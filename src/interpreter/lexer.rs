use super::{consts, token::Token};

#[derive(Debug, Clone)]
pub struct Lexer {
    text: String,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Lexer{
        let txt = text.clone();
        Lexer {
            text: text,
            pos: 0,
            current_char: txt.chars().nth(0),
        }
    }
    
    fn get_text(&self) -> String {
        self.text.clone()
    }

    fn get_current_char(&self) -> Option<char> {
        self.current_char.clone()
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.get_text().chars().nth(self.pos);
    }

    fn skip_whitespace(&mut self) {
        while !self.get_current_char().is_none() && self.get_current_char().unwrap().is_ascii_whitespace() {
            self.advance()
        }
    }

    fn number(&mut self) -> String {
        let mut result = String::new();
        while !self.get_current_char().is_none() && 
                (self.get_current_char().unwrap().is_ascii_digit() || 
                self.get_current_char().unwrap() == '.') {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        result
    }

    pub fn get_next_token(&mut self) -> Option<Token> {

        while !self.get_current_char().is_none() {
            let current_char = self.get_current_char().unwrap();
            if current_char.is_ascii_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if current_char.is_ascii_digit() || current_char == '.' {
                return Some(Token::new(
                    consts::NUMBER, 
                    Some(self.number())
                ));
            }
            
            if current_char == '+' {
                self.advance();
                return Some(Token::new(
                    consts::PLUS, 
                    Some(current_char.to_string())
                ));
            }
            
            if current_char == '-' {
                self.advance();
                return Some(Token::new(
                    consts::MINUS, 
                    Some(current_char.to_string())
                ));
            }
            
            if current_char == '/' {
                self.advance();
                return Some(Token::new(
                    consts::DIV, 
                    Some(current_char.to_string())
                ));
            }
            
            if current_char == '*' {
                self.advance();
                return Some(Token::new(
                    consts::MUL, 
                    Some(current_char.to_string())
                ));
            }
            
            if current_char == '%' {
                self.advance();
                return Some(Token::new(
                    consts::MOD, 
                    Some(current_char.to_string())
                ));
            }
            
            if current_char == '^' {
                self.advance();
                return Some(Token::new(
                    consts::POW, 
                    Some(current_char.to_string())
                ));
            }

            if current_char == '(' {
                self.advance();
                return Some(Token::new(
                    consts::LPAREN, 
                    Some(current_char.to_string())
                ));
            }

            if current_char == ')' {
                self.advance();
                return Some(Token::new(
                    consts::RPAREN, 
                    Some(current_char.to_string())
                ));
            }

            self.error();
    
        }

        return Some(Token::new(consts::EOF, None));
    }

    fn error(&self) {
        panic!("Invalid character")
    }
}