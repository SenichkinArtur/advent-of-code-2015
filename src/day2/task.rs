use std::fs::read_to_string;

pub fn day2() {
    calculate_total_square_feet();
    calculate_ribon_length();
}

fn calculate_total_square_feet() {
    let boxes_dimensions: Vec<String> = read_lines("src/day2/input.txt");
    let mut wrapping_paper_size: i32 = 0;

    for box_dimensions in boxes_dimensions {
        let box_dimensions_vec: Vec<&str> = box_dimensions.split('x').collect();
        let mut box_dimensions_vec_sorted: Vec<i32> = box_dimensions
            .split('x')
            .map(|s: &str| s.parse::<i32>().unwrap())
            .collect();
        box_dimensions_vec_sorted.sort_unstable();
        let smallest_side: i32 = box_dimensions_vec_sorted[0] * box_dimensions_vec_sorted[1];
        let length: i32 = box_dimensions_vec[0].parse::<i32>().unwrap();
        let width: i32 = box_dimensions_vec[1].parse::<i32>().unwrap();
        let height: i32 = box_dimensions_vec[2].parse::<i32>().unwrap();
        // 2*l*w + 2*w*h + 2*h*l
        let present_size: i32 = (2 * length * width) + (2 * width * height) + (2 * height * length);
        wrapping_paper_size = wrapping_paper_size + present_size + smallest_side;
    }

    println!("Day 2 part 1 answer is {}", wrapping_paper_size);
}

fn calculate_ribon_length() {
    let boxes_dimensions: Vec<String> = read_lines("src/day2/input.txt");
    let mut ribon_length: i32 = 0;

    for box_dimensions in boxes_dimensions {
        let mut box_dimensions_vec_sorted: Vec<i32> = box_dimensions
            .split('x')
            .map(|s: &str| s.parse::<i32>().unwrap())
            .collect();
        box_dimensions_vec_sorted.sort_unstable();

        let wrapper_ribon: i32 = box_dimensions_vec_sorted[0]
            + box_dimensions_vec_sorted[0]
            + box_dimensions_vec_sorted[1]
            + box_dimensions_vec_sorted[1];
        let bow_ribon: i32 = box_dimensions_vec_sorted[0]
            * box_dimensions_vec_sorted[1]
            * box_dimensions_vec_sorted[2];

        ribon_length = ribon_length + wrapper_ribon + bow_ribon;
    }

    println!("Day 2 part 2 answer is {}", ribon_length);
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
