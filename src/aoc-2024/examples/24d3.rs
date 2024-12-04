use std::fs;

fn main() {
    let raw_input = fs::read("inputs/24d3.txt").expect("Something went wrong reading the file");
    let mut c1 = Calculator::new(false);
    let r1 = c1.parse(&raw_input);
    println!("Result1: {}", r1);
    let mut c2 = Calculator::new(true);
    let r2 = c2.parse(&raw_input);
    println!("Result2: {}", r2);
}

enum State {
    ParsingLhs,
    ParsingRhs,
    ParsingOp,
    ParsingOpDo,
    ParsingOpDoNot,
}

struct Calculator {
    parsing: Vec<u8>,
    last_char: u8,
    lhs: u64,
    rhs: u64,
    acc: u64,
    state: State,
    branching: bool,
    enabled: bool,
}

impl Calculator {
    fn new(branching: bool) -> Self {
        Self {
            parsing: Vec::new(),
            last_char: 0,
            lhs: 0,
            rhs: 0,
            acc: 0,
            state: State::ParsingOp,
            branching,
            enabled: true,
        }
    }

    fn reset(&mut self) {
        self.parsing.clear();
        self.last_char = 0;
        self.lhs = 0;
        self.rhs = 0;
        self.state = State::ParsingOp;
    }

    fn parse_char(&mut self, c: u8) {
        match self.state {
            State::ParsingOp => self.handle_parsing_op(c),
            State::ParsingLhs => self.handle_parsing_lhs(c),
            State::ParsingRhs => self.handle_parsing_rhs(c),
            State::ParsingOpDo => self.handle_parsing_op_do(c),
            State::ParsingOpDoNot => self.handle_parsing_op_do_not(c),
        }
    }

    fn handle_parsing_op(&mut self, c: u8) {
        match (self.last_char, c) {
            (0, b'm') | (b'm', b'u') | (b'u', b'l') | (0, b'd') | (b'd', b'o') => {
                self.last_char = c;
            }
            (b'l', b'(') => {
                self.last_char = c;
                self.state = State::ParsingLhs;
            }
            (b'o', b'(') if self.branching => {
                self.last_char = c;
                self.state = State::ParsingOpDo;
            }
            (b'o', b'n') if self.branching => {
                self.last_char = c;
                self.state = State::ParsingOpDoNot;
            }
            _ => {
                self.reset();
            }
        }
    }

    fn handle_parsing_lhs(&mut self, c: u8) {
        match c {
            b'0'..=b'9' => {
                self.parsing.push(c);
            }
            b',' => {
                self.lhs = String::from_utf8(self.parsing.clone())
                    .unwrap()
                    .parse()
                    .unwrap();
                self.parsing.clear();
                self.state = State::ParsingRhs;
            }
            _ => {
                self.reset();
            }
        }
    }

    fn handle_parsing_rhs(&mut self, c: u8) {
        match c {
            b'0'..=b'9' => {
                self.parsing.push(c);
            }
            b')' => {
                self.rhs = String::from_utf8(self.parsing.clone())
                    .unwrap()
                    .parse()
                    .unwrap();
                self.parsing.clear();
                self.state = State::ParsingOp;
                if self.enabled || !self.branching {
                    self.acc += self.lhs * self.rhs;
                }
                self.reset();
            }
            _ => {
                self.reset();
            }
        }
    }

    fn handle_parsing_op_do(&mut self, c: u8) {
        // When parsing do() commands, the last character received is '('
        match (self.last_char, c) {
            (b'(', b')') => {
                self.enabled = true;
                self.reset();
            }
            _ => {
                self.reset();
            }
        }
    }

    fn handle_parsing_op_do_not(&mut self, c: u8) {
        // When parsing do_not() commands, the last character received is 'n'
        match (self.last_char, c) {
            (b'n', b'\'') | (b'\'', b't') | (b't', b'(') | (b'(', b')') => {
                self.enabled = false;
                self.reset();
            }
            _ => {
                self.reset();
            }
        }
    }

    fn parse(&mut self, raw: &Vec<u8>) -> u64 {
        raw.iter().for_each(|&c| self.parse_char(c));
        self.acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let mut calculator = Calculator::new(false);
        let result = calculator.parse(&input.to_vec());
        assert_eq!(result, 161);
    }
}
