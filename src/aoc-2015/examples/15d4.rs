use core::num;

const INPUT: &str = "yzbqklnj";

fn main() {
    for (i, n_sol) in [5, 6].iter().enumerate() {
        let mut key = 0;
        loop {
            let key_s = key.to_string();
            let appended = format!("{}{}", INPUT, key_s);
            let digest = md5::compute(appended);
            if valid_hash(digest.0, *n_sol) {
                println!("Hash: {:x}", digest);
                println!("Solution {}: {}", i + 1, key);
                break;
            }
            key += 1;
        }
    }
}

fn valid_hash(hash: [u8; 16], num_zeros: usize) -> bool {
    let odd = num_zeros % 2 == 1;
    let num_zeros = num_zeros / 2;
    for i in 0..num_zeros {
        if hash[i] != 0x0 {
            return false;
        }
    }
    if odd && hash[num_zeros] & 0xf0 != 0 {
        return false;
    }
    true
}
