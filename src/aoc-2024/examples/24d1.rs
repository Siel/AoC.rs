use std::fs;

fn main() {
    let content = fs::read("inputs/24d1.txt").expect("Something went wrong reading the file");
    //split the content by lines
    let (mut l1, mut l2) = parse_lines(content);
    l1.sort();
    l2.sort();
    let res: u32 = l1
        .iter()
        .zip(l2.iter())
        .fold(0, |acc, (&a, &b)| a.abs_diff(b) + acc);
    println!("Result 1: {}", res);
}

fn parse_lines(raw_content: Vec<u8>) -> (Vec<u32>, Vec<u32>) {
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    raw_content.split(|&c| c == b'\n').for_each(|line| {
        if line.len() > 0 {
            let (a, b) = parse_line(line);
            l1.push(a);
            l2.push(b);
        }
    });
    (l1, l2)
}

fn parse_line(raw_line: &[u8]) -> (u32, u32) {
    let lnstr = String::from_utf8(raw_line.to_vec()).unwrap();
    let parsed: Vec<u32> = lnstr.split("   ").map(|s| s.parse().unwrap()).collect();
    (parsed[0], parsed[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
    }
}
