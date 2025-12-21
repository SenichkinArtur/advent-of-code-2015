use std::fs::read_to_string;

const ROWS: usize = 1000;
const COLS: usize = 1000;
const INSTRUCTION_TURN: &str = "turn";
const INSTRUCTION_TURN_ON: &str = "on";
const INSTRUCTION_TURN_OFF: &str = "off";
const INSTRUCTION_TOGGLE: &str = "toggle";

#[derive(Debug)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
    Default,
}

#[derive(Debug)]
struct Instruction {
    command: Command,
    start: [usize; 2],
    end: [usize; 2],
}

pub fn day6() {
    let input: Vec<String> = get_lines("src/day6/input.txt");
    let part1 = get_the_lights(&input);

    println!("Day 5 part 1 answer is {:?}", part1);
}

fn get_the_lights(lines: &[String]) {
    let _lights = [[false; COLS]; ROWS];
    let tokens = get_instructions(lines);

    println!("tokens: {:?}", tokens);
}

fn get_instructions(lines: &[String]) {
    let mut instructions: Vec<Instruction> = vec![];
    for line in lines {
        let token: Vec<&str> = line.split_whitespace().collect();
        let mut command = Command::Default;
        let mut start: [usize; 2] = [0, 0];
        let mut end: [usize; 2] = [0, 0];

        if token[0] == INSTRUCTION_TURN {
            if token[1] == INSTRUCTION_TURN_ON {
                command = Command::TurnOn;
            }
            if token[1] == INSTRUCTION_TURN_OFF {
                command = Command::TurnOff;
            }
            start = [
                // TODO: move this chain so a separate function
                token[2].split(",").collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap(),
                token[2].split(",").collect::<Vec<&str>>()[1]
                    .parse()
                    .unwrap(),
            ];
            end = [
                token[4].split(",").collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap(),
                token[4].split(",").collect::<Vec<&str>>()[1]
                    .parse()
                    .unwrap(),
            ];
        }
        if token[0] == INSTRUCTION_TOGGLE {
            command = Command::Toggle;
            start = [
                token[1].split(",").collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap(),
                token[1].split(",").collect::<Vec<&str>>()[1]
                    .parse()
                    .unwrap(),
            ];
            end = [
                token[3].split(",").collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap(),
                token[3].split(",").collect::<Vec<&str>>()[1]
                    .parse()
                    .unwrap(),
            ];
        }

        instructions.push(Instruction {
            command,
            start,
            end,
        });
    }

    for i in instructions {
        println!("{:?}", i);
    }
    // println!("instructions: {:?}", instructions);
}

fn get_lines(path: &str) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
