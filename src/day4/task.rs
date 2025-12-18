use md5::Digest;
use std::time::Instant;

pub fn day4() {
    let input: &str = "ckczppom";

    let start1 = Instant::now();
    let part1: u64 = get_lowest_number_for_md5(input, "00000");
    let elapsed1 = start1.elapsed();
    println!("took {:.3?}", elapsed1);

    let start2 = Instant::now();
    let part2: u64 = get_lowest_number_for_md5(input, "000000");
    let elapsed2 = start2.elapsed();
    println!("took {:.3?}", elapsed2);

    println!("Day 4 part 1 answer is {}", part1);
    println!("Day 4 part 2 answer is {}", part2);
}

// TODO: need to optimize
fn get_lowest_number_for_md5(input: &str, target: &str) -> u64 {
    let mut lowest_number: u64 = 1;

    loop {
        let digest: Digest = md5::compute(format!("{}{}", input, lowest_number));
        let hash: String = format!("{:x}", digest);

        if hash.starts_with(target) {
            return lowest_number;
        }

        lowest_number += 1;
    }
}
