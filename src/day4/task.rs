use md5::Digest;

pub fn day4() {
    let input: &str = "ckczppom";

    let part1: u64 = get_lowest_number_for_md5(input, "00000");
    let part2: u64 = get_lowest_number_for_md5_bytes(input);

    println!("Day 4 part 1 answer is {}", part1);
    println!("Day 4 part 2 answer is {}", part2);
}

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

fn get_lowest_number_for_md5_bytes(input: &str) -> u64 {
    let mut lowest_number: u64 = 1;

    loop {
        let digest: Digest = md5::compute(format!("{}{}", input, lowest_number));
        let bytes = digest.0;

        if bytes[0] == 0 && bytes[1] == 0 && bytes[2] == 0 {
            return lowest_number;
        }

        lowest_number += 1;
    }
}
