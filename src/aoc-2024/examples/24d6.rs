use std::{collections::HashMap, fs};

fn main() {
    let raw_input = fs::read("inputs/24d6.txt").expect("Something went wrong reading the file");
    let grid = Grid::from_raw(raw_input);
    let mut guard = grid.find_guard().unwrap();
    guard.patrol(&grid);
    println!("Visited spots: {}", guard.visited_spots());
}

struct Grid {
    data: Vec<Vec<u8>>,
}

impl Grid {
    fn from_raw(raw: Vec<u8>) -> Self {
        let mut grid = Vec::new();
        let mut row = Vec::new();
        for c in raw {
            if c == b'\n' {
                grid.push(row);
                row = Vec::new();
            } else {
                row.push(c);
            }
        }
        grid.push(row);
        Self { data: grid }
    }

    fn find_guard(&self) -> Option<Guard> {
        for (y, row) in self.data.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == b'^' {
                    let mut log = HashMap::new();
                    log.insert((x, y), 0);
                    return Some(Guard {
                        x,
                        y,
                        direction: Direction::Up,
                        log,
                    });
                }
            }
        }
        None
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
    log: HashMap<(usize, usize), usize>,
}

impl Guard {
    fn step(&mut self, grid: &Grid) -> Option<()> {
        let (nx, ny) = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        if nx >= grid.data.len() || ny >= grid.data[0].len() {
            return None;
        }
        match grid.data[ny][nx] {
            b'#' => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
            }
            b'.' | b'^' => {
                self.x = nx;
                self.y = ny;
                *self.log.entry((nx, ny)).or_insert(0) += 1;
            }
            _ => {
                return None;
            }
        }
        Some(())
    }
    fn patrol(&mut self, grid: &Grid) {
        while self.step(grid).is_some() {}
    }

    fn visited_spots(&self) -> usize {
        self.log.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &[u8] = b"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn test_grid() {
        let grid = Grid::from_raw(RAW.to_vec());
        let guard = grid.find_guard().unwrap();
        dbg!(&guard);
        assert_eq!(guard.x, 4);
        assert_eq!(guard.y, 6);
    }
}
