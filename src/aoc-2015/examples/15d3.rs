use std::collections::HashMap;

#[derive(Default)]
struct Position {
    x: i32,
    y: i32,
    log: HashMap<String, u32>,
}

impl Position {
    fn new() -> Position {
        let mut pos = Position {
            x: 0,
            y: 0,
            log: HashMap::new(),
        };
        pos.log_current_position();
        pos
    }
    fn step(&mut self, code: u8) {
        match code {
            b'^' => self.y += 1,
            b'v' => self.y -= 1,
            b'>' => self.x += 1,
            b'<' => self.x -= 1,
            _ => panic!("not valid code"),
        }
        self.log_current_position();
    }

    fn log_current_position(&mut self) {
        let key = format!("{}.{}", self.x, self.y);
        match &self.log.get(&key) {
            None => self.log.insert(key, 1),
            Some(val) => self.log.insert(key, *val + 1),
        };
    }

    fn visited_houses(&self) -> u32 {
        self.log.len() as u32
    }

    fn merge_log(&mut self, other: &Position) {
        for (key, val) in other.log.iter() {
            match &self.log.get(key) {
                None => self.log.insert(key.clone(), *val),
                Some(v) => self.log.insert(key.clone(), *v + *val),
            };
        }
    }
}

fn main() {
    let content = std::fs::read("inputs/2015/d3.txt").expect("Input file not found");
    let mut s1 = Position::new();
    let mut s2 = Position::new();
    let mut rs2 = Position::new();
    for (i, code) in content.iter().enumerate() {
        s1.step(*code);
        if i % 2 == 0 {
            s2.step(*code);
        } else {
            rs2.step(*code);
        }
    }
    s2.merge_log(&rs2);
    println!("Solution 1: {}", s1.visited_houses());
    println!("Solution 2: {}", s2.visited_houses());
}
