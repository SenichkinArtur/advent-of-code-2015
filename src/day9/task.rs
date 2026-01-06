use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct CityDistance {
    city_1: String,
    city_2: String,
    distance: i32,
}
#[derive(Debug)]
enum Mode {
    Min,
    Max,
}

pub fn day9() {
    let input = get_lines("src/day9/input.txt");
    let part1 = get_disatance(&input, Mode::Min);
    let part2 = get_disatance(&input, Mode::Max);

    println!("Day 9 part 1 answer is {}", part1);
    println!("Day 9 part 2 answer is {}", part2);
}

fn get_disatance(lines: &Vec<String>, mode: Mode) -> i32 {
    let mut distances: Vec<CityDistance> = vec![];
    let mut cities: HashSet<&str> = HashSet::new();
    let mut dist_table: Vec<CityDistance> = vec![];

    for line in lines {
        let [city_1, "to", city_2, "=", distance] =
            line.split_whitespace().collect::<Vec<&str>>()[..]
        else {
            panic!("error while parsing lines from input.txt");
        };
        let dist = match distance.parse::<i32>() {
            Ok(number) => number,
            Err(_e) => return 0,
        };
        cities.insert(city_1);
        cities.insert(city_2);
        distances.push(CityDistance {
            city_1: city_1.to_string(),
            city_2: city_2.to_string(),
            distance: dist,
        });
    }

    for c in cities.iter() {
        let disttances_city_1: Vec<&CityDistance> =
            distances.iter().filter(|x| x.city_1 == *c).collect();
        let disttances_city_2: Vec<&CityDistance> =
            distances.iter().filter(|x| x.city_2 == *c).collect();

        for d in disttances_city_1 {
            dist_table.push(CityDistance {
                city_1: d.city_1.to_string(),
                city_2: d.city_2.to_string(),
                distance: d.distance,
            })
        }
        for d in disttances_city_2 {
            dist_table.push(CityDistance {
                city_1: d.city_2.to_string(),
                city_2: d.city_1.to_string(),
                distance: d.distance,
            })
        }
    }

    // dist_table.sort_by_key(|d| d.city_1.clone());

    let mut visited: HashSet<&str> = HashSet::new();
    let mut path: Vec<String> = Vec::new();

    // println!("mode: {:?}", mode);
    // let start_city = match mode {
    //     Mode::Min => match &distances.iter().min_by_key(|d| d.distance) {
    //         Some(d) => &d.city_1,
    //         None => {
    //             panic!("start city not found");
    //         }
    //     },
    //     Mode::Max => match &distances.iter().max_by_key(|d| d.distance) {
    //         Some(d) => &d.city_1,
    //         None => {
    //             panic!("start city not found");
    //         }
    //     },
    // };
    // println!("start_city: {}", start_city);
    // let start_city =;

    let mut distance = 0;

    let path = get_path("London", &cities, &dist_table, &mut visited, &mut path, mode);

    let mut city_distance: i32 = 0;

    for w in path.windows(2) {
        let dist = &dist_table
            .iter()
            .find(|d| d.city_1 == w[0] && d.city_2 == w[1]);
        match dist {
            Some(numb) => city_distance += numb.distance,
            None => city_distance += 0,
        }
    }

    distance
}

fn get_path<'a>(
    city: &'a str,
    cities: &HashSet<&str>,
    dist_table: &'a Vec<CityDistance>,
    visited: &'a mut HashSet<&'a str>,
    path: &mut Vec<String>,
    mode: Mode,
) -> Vec<String> {
    if visited.len() == cities.len() {
        return path.clone();
    }

    visited.insert(city);

    if path.is_empty() || path.last().unwrap() != city {
        path.push(city.to_string());
    }

    let next = match mode {
        Mode::Min => dist_table
            .iter()
            .filter(|x| x.city_1 == city && !visited.contains(x.city_2.as_str()))
            .min_by_key(|d| d.distance),
        Mode::Max => dist_table
            .iter()
            .filter(|x| x.city_1 == city && !visited.contains(x.city_2.as_str()))
            .max_by_key(|d| d.distance),
    };
    println!("next: {:?}", next);
    // let next = dist_table
    //     .iter()
    //     .filter(|x| x.city_1 == city && !visited.contains(x.city_2.as_str()))
    //     .min_by_key(|d| d.distance);

    match next {
        Some(edge) => get_path(
            edge.city_2.as_str(),
            cities,
            dist_table,
            visited,
            path,
            mode,
        ),
        None => path.clone(),
    }
}

fn get_lines(path: &str) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
