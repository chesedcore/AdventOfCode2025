use std::fs;

fn get_instructions() -> Vec<String> {
    let stuff = fs::read_to_string("data.txt").unwrap();
    stuff.split(",")
        .map(|s| s.to_string())
        .collect()
}

fn is_invalid(s: &str) -> bool {
    let len = s.chars().count();
    for k in 1..=len/2 {
        if len % k != 0 { continue; }

        let block = &s[..k];
        let mut ok = true;

        let mut i = 0;
        while i < len {
            if &s[i..i+k] != block {
                ok = false;
                break;
            }
            i += k;
        }
        if ok {return true;}
    }
    false
}


fn count_hits_for_instruction(instruction: &str) -> i64 {
    let (pre_range, post_range) = instruction
        .split_once("-")
        .map(|(a, b) | (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .unwrap();

    let mut hits = 0;
    for num in pre_range..=post_range {
        let num_str: String = num.to_string();
        if is_invalid(&num_str) { hits += num; }
    }
    hits
}

fn count_hits_total(instructions: Vec<String>) -> i64 {
    let mut hits = 0;
    for inst in instructions {
        hits += count_hits_for_instruction(&inst);
    }
    hits
}

fn main() {
    let instructions = get_instructions();
    println!("Hits: {}", count_hits_total(instructions));
}

