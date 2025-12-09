use std::fs;

enum Rotation {
    Left(i32),
    Right(i32),
}

fn get_instructions() -> Vec<String> {

    let stuff = fs::read_to_string("rotations.txt").unwrap();
    stuff.lines()
        .map(|s| s.to_string())
        .collect()
}

fn instructions_to_rotations(lines: Vec<String>) -> Vec<Rotation> {
    lines.into_iter()
        .map(|line| {
            let (dir, num_str) = line.split_at(1);
            let value: i32 = num_str.parse().unwrap();

            match dir {
                "L" => Rotation::Left(value),
                "R" => Rotation::Right(value),
                _ => unreachable!(),
            }
        }).collect()
}

fn count_zero_hits(rotations: Vec<Rotation>) -> i32 {
    let mut acc = 50;
    let mut hits = 0;

    for rot in rotations {
        let delta = match rot {
            Rotation::Left(v) => -v,
            Rotation::Right(v) => v,
        };
        
        let old_raw = acc;
        let new_raw = old_raw + delta;
        let old_block;
        let new_block;

        if delta >= 0 {
            old_block = old_raw.div_euclid(100);
            new_block = new_raw.div_euclid(100);
        } else {
            old_block = (old_raw - 1).div_euclid(100);
            new_block = (new_raw - 1).div_euclid(100);
        }
        hits += (old_block - new_block).abs();
        acc = new_raw.rem_euclid(100);

    }

    hits

}

fn main() {
    let stuff = get_instructions();
    let rotations = instructions_to_rotations(stuff);
    println!("{}", count_zero_hits(rotations));
}