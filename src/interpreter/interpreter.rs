use super::lexer::Lexer;
use super::token::Token;
use super::consts;

#[derive(Debug)]
pub struct Interpreter {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Interpreter {
    pub fn new(mut lexer: Lexer) -> Interpreter {
        let token = lexer.get_next_token();
        Interpreter {
            lexer: lexer,
            current_token: token,
        }
    }

    fn get_token(&self) -> Option<Token> {
        self.current_token.clone()
    }

    fn set_token(&mut self, token: Option<Token>) {
        self.current_token = token;
    }

    fn error(&self) {
        panic!("Invalid syntax")
    }

    fn eat(&mut self, token_type: &str) {
        let current_token = self.get_token().unwrap();
        if  current_token.get_type().eq(token_type) {
            let token = self.lexer.get_next_token();
            self.set_token(token);
        } else {
            self.error()
        }
    }

    fn factor(&mut self) -> f64 {
        let token = self.get_token().unwrap();
        let mut result = 0.0;
        if token.get_type() == consts::NUMBER {
            self.eat(consts::NUMBER);
            result = token.get_value().parse::<f64>().unwrap();
        } else if token.get_type() == consts::LPAREN {
            self.eat(consts::LPAREN);
            result = self.expr();
            self.eat(consts::RPAREN);
        }

        result
    }

    fn pow_term(&mut self) -> f64 {
        let mut result = self.factor();
        let ops = vec![consts::POW];

        while ops.contains(&self.get_token().unwrap().get_type().as_str()) {
            let token = self.get_token().unwrap();
            if token.get_type().eq(consts::POW) {
                self.eat(consts::POW);
                result = result.powf(self.factor());
            }
        }

        result
    }

    fn term(&mut self) -> f64 {
        let mut result = self.pow_term();
        let ops = vec![consts::DIV, consts::MUL];

        while ops.contains(&self.get_token().unwrap().get_type().as_str()) {
            let token = self.get_token().unwrap();
            if token.get_type().eq(consts::DIV) {
                self.eat(consts::DIV);
                result /= self.pow_term();
            } else if token.get_type().eq(consts::MUL) {
                self.eat(consts::MUL);
                result *= self.pow_term();
            } else if token.get_type().eq(consts::MOD) {
                self.eat(consts::MOD);
                result = result % self.pow_term();
            }
        }

        return result;
    }

    pub fn expr(&mut self) -> f64{

        let mut result = self.term();
        let ops = vec![consts::PLUS, consts::MINUS];

        while ops.contains(&self.get_token().unwrap().get_type().as_str()) {
            let token = self.get_token().unwrap();
            if token.get_type().eq(consts::PLUS) {
                self.eat(consts::PLUS);
                result += self.term();
            } else if token.get_type().eq(consts::MINUS) {
                self.eat(consts::MINUS);
                result -= self.term();
            }
        }

        return result;
    }
} 