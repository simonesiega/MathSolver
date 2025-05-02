#[allow(unused)]
use std::io::{self, Write};

#[cfg(debug_assertions)]
macro_rules! trace_log { ($($arg:tt)*) => { log::trace!($($arg)*); }; }

#[cfg(debug_assertions)]
macro_rules! debug_log { ($($arg:tt)*) => { log::debug!($($arg)*); }; }

#[cfg(debug_assertions)]
macro_rules! info_log { ($($arg:tt)*) => { log::info!($($arg)*); }; }

#[cfg(debug_assertions)]
macro_rules! warn_log { ($($arg:tt)*) => { log::warn!($($arg)*); }; }

#[cfg(debug_assertions)]
macro_rules! error_log { ($($arg:tt)*) => { log::error!($($arg)*); }; }

#[cfg(not(debug_assertions))]
macro_rules! trace_log { ($($arg:tt)*) => {}; }

#[cfg(not(debug_assertions))]
macro_rules! debug_log { ($($arg:tt)*) => {}; }

#[cfg(not(debug_assertions))]
macro_rules! info_log { ($($arg:tt)*) => {}; }

#[cfg(not(debug_assertions))]
macro_rules! warn_log { ($($arg:tt)*) => {}; }

#[cfg(not(debug_assertions))]
macro_rules! error_log { ($($arg:tt)*) => {}; }



#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    Equals,
}

impl Token {
    #[inline]
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '=' => Some(Token::Equals),
            _ => None,
        }
    }

    #[allow(unused)]
    #[inline]
    fn is_operator(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Multiply | Token::Divide)
    }
}

#[derive(Debug)]
#[allow(unused)]
enum MathError {
    InvalidNumber(String),
    MissingParenthesis(usize),
    UnexpectedEnd,
    InvalidExpression(String),
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::InvalidNumber(msg) => {
                warn_log!("Numero non valido: {}", msg);
                write!(f, "Numero non valido: {}", msg)
            },
            MathError::MissingParenthesis(pos) => {
                warn_log!("Parentesi mancante alla posizione {}", pos);
                write!(f, "Parentesi mancante alla posizione {}", pos)   
            }
            MathError::UnexpectedEnd => {
                error_log!("Espressione terminata inaspettatamente");
                write!(f, "Espressione terminata inaspettatamente")
            },
            MathError::InvalidExpression(msg) => {
                error_log!("Espressione non valida: {}", msg);
                write!(f, "Espressione non valida: {}", msg)
            } 
        }
    }
}

impl std::error::Error for MathError {}

type MathResult = Result<f64, MathError>;

struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        info_log!("Creazione nuovo tokenizer con input: {}", input);
        Self {
            input,
            position: 0,
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, MathError> { 
        info_log!("Inizio tokenizzazione");
        let mut tokens = Vec::new();
        
        while self.position < self.input.len() {
            let c = self.current_char();
            match c {
                c if c.is_whitespace() => {
                    self.advance();
                }
                c if c.is_ascii_digit() || c == '.' => {
                    let token = self.parse_number()?;
                    trace_log!("Token numerico trovato: {:?}", token);
                    tokens.push(token);
                }
                c => {
                    if let Some(token) = Token::from_char(c) {
                        trace_log!("Token operatore trovato: {:?}", token);
                        tokens.push(token);
                        self.advance();
                    } else {
                        return Err(MathError::InvalidExpression(
                            format!("Carattere non valido: {}", c)
                        ));
                    }
                }
            }
        }
        
        info_log!("Tokenizzazione completata. Tokens: {:?}", tokens);
        Ok(tokens)
    }

    fn parse_number(&mut self) -> Result<Token, MathError> {
        let start = self.position;
        let mut has_decimal = false;

        while self.position < self.input.len() {
            match self.current_char() {
                c if c.is_ascii_digit() => {
                    self.advance();
                }
                '.' if !has_decimal => {
                    has_decimal = true;
                    self.advance();
                }
                '.' => return Err(MathError::InvalidNumber(
                    "Troppi punti decimali".to_string()
                )),
                _ => break,
            }
        }

        let number_str = &self.input[start..self.position];
        number_str.parse::<f64>()
            .map(Token::Number)
            .map_err(|_| MathError::InvalidNumber(number_str.to_string()))
    }

    #[inline]
    fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    #[inline]
    fn advance(&mut self) {
        self.position += 1;
    }
}

