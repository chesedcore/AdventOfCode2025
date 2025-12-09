use std::fs;

fn get_instructions() -> Vec<String> {
    let stuff = fs::read_to_string("data.txt").unwrap();
    stuff.lines()
        .map(|s| s.to_string())
        .collect()
}

fn char_to_i32(ch: char) -> i32 {
    ch.to_digit(10).unwrap() as i32
}

fn idx(string: &str, idx: usize) -> char {
    string.chars().nth(idx).unwrap()
}

fn join(ch: char, ch2: char) -> i32 {
    10 * char_to_i32(ch) + char_to_i32(ch2)
}

fn get_joltage_from(bank: &str) -> i32 {
    let len = bank.chars().count();
    let mut max = 0;
    for i in 0..len {
        for j in i+1..len {
            let ch = idx(bank, i);
            let ch2 = idx(bank, j);
            let num = join(ch, ch2);
            if num > max { max = num; }
        }
    }
    max
}

fn get_peak_joltage(banks: Vec<String>) -> i32 {
    let mut joltage = 0;
    for bank in banks {
        joltage += get_joltage_from(&bank);
    }
    joltage
}
fn main() {
    let banks = get_instructions();
    println!("Peak joltage: {}", get_peak_joltage(banks));
}
