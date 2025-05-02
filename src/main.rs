use std::io::{self, Write};

#[derive(Debug)]
struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn parse(&mut self) -> f64 {
        let result = self.parse_expression();
        self.skip_whitespace();
        match self.peek() {
            Some('=') => result,
            Some(c) => panic!("Expected '=' at the end, found '{}'", c),
            None => panic!("Expected '=' at the end, but reached end of input"),
        }
    }

    fn parse_expression(&mut self) -> f64 {
        println!("[parse_expression] pos: {}", self.pos);
        let mut value = self.parse_product();
        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('+') => {
                    println!("[parse_expression] found '+' at {}", self.pos);
                    self.consume();
                    value += self.parse_product();
                }
                Some('-') => {
                    println!("[parse_expression] found '-' at {}", self.pos);
                    self.consume();
                    value -= self.parse_product();
                }
                _ => break,
            }
        }
        value
    }

    fn parse_product(&mut self) -> f64 {
        println!("[parse_product] pos: {}", self.pos);
        let mut value = self.parse_term();
        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('*') => {
                    println!("[parse_product] found '*' at {}", self.pos);
                    self.consume();
                    value *= self.parse_term();
                }
                Some('/') => {
                    println!("[parse_product] found '/' at {}", self.pos);
                    self.consume();
                    value /= self.parse_term();
                }

                /*
                * Moltiplicazione implicita nei casi:
                * Un numero è seguito da una parentesi: .12(…)
                * Una parentesi è seguita da un numero: (2+1)3
                * Due parentesi sono adiacenti: (2)(3)
                 */
                Some('(') => {
                    println!("[parse_product] found implicit '*' before '(' at {}", self.pos);
                    value *= self.parse_term();
                }
                Some(c) if c.is_ascii_digit() || c == '.' => {
                    println!("[parse_product] found implicit '*' before digit at {}", self.pos);
                    value *= self.parse_term();
                }
                _ => break,
            }
        }
        value
    }

    fn parse_term(&mut self) -> f64 {
        self.skip_whitespace();
        println!("[parse_term] pos: {}, next: {:?}", self.pos, self.peek());
        match self.peek() {
            Some('-') => {
                self.consume();
                -self.parse_term()
            }
            Some('(') => {
                self.consume();
                let value = self.parse_expression();
                self.skip_whitespace();
                match self.peek() {
                    Some(')') => {
                        self.consume();
                        value
                    }
                    _ => panic!("Expected ')' at position {}", self.pos),
                }
            }
            _ => self.parse_number(),
        }
    }

    fn parse_number(&mut self) -> f64 {
        self.skip_whitespace();
        let mut num = String::new();
        let mut dot_seen = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num.push(c);
                self.consume();
            } else if c == '.' && !dot_seen {
                dot_seen = true;
                num.push(c);
                self.consume();
            } else {
                break;
            }
        }

        if num.is_empty() || num == "." {
            panic!("Expected number at position {}, found {:?}", self.pos, self.peek());
        }

        let parsed = num.parse::<f64>().expect("Invalid number format");
        println!("[parse_number] parsed '{}' as {}", num, parsed);
        parsed
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn consume(&mut self) {
        self.pos += 1;
    }
}

fn main() {
    println!("Inserisci un'espressione che termina con '=' (es: ((8-9.81*3.14)-.12(1*9/2.3)+-5.17)= )");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut parser = Parser::new(&input.trim());
    let result = parser.parse();
    println!("Risultato: {}", result);
}
