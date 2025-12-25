use std::fs::read_to_string;

pub fn day7() {
    let input: Vec<String> = get_lines("src/day7/input.txt");
    let part1 = get_signal(&input);

    println!("Day 8 part 1 answer is {}", part1);
}

fn get_signal(input: &[String]) -> i32 {

    1
}

fn get_lines(path: &str) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}