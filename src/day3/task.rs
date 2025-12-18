use std::fs::read_to_string;

const NORTH: char = '^';
const SOUTH: char = 'v';
const EAST: char = '>';
const WEST: char = '<';

pub fn day3() {
    get_number_visited_houses();
    get_number_visited_houses_robo();
}

fn get_number_visited_houses() {
    let input: String = read_to_string("src/day3/input.txt").unwrap();
    let mut current_pos: [i32; 2] = [0, 0]; // [x, y]
    let mut visited_houses_coords: Vec<[i32; 2]> = vec![current_pos];

    for char in input.chars() {
        set_visited_coords(char, &mut visited_houses_coords, &mut current_pos);
    }

    println!("Day 3 part 1 answer is {}", visited_houses_coords.len());
}

fn get_number_visited_houses_robo() {
    let input: String = read_to_string("src/day3/input.txt").unwrap();
    let mut current_pos_santa: [i32; 2] = [0, 0]; // [x, y]
    let mut current_pos_robo_santa: [i32; 2] = [0, 0]; // [x, y]
    let mut visited_houses_coords: Vec<[i32; 2]> = vec![current_pos_santa];

    for (index, char) in input.char_indices() {
        let is_santa: bool = index % 2 == 0;

        if is_santa {
            set_visited_coords(char, &mut visited_houses_coords, &mut current_pos_santa);
        } else {
            set_visited_coords(char, &mut visited_houses_coords, &mut current_pos_robo_santa);
        }
    }

    println!("Day 3 part 2 answer is {}", visited_houses_coords.len());
}

fn set_visited_coords(char: char, visited_houses_coords: &mut Vec<[i32; 2]>, current_pos: &mut [i32; 2]) {
    match char {
        NORTH => current_pos[1] += 1,
        SOUTH => current_pos[1] -= 1,
        EAST => current_pos[0] += 1,
        WEST => current_pos[0] -= 1,
        _ => {}
    }
    if !visited_houses_coords.contains(current_pos) {
        visited_houses_coords.push(*current_pos);
    }
}
