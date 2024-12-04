use std::fs;

fn main() {
    let raw_input = fs::read("inputs/24d2.txt").expect("Something went wrong reading the file");
    let levels = Levels::from_raw(raw_input);
    let safe_levels = levels.0.iter().filter(|l| l.is_safe()).count();
    let dampened_safe_levels = levels.0.iter().filter(|l| l.dampened_safe()).count();
    println!("Safe levels: {}", safe_levels);
    println!("Dampened safe levels: {}", dampened_safe_levels);
}

#[derive(Debug)]
struct Levels(Vec<Level>);

#[derive(Debug)]
struct Level(Vec<u32>);

impl Levels {
    fn from_raw(raw: Vec<u8>) -> Self {
        let mut levels = Vec::new();
        raw.split(|&c| c == b'\n').for_each(|line| {
            if line.len() > 0 {
                levels.push(Level::from_raw(line));
            }
        });
        Levels(levels)
    }
}

impl Level {
    fn from_raw(raw: &[u8]) -> Self {
        let lnstr = String::from_utf8(raw.to_vec()).unwrap();
        let parsed: Vec<u32> = lnstr.split(" ").map(|s| s.parse().unwrap()).collect();
        Level(parsed)
    }

    fn is_safe(&self) -> bool {
        if let Some(tendency) = self.tendency() {
            for i in 0..(self.0.len() - 1) {
                match self.0[i].abs_diff(self.0[i + 1]) {
                    1..=3 => (),
                    _ => return false,
                }
                match self.0[i].cmp(&self.0[i + 1]) {
                    std::cmp::Ordering::Less => {
                        if tendency == Tendency::Down {
                            return false;
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        if tendency == Tendency::Up {
                            return false;
                        }
                    }
                    std::cmp::Ordering::Equal => return false,
                }
            }
        } else {
            return false;
        }

        true
    }

    fn dampened_safe(&self) -> bool {
        //Not the most efficient way to do this, but it works
        let mut levels = Vec::new();
        for i in 0..self.0.len() {
            let mut new_level = self.0.clone();
            new_level.remove(i);
            levels.push(Level(new_level));
        }
        levels.iter().any(|l| l.is_safe())
    }

    fn tendency(&self) -> Option<Tendency> {
        match self.0[0].cmp(&self.0[1]) {
            std::cmp::Ordering::Less => Some(Tendency::Up),
            std::cmp::Ordering::Greater => Some(Tendency::Down),
            std::cmp::Ordering::Equal => None,
        }
    }
}

enum Tendency {
    Up,
    Down,
}

impl PartialEq for Tendency {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tendency::Up, Tendency::Up) => true,
            (Tendency::Down, Tendency::Down) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let l = Level(vec![1, 3, 6, 7, 9]);
        assert!(l.is_safe());
    }

    #[test]
    fn test_is_not_valid() {
        let l = Level(vec![8, 6, 4, 4, 1]);
        assert!(!l.is_safe());
    }
}
