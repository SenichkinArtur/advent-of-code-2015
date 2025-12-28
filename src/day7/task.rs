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
impl Command {
    pub fn as_str(&self) -> &str {
        match self {
            Command::Not => "NOT",
            Command::Or => "OR",
            Command::And => "AND",
            Command::Rshift => "RSHIFT",
            Command::Lshift => "LSHIFT",
            Command::Direct => "",
        }
    }
}
#[derive(Debug)]
struct Instruction {
    target: String,
    source_1: String,
    source_2: Option<String>,
    command: Command,
}

pub fn day7() {
    let input: Vec<String> = get_lines("src/day7/input.txt");
    let _part1 = get_signal(&input);

    // println!("Day 8 part 1 answer is {}", part1);
}

fn get_signal(input: &[String]) -> i32 {
    let instructions: HashMap<String, Instruction> = parse_instructions(input);
    let mut wire_memo: HashMap<String, u16> = HashMap::new();

    eval_wire("a".to_string(), &instructions, &mut wire_memo);

    println!("wire_memo: {:?}", wire_memo);

    // for (wire, instruction) in instructions {
    //     eval_wire(wire.clone(), instruction, &mut wire_memo);
    //     // println!("{:?}", inst);
    // }

    1
}

fn parse_instructions(input: &[String]) -> HashMap<String, Instruction> {
    // let mut instructions: Vec<Instruction> = vec![];
    let mut instructions: HashMap<String, Instruction> = HashMap::new();
    for line in input {
        let token: Vec<&str> = line.split_whitespace().collect();

        match token.as_slice() {
            ["NOT", source_1, "->", target] => {
                instructions.insert(
                    target.to_string(),
                    Instruction {
                        target: target.to_string(),
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
                        target: target.to_string(),
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
                        target: target.to_string(),
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
                        target: target.to_string(),
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
                        target: target.to_string(),
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
                        target: target.to_string(),
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
    wire: String,
    instructions: &HashMap<String, Instruction>,
    wire_memo: &mut HashMap<String, u16>,
) -> u16 {
    println!("wire: {}", wire);
    let mut val: u16 = 0;
    // check cache and return if found
    if let Some(wire) = wire_memo.get(&wire) {
        println!("cache branch {}", *wire);
        return *wire;
    }

    match wire.parse::<u16>() {
        Ok(number) => {
            println!("Wire.parse success: {}", number);
            // wire_memo.insert(wire.to_string(), number);
            return number;
        }
        Err(_e) => {
            println!("Wire.parse error");
        }
    };

    let target_instruction = instructions.get(&wire).unwrap();
    let source_1_parsed: u16 = match target_instruction.source_1.parse::<u16>() {
        Ok(number) => {
            println!("source_1_parsed {}", number);
            // wire_memo.insert(target_instruction.source_1.to_string(), number);
            number
        }
        Err(_e) => {
            println!("source_1_parsed Err");
            eval_wire(target_instruction.source_1.clone(), instructions, wire_memo)
        }
    };
    let mut source_2_parsed: u16 = 0;
    println!(
        "target_instruction.source_2: {:?}",
        target_instruction.source_2
    );
    if let Some(j) = &target_instruction.source_2 {
        println!("if let Some(j): {:?}", j);
        let source_2: u16 = match j.parse::<u16>() {
            Ok(number) => {
                // source_2_parsed = number;
                println!("source_2_parsed {}", number);
                // wire_memo.insert(j.to_string(), number);
                number
            }
            Err(_e) => {
                println!("source_2_parsed Err");
                eval_wire(j.clone(), instructions, wire_memo)
            }
        };
        source_2_parsed = source_2;
    }

    match target_instruction.command {
        Command::Direct => val = source_1_parsed,
        Command::Not => val = !source_1_parsed,
        Command::And => val = source_1_parsed & source_2_parsed,
        Command::Or => val = source_1_parsed | source_2_parsed,
        Command::Lshift => val = source_1_parsed << source_2_parsed,
        Command::Rshift => val = source_1_parsed >> source_2_parsed,
    };

    wire_memo.insert(wire.clone(), val);
    println!("after match command wire: {:?}, value: {}", wire.clone(), val);

    return val;

    // target_value
}

fn get_lines(path: &str) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
