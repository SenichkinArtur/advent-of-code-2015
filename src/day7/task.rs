use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Direct,
    Not,
    Or,
    And,
    Rshift,
    Lshift,
}
#[derive(Debug)]
struct Instruction {
    source_1: String,
    source_2: Option<String>,
    command: Command,
}

pub fn day7() {
    let input: Vec<String> = get_lines("src/day7/input.txt");
    let (part1, part2) = get_signal(&input);

    println!("Day 7 part 1 answer is {}", part1);
    println!("Day 7 part 2 answer is {}", part2);
}

fn get_signal(input: &[String]) -> (u16, u16) {
    let instructions: HashMap<String, Instruction> = parse_instructions(input);
    let mut wire_memo: HashMap<String, u16> = HashMap::new();

    let wire_a_v1 = eval_wire("a", &instructions, &mut wire_memo);

    wire_memo = HashMap::new();
    wire_memo.insert("b".to_string(), wire_a_v1);
    let wire_a_v2 = eval_wire("a", &instructions, &mut wire_memo);

    (wire_a_v1, wire_a_v2)
}

fn parse_instructions(input: &[String]) -> HashMap<String, Instruction> {
    let mut instructions: HashMap<String, Instruction> = HashMap::new();
    for line in input {
        let token: Vec<&str> = line.split_whitespace().collect();

        match token.as_slice() {
            ["NOT", source_1, "->", target] => {
                instructions.insert(
                    target.to_string(),
                    Instruction {
                        source_1: source_1.to_string(),
                        source_2: None,
                        command: Command::Not,
                    },
                );
            }
            [source_1, "OR", source_2, "->", target] => {
                instructions.insert(
                    target.to_string(),
                    Instruction {
                        source_1: source_1.to_string(),
                        source_2: Some(source_2.to_string()),
                        command: Command::Or,
                    },
                );
            }
            [source_1, "->", target] => {
                instructions.insert(
                    target.to_string(),
                    Instruction {
                        source_1: source_1.to_string(),
                        source_2: None,
                        command: Command::Direct,
                    },
                );
            }
            [source_1, "AND", source_2, "->", target] => {
                instructions.insert(
                    target.to_string(),
                    Instruction {
                        source_1: source_1.to_string(),
                        source_2: Some(source_2.to_string()),
                        command: Command::And,
                    },
                );
            }
            [source_1, "RSHIFT", source_2, "->", target] => {
                instructions.insert(
                    target.to_string(),
                    Instruction {
                        source_1: source_1.to_string(),
                        source_2: Some(source_2.to_string()),
                        command: Command::Rshift,
                    },
                );
            }
            [source_1, "LSHIFT", source_2, "->", target] => {
                instructions.insert(
                    target.to_string(),
                    Instruction {
                        source_1: source_1.to_string(),
                        source_2: Some(source_2.to_string()),
                        command: Command::Lshift,
                    },
                );
            }
            _ => {
                println!("Unexpected line: {}", line);
            }
        }
    }

    instructions
}

fn eval_wire(
    wire: &str,
    instructions: &HashMap<String, Instruction>,
    wire_memo: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(w) = wire_memo.get(wire) {
        return *w;
    }

    let target_instruction = instructions
        .get(wire)
        .expect("Missing instructions for wire");
    let source_1_parsed: u16 = match target_instruction.source_1.parse::<u16>() {
        Ok(number) => number,
        Err(_e) => eval_wire(&target_instruction.source_1, instructions, wire_memo),
    };
    let source_2_parsed = if let Some(s) = &target_instruction.source_2 {
        match s.parse::<u16>() {
            Ok(number) => number,
            Err(_e) => eval_wire(&s, instructions, wire_memo),
        }
    } else {
        0
    };

    let wire_value: u16 = match target_instruction.command {
        Command::Direct => source_1_parsed,
        Command::Not => !source_1_parsed,
        Command::And => source_1_parsed & source_2_parsed,
        Command::Or => source_1_parsed | source_2_parsed,
        Command::Lshift => source_1_parsed << source_2_parsed,
        Command::Rshift => source_1_parsed >> source_2_parsed,
    };
    wire_memo.insert(wire.to_string(), wire_value);

    wire_value
}

fn get_lines(path: &str) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
