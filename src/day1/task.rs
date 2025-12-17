use std::fs::read_to_string;

pub fn day1() {
    calculate_floor();
    find_floor_position();
}

// part 1
fn calculate_floor() {
    let input: String = read_to_string("src/day1/input1.txt").unwrap();
    let mut floor: i32 = 0;

    for character in input.chars() {
        if character == '(' {
            floor = floor + 1;
        }
        if character == ')' {
            floor = floor - 1;
        }
    }
    println!("Day1 part 1 answer is {}", floor);
}

// part 2
fn find_floor_position() {
    let input: String = read_to_string("src/day1/input1.txt").unwrap();
    let mut floor: i32 = 0;
    let mut floor_position: usize = 0;

    for (index, character) in input.char_indices() {
        if character == '(' {
            floor = floor + 1;
        }
        if character == ')' {
            floor = floor - 1;
        }
        if floor == -1 {
            floor_position = index + 1;
            break;
        }
    }
    println!("Day1 part 2 answer is {}", floor_position);
}
