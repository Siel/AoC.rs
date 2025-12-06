use std::fs;

struct Lock {
    pointing_to: usize,
    zero_stop_count: usize,
    zero_pass_count: usize,
}

impl Default for Lock {
    fn default() -> Self {
        Lock {
            pointing_to: 50,
            zero_stop_count: 0,
            zero_pass_count: 0,
        }
    }
}

impl Lock {
    fn rotate_right(&mut self, amount: usize) {
        self.zero_pass_count += amount / 100;
        let rem = amount % 100;
        if self.pointing_to + rem >= 100 {
            self.zero_pass_count += 1;
        }
        self.pointing_to = (self.pointing_to + rem) % 100;
    }
    fn rotate_left(&mut self, amount: usize) {
        self.zero_pass_count += amount / 100;
        let rem = amount % 100;
        if rem >= self.pointing_to && self.pointing_to != 0 {
            self.zero_pass_count += 1;
        }
        self.pointing_to = (self.pointing_to + 100 - rem) % 100;
    }

    fn decode_command(&mut self, command: &[u8]) -> Result<(), String> {
        if command.len() < 2 {
            return Err("Command too short".to_string());
        }
        match command[0] {
            b'R' => {
                self.rotate_right(decode_amount(&command[1..]));
            }
            b'L' => {
                self.rotate_left(decode_amount(&command[1..]));
            }
            _ => return Err("Not valid direction, expecting R/L".to_string()),
        }
        if self.pointing_to == 0 {
            self.zero_stop_count += 1;
        }
        Ok(())
    }

    fn zero_stop_count(&self) -> usize {
        self.zero_stop_count
    }
    fn zero_pass_count(&self) -> usize {
        self.zero_pass_count
    }
}

#[inline]
fn decode_amount(amount: &[u8]) -> usize {
    let amtstr = String::from_utf8(amount.to_vec()).unwrap();
    amtstr.trim().parse().unwrap()
}

fn main() -> Result<(), String> {
    let input = fs::read("inputs/25d1.txt").expect("Cannot open file");
    let mut lock = Lock::default();
    for line in input.split(|&c| c == b'\n') {
        lock.decode_command(line)?;
    }
    println!("Zero stop count: {}", lock.zero_stop_count());
    println!("Zero pass count: {}", lock.zero_pass_count());
    Ok(())
}
