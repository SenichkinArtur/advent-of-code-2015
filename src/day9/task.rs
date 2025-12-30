use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct CityDistance {
    city_1: String,
    city_2: String,
    distance: i32,
}

pub fn day9() {
    let input = get_lines("src/day9/input.txt");
    let part1 = get_shortest_disatance(&input);

    println!("Day 9 part 1 answer is {}", part1);
}

fn get_shortest_disatance(lines: &Vec<String>) -> i32 {
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
        let qwe: Vec<&CityDistance> = distances.iter().filter(|x| x.city_1 == *c).collect();
        let asd: Vec<&CityDistance> = distances.iter().filter(|x| x.city_2 == *c).collect();

        for q in qwe {
            dist_table.push(CityDistance {
                city_1: q.city_1.to_string(),
                city_2: q.city_2.to_string(),
                distance: q.distance,
            })
        }
        for a in asd {
            dist_table.push(CityDistance {
                city_1: a.city_2.to_string(),
                city_2: a.city_1.to_string(),
                distance: a.distance,
            })
        }
    }

    dist_table.sort_by_key(|d| d.city_1.clone());

    let mut visited: HashSet<&str> = HashSet::new();
    let mut path: Vec<String> = Vec::new();
    let a = get_min_route("Faerun", &cities, &dist_table, &mut visited, &mut path);

    // for c in &cities {
    //     let a = get_min_route(*c, &cities, &dist_table);
    //     println!("a: {:?}", a);
    // }

    1
}

fn get_min_route(
    city: &str,
    cities: &HashSet<&str>,
    dist_table: &Vec<CityDistance>,
    visited: &mut HashSet<&str>,
    path: &mut Vec<String>,
) -> Vec<String> {
    if visited.len() == cities.len() {
        return path.clone();
    }

    visited.insert(city);

    if path.is_empty() || path.last().unwrap() != city {
        path.push(city.to_string());
    }

    let next = dist_table
        .iter()
        .filter(|x| x.city_1 == city && !visited.contains(x.city_2.as_str()))
        .min_by_key(|d| d.distance);

    match next {
        Some(edge) => get_min_route(edge.city_2.as_str(), cities, dist_table, visited, path),
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
