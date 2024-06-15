fn main() {
    let content = std::fs::read("inputs/2015/d1.txt").expect("Input file not found");
    let mut floor = 0;
    let mut basement: Option<usize> = None;
    for (i, inst) in content.iter().enumerate() {
        match inst {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => {}
        }
        if floor == -1 && basement.is_none() {
            basement = Some(i + 1);
        }
    }
    println!("Solution 1: {floor}");
    println!("Solution 2: {basement:?}");
}
