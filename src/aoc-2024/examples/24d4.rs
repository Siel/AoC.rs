use std::fs;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let raw_input = fs::read("inputs/24d4.txt").expect("Something went wrong reading the file");
    let matrix = Matrix::init(raw_input);
    let nodes = matrix.find_all_x();
    let hits = nodes
        .par_iter()
        .map(|node| node.count_xmas(&matrix))
        .collect::<Vec<usize>>();
    let result = hits.iter().sum::<usize>();
    println!("Result: {}", result);
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
    fn find_all_x(&self) -> Vec<Node> {
        let mut nodes = Vec::new();
        for y in 0..self.rows {
            for x in 0..self.cols {
                if self.data[y][x] == b'X' {
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
            match dir {
                Direction::N => y -= 1,
                Direction::NW => {
                    x -= 1;
                    y -= 1;
                }
                Direction::W => x -= 1,
                Direction::SW => {
                    x -= 1;
                    y += 1;
                }
                Direction::S => y += 1,
                Direction::SE => {
                    x += 1;
                    y += 1;
                }
                Direction::E => x += 1,
                Direction::NE => {
                    x += 1;
                    y -= 1;
                }
            }
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
        let nodes = matrix.find_all_x();
        println!("{:?}", nodes);
        let hits = nodes
            .par_iter()
            .map(|node| node.count_xmas(&matrix))
            .collect::<Vec<usize>>();
        let result = hits.iter().sum::<usize>();
        assert_eq!(result, 18);
    }
}
