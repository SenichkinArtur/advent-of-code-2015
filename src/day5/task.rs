use std::fs::read_to_string;

pub fn day5() {
    let input: Vec<String> = get_lines("src/day5/input.txt");
    get_nice_strings_number(&input);
}

fn get_nice_strings_number(input: &Vec<String>) {
    for line in input {
        println!("{}", line);
    }
}

fn get_lines(path: &str) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}