use petgraph::dot::Dot;
use petgraph::graphmap::UnGraphMap;
use std::fs::read_to_string;

pub fn day9() {
    let input = get_lines("src/day9/input.txt");
    let part1 = get_shortest_disatance(&input);

    println!("Day 9 part 1 answer is {}", part1);
}

fn get_shortest_disatance(lines: &Vec<String>) -> i32 {
    let mut g = UnGraphMap::<&str, i32>::new();
    for line in lines {
        let [start_city, "to", end_city, "=", distance] =
            line.split_whitespace().collect::<Vec<&str>>()[..]
        else {
            panic!("error while parsing lines from input.txt");
        };
        let a = g.add_node(start_city);
        let b = g.add_node(end_city);
        let dist = match distance.parse::<i32>() {
            Ok(number) => number,
            Err(_e) => return 0,
        };

        g.add_edge(a, b, dist);
    }

    let basic_dot = Dot::new(&g);

    println!("graph: {:?}", basic_dot);

    1
}

fn get_lines(path: &str) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
