use std::collections::HashMap;
use std::fs::read_to_string;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const BANNED_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn day5() {
    let input: Vec<String> = get_lines("src/day5/input.txt");

    let part1: i32 = get_nice_strings_number(&input);
    let part2: i32 = get_nice_strings_number_new_model(&input);

    println!("Day 5 part 1 answer is {}", part1);
    println!("Day 5 part 2 answer is {}", part2);
}

fn get_nice_strings_number(input: &[String]) -> i32 {
    let mut nice_strings_count: i32 = 0;

    'outer: for line in input {
        if BANNED_STRINGS.iter().any(|s| line.contains(s)) {
            continue 'outer;
        }

        if line.chars().filter(|c| VOWELS.contains(c)).count() < 3 {
            continue 'outer;
        }

        if line.as_bytes().windows(2).any(|w| w[0] == w[1]) {
            nice_strings_count += 1;
        }
    }

    return nice_strings_count;
}

fn get_nice_strings_number_new_model(input: &[String]) -> i32 {
    let mut nice_strings_count: i32 = 0;

    for line in input {
        if !line.as_bytes().windows(3).any(|w| w[0] == w[2]) {
            continue;
        }

        let mut seen: HashMap<(u8, u8), usize> = HashMap::new();

        for (i, w) in line.as_bytes().windows(2).enumerate() {
            let pair: (u8, u8) = (w[0], w[1]);

            if let Some(&j) = seen.get(&pair) {
                if i - j >= 2 {
                    nice_strings_count += 1;
                    break;
                }
            } else {
                seen.insert(pair, i);
            }
        }
    }

    return nice_strings_count;
}

fn get_lines(path: &str) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
