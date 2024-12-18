use std::fs;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let raw_input = fs::read("inputs/24d4.txt").expect("Something went wrong reading the file");
    let matrix = Matrix::init(raw_input);
    let nodes = matrix.find_nodes(b'X');
    let hits = nodes
        .par_iter()
        .map(|node| node.count_xmas(&matrix))
        .collect::<Vec<usize>>();
    let result = hits.iter().sum::<usize>();
    println!("Result 1: {}", result);

    let nodes = matrix.find_nodes(b'A');
    let hits = nodes
        .par_iter()
        .map(|node| node.count_cross_mas(&matrix))
        .collect::<Vec<usize>>();
    let result = hits.iter().sum::<usize>();
    println!("Result 2: {}", result);
}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn init(raw_input: Vec<u8>) -> Self {
        let mut matrix = Vec::new();
        let mut row = Vec::new();
        for c in raw_input {
            if c == b'\n' {
                matrix.push(row);
                row = Vec::new();
            } else {
                row.push(c);
            }
        }
        matrix.push(row);
        let rows = matrix.len();
        let cols = matrix[0].len();
        Self {
            data: matrix,
            rows,
            cols,
        }
    }
    fn get(&self, y: usize, x: usize) -> Option<u8> {
        if x < self.cols && y < self.rows {
            Some(self.data[y][x])
        } else {
            None
        }
    }
    fn find_nodes(&self, node: u8) -> Vec<Node> {
        let mut nodes = Vec::new();
        for y in 0..self.rows {
            for x in 0..self.cols {
                if self.data[y][x] == node {
                    nodes.push(Node::new(y, x));
                }
            }
        }
        nodes
    }
}

enum Direction {
    N,
    NW,
    W,
    SW,
    S,
    SE,
    E,
    NE,
}
impl Direction {
    fn get_all() -> Vec<Direction> {
        vec![
            Direction::N,
            Direction::NW,
            Direction::W,
            Direction::SW,
            Direction::S,
            Direction::SE,
            Direction::E,
            Direction::NE,
        ]
    }

    fn get_cross() -> Vec<Direction> {
        vec![Direction::NW, Direction::SW, Direction::SE, Direction::NE]
    }
    fn move_dir(&self, x: &mut i32, y: &mut i32) {
        (*x, *y) = match *self {
            Direction::N => (*x, *y - 1),
            Direction::NW => (*x - 1, *y - 1),
            Direction::W => (*x - 1, *y),
            Direction::SW => (*x - 1, *y + 1),
            Direction::S => (*x, *y + 1),
            Direction::SE => (*x + 1, *y + 1),
            Direction::E => (*x + 1, *y),
            Direction::NE => (*x + 1, *y - 1),
        }
    }
    fn opposite(&self) -> Direction {
        match *self {
            Direction::N => Direction::S,
            Direction::NW => Direction::SE,
            Direction::W => Direction::E,
            Direction::SW => Direction::NE,
            Direction::S => Direction::N,
            Direction::SE => Direction::NW,
            Direction::E => Direction::W,
            Direction::NE => Direction::SW,
        }
    }
}

#[derive(Debug)]
struct Node(usize, usize);

impl Node {
    fn new(y: usize, x: usize) -> Self {
        Self(y, x)
    }

    fn count_xmas(&self, matrix: &Matrix) -> usize {
        let counts: Vec<usize> = Direction::get_all()
            .par_iter()
            .map(|dir| self.check_xmas(matrix, dir))
            .collect();
        counts.iter().sum()
    }

    fn check_xmas(&self, matrix: &Matrix, dir: &Direction) -> usize {
        let mut y = self.0 as i32;
        let mut x = self.1 as i32;
        for l in [b'M', b'A', b'S'].iter() {
            dir.move_dir(&mut x, &mut y);
            if x < 0 || y < 0 {
                return 0;
            }
            if let Some(c) = matrix.get(y as usize, x as usize) {
                if c != *l {
                    return 0;
                }
            } else {
                return 0;
            }
        }
        1
    }

    fn count_cross_mas(&self, matrix: &Matrix) -> usize {
        let counts: Vec<usize> = Direction::get_cross()
            .par_iter()
            .map(|dir| self.check_cross_mas(matrix, dir))
            .collect();
        let sum: usize = counts.iter().sum();
        if sum == 2 {
            1
        } else {
            0
        }
    }

    fn check_cross_mas(&self, matrix: &Matrix, dir: &Direction) -> usize {
        //check if the value in the opposite direction is an 'M'
        // and the value in the direction is an 'S'
        let y0 = self.0 as i32;
        let x0 = self.1 as i32;
        let opposite = dir.opposite();
        let (mut bx, mut by) = (x0, y0);
        let (mut fx, mut fy) = (x0, y0);
        opposite.move_dir(&mut bx, &mut by);
        dir.move_dir(&mut fx, &mut fy);

        if let Some(c) = matrix.get(by as usize, bx as usize) {
            if c != b'M' {
                return 0;
            }
        } else {
            return 0;
        }
        if let Some(c) = matrix.get(fy as usize, fx as usize) {
            if c != b'S' {
                return 0;
            }
        } else {
            return 0;
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = b"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = Matrix::init(input.to_vec());
        let nodes = matrix.find_nodes(b'X');
        println!("{:?}", nodes);
        let hits = nodes
            .par_iter()
            .map(|node| node.count_xmas(&matrix))
            .collect::<Vec<usize>>();
        let result = hits.iter().sum::<usize>();
        assert_eq!(result, 18);
        let nodes = matrix.find_nodes(b'A');
        println!("{:?}", nodes);
        let hits = nodes
            .par_iter()
            .map(|node| node.count_cross_mas(&matrix))
            .collect::<Vec<usize>>();
        let result = hits.iter().sum::<usize>();
        assert_eq!(result, 9);
    }
}
