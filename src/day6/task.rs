use std::fs::read_to_string;

const ROWS: usize = 1000;
const COLS: usize = 1000;
const INSTRUCTION_TURN: &str = "turn";
const INSTRUCTION_TURN_ON: &str = "on";
const INSTRUCTION_TURN_OFF: &str = "off";
const INSTRUCTION_TOGGLE: &str = "toggle";

#[derive(PartialEq)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    command: Command,
    start: [usize; 2],
    end: [usize; 2],
}

pub fn day6() {
    let input: Vec<String> = get_lines("src/day6/input.txt");

    let part1 = get_the_lights(&input);
    let part2 = get_the_lights_brightness(&input);

    println!("Day 6 part 1 answer is {:?}", part1);
    println!("Day 6 part 2 answer is {:?}", part2);
}

fn get_the_lights(lines: &[String]) -> usize {
    let mut lights: Box<[[u8; 1000]; 1000]> = Box::new([[0; COLS]; ROWS]);
    let instructions = get_instructions(lines);

    for instruction in instructions {
        for y in instruction.start[0]..=instruction.end[0] {
            for x in instruction.start[1]..=instruction.end[1] {
                match instruction.command {
                    Command::TurnOn => lights[y][x] = 1,
                    Command::TurnOff => lights[y][x] = 0,
                    Command::Toggle => lights[y][x] ^= 1,
                }
            }
        }
    }

    let count: usize = get_size(&lights);

    count
}

fn get_the_lights_brightness(lines: &[String]) -> usize {
    let mut lights: Box<[[u8; 1000]; 1000]> = Box::new([[0; COLS]; ROWS]);
    let instructions = get_instructions(lines);

    for instruction in instructions {
        for y in instruction.start[0]..=instruction.end[0] {
            for x in instruction.start[1]..=instruction.end[1] {
                match instruction.command {
                    Command::TurnOn => lights[y][x] += 1,
                    Command::TurnOff => lights[y][x] = lights[y][x].saturating_sub(1),
                    Command::Toggle => lights[y][x] += 2,
                }
            }
        }
    }

    let count: usize = get_size(&lights);

    count
}

fn get_instructions(lines: &[String]) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for line in lines {
        let token: Vec<&str> = line.split_whitespace().collect();
        let mut command: Option<Command> = None;
        let mut start: [usize; 2] = [0, 0];
        let mut end: [usize; 2] = [0, 0];

        if token[0] == INSTRUCTION_TURN {
            if token[1] == INSTRUCTION_TURN_ON {
                command = Some(Command::TurnOn);
            }
            if token[1] == INSTRUCTION_TURN_OFF {
                command = Some(Command::TurnOff);
            }
            let (start_x, start_y) = parse_coords(token[2]);
            let (end_x, end_y) = parse_coords(token[4]);
            start = [start_y, start_x];
            end = [end_y, end_x];
        }
        if token[0] == INSTRUCTION_TOGGLE {
            command = Some(Command::Toggle);
            let (start_x, start_y) = parse_coords(token[1]);
            let (end_x, end_y) = parse_coords(token[3]);
            start = [start_y, start_x];
            end = [end_y, end_x];
        }

        instructions.push(Instruction {
            command: command.unwrap(),
            start,
            end,
        });
    }

    instructions
}

fn get_size(lights: &[[u8; 1000]; 1000]) -> usize {
    return lights
        .iter()
        .flatten()
        .copied()
        .map(usize::from)
        .sum::<usize>();
}

fn parse_coords(token: &str) -> (usize, usize) {
    let (left, right) = token.split_once(",").unwrap();
    (
        left.parse::<usize>().unwrap(),
        right.parse::<usize>().unwrap(),
    )
}

fn get_lines(path: &str) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
