use std::fs;

fn main() {
    let raw_input = fs::read("inputs/24d3.txt").expect("Something went wrong reading the file");
    let mut calculator = Calculator::new();
    let result = calculator.parse(raw_input);
    println!("Result: {}", result);
}

enum State {
    ParsingLhs,
    ParsingRhs,
    ParsingOp,
}

struct Calculator {
    parsing: Vec<u8>,
    last_char: u8,
    lhs: u64,
    rhs: u64,
    acc: u64,
    state: State,
}

impl Calculator {
    fn new() -> Self {
        Self {
            parsing: Vec::new(),
            last_char: 0,
            lhs: 0,
            rhs: 0,
            acc: 0,
            state: State::ParsingOp,
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
            State::ParsingOp => match (self.last_char, c) {
                (0, b'm') | (b'm', b'u') | (b'u', b'l') => {
                    self.last_char = c;
                }
                (b'l', b'(') => {
                    self.last_char = c;
                    self.state = State::ParsingLhs;
                }
                _ => {
                    self.reset();
                }
            },
            State::ParsingLhs => match c {
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
            },
            State::ParsingRhs => match c {
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
                    self.acc += self.lhs * self.rhs;
                    self.reset();
                }
                _ => {
                    self.reset();
                }
            },
        }
    }

    fn parse(&mut self, raw: Vec<u8>) -> u64 {
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
        let mut calculator = Calculator::new();
        let result = calculator.parse(input.to_vec());
        assert_eq!(result, 161);
    }
}
