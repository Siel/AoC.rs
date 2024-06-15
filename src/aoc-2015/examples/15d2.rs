fn main() {
    let content = std::fs::read_to_string("inputs/2015/d2.txt").expect("Input file not found");

    let needed = content.split("\n").fold((0, 0), |acc, x| {
        let dim: Vec<&str> = x.split("x").collect();
        if dim.len() == 3 {
            let l = dim[0].parse::<u32>().unwrap();
            let w = dim[1].parse::<u32>().unwrap();
            let h = dim[2].parse::<u32>().unwrap();
            (
                acc.0 + area_per_box(l, w, h),
                acc.1 + ribbon_per_box(l, w, h),
            )
        } else {
            println!("Non parseable line? EoF?");
            acc
        }
    });
    println!("Solution 1: {}", needed.0);
    println!("Solution 2: {}", needed.1);
}

fn area_per_box(l: u32, w: u32, h: u32) -> u32 {
    let a = l * w;
    let b = l * h;
    let c = w * h;
    let min = a.min(b).min(c);

    2 * a + 2 * b + 2 * c + min
}

fn ribbon_per_box(l: u32, w: u32, h: u32) -> u32 {
    let mut sides = [l, w, h];
    sides.sort();
    2 * sides[0] + 2 * sides[1] + l * w * h
}
