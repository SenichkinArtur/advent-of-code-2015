use std::collections::HashSet;
use std::fs::read_to_string;

const NORTH: char = '^';
const SOUTH: char = 'v';
const EAST: char = '>';
const WEST: char = '<';

type CoordsArray = (i32, i32);

pub fn day3() {
    let input: String = read_to_string("src/day3/input.txt").unwrap();

    let part1: usize = get_number_visited_houses(&input);
    let part2: usize = get_number_visited_houses_robo(&input);

    println!("Day 3 part 1 answer is {}", part1);
    println!("Day 3 part 2 answer is {}", part2);
}

fn get_number_visited_houses(input: &str) -> usize {
    let mut current_pos: CoordsArray = (0, 0); // [x, y]
    let mut visited_houses_coords: HashSet<CoordsArray> = HashSet::new();
    visited_houses_coords.insert(current_pos);

    for direction in input.trim().chars() {
        set_visited_coords(direction, &mut visited_houses_coords, &mut current_pos);
    }

    return visited_houses_coords.len();
}

fn get_number_visited_houses_robo(input: &str) -> usize {
    let mut current_pos_santa: CoordsArray = (0, 0); // [x, y]
    let mut current_pos_robo_santa: CoordsArray = (0, 0); // [x, y]
    let mut visited_houses_coords: HashSet<CoordsArray> = HashSet::new();
    visited_houses_coords.insert(current_pos_santa);
    visited_houses_coords.insert(current_pos_robo_santa);

    for (index, direction) in input.trim().chars().enumerate() {
        let is_santa: bool = index % 2 == 0;

        if is_santa {
            set_visited_coords(
                direction,
                &mut visited_houses_coords,
                &mut current_pos_santa,
            );
        } else {
            set_visited_coords(
                direction,
                &mut visited_houses_coords,
                &mut current_pos_robo_santa,
            );
        }
    }

    return visited_houses_coords.len();
}

fn set_visited_coords(
    direction: char,
    visited_houses_coords: &mut HashSet<CoordsArray>,
    current_pos: &mut CoordsArray,
) {
    match direction {
        NORTH => current_pos.1 += 1,
        SOUTH => current_pos.1 -= 1,
        EAST => current_pos.0 += 1,
        WEST => current_pos.0 -= 1,
        _ => {}
    }
    visited_houses_coords.insert(*current_pos);
}
