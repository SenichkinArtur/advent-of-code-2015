use std::fs::read_to_string;
use unescaper::unescape;

const DOUBLE_QUOTES_LENGTH: usize = 2;

pub fn day8() {
    let input: Vec<String> = get_lines("src/day8/input.txt");
    let part1 = get_space(&input);

    println!("Day 8 part 1 answer is {}", part1);
}

fn get_space(lines: &[String]) -> usize {
    let mut code_len: usize = 0;
    let mut memory_len: usize = 0;

    for line in lines {
        code_len += line.len();
        memory_len += unescape(line).unwrap().chars().count() - DOUBLE_QUOTES_LENGTH;
        println!(
            "line: {}, unescaped: {}, code_len: {}, memory_len: {}",
            line,
            unescape(line).unwrap(),
            code_len,
            memory_len
        );
    }

    println!("chars_code_number: {}", code_len);
    println!("chars_memory_number: {}", memory_len);

    code_len - memory_len
}

fn get_lines(path: &str) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