struct MathExpressionParser {
    tokens: Vec<Token>,
    position: usize,
}

impl MathExpressionParser {
    fn new(tokens: Vec<Token>) -> Self {
        info_log!("Creazione nuovo parser con tokens: {:?}", tokens);
        Self {
            tokens,
            position: 0,
        }
    }

    fn evaluate(&mut self) -> MathResult {
        info_log!("Inizio valutazione espressione");
        let result = self.evaluate_expression()?;
        
        match self.peek() {
            Some(&Token::Equals) => {
                info_log!("Espressione valutata correttamente: {}", result);
                Ok(result)
            }
            _ => Err(MathError::InvalidExpression("Manca il simbolo =".to_string()))
        }
    }

    fn evaluate_expression(&mut self) -> MathResult {
        debug_log!("Valutazione espressione alla posizione: {}", self.position);
        let mut result = self.evaluate_term()?;

        while let Some(token) = self.peek() {
            match *token {
                Token::Plus => {
                    self.advance();
                    let term = self.evaluate_term()?;
                    trace_log!("Addizione: {} + {}", result, term);
                    result += term;
                }
                Token::Minus => {
                    self.advance();
                    let term = self.evaluate_term()?;
                    trace_log!("Sottrazione: {} - {}", result, term);
                    result -= term;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn evaluate_term(&mut self) -> MathResult {
        debug_log!("Valutazione termine alla posizione: {}", self.position);
        let mut result = self.evaluate_factor()?;

        while let Some(token) = self.peek() {
            match *token {
                Token::Multiply => {
                    self.advance();
                    let factor = self.evaluate_factor()?;
                    trace_log!("Moltiplicazione: {} * {}", result, factor);
                    result *= factor;
                }
                Token::Divide => {
                    self.advance();
                    let factor = self.evaluate_factor()?;
                    if factor == 0.0 {
                        return Err(MathError::InvalidExpression(
                            "Divisione per zero".to_string()
                        ));
                    }
                    trace_log!("Divisione: {} / {}", result, factor);
                    result /= factor;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn evaluate_factor(&mut self) -> MathResult {
        debug_log!("Valutazione fattore alla posizione: {}", self.position);
        match self.next() {
            Some(Token::Number(n)) => Ok(n),
            Some(Token::Minus) => {
                let value = self.evaluate_factor()?;
                trace_log!("Negazione: -{}", value);
                Ok(-value)
            }
            Some(Token::LeftParen) => {
                let result = self.evaluate_expression()?;
                match self.next() {
                    Some(Token::RightParen) => {
                        trace_log!("Parentesi valutata: ({})", result);
                        Ok(result)
                    }
                    _ => Err(MathError::MissingParenthesis(self.position)),
                }
            }
            _ => Err(MathError::InvalidExpression(
                "Espressione non valida".to_string()
            )),
        }
    }

    #[inline]
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    #[inline]
    fn next(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    #[inline]
    fn advance(&mut self) {
        self.position += 1;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let input = "((8-9.81*3.14)-.12*(1*9/2.3)+-5.17)=";
    info_log!("Input espressione: {}", input);

    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize()?;
    let mut parser = MathExpressionParser::new(tokens);
    
    match parser.evaluate() {
        Ok(result) => {
            println!("Risultato: {:.3}", result);
            Ok(())
        }
        Err(e) => {
            eprintln!("Errore: {}", e);
            Err(Box::new(e))
        }
    }
}