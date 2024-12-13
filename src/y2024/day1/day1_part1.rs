use std::fs;

pub fn total_distance_btw_list() -> u32 {
    let mut first_part = Vec::new();
    let mut second_part = Vec::new();
    let binding = fs::read_to_string("src/y2024/day1/day1.txt").expect("Failed to read file");

    for line in binding.lines() {
        // split in on space
        let stuff = line.split_whitespace().collect::<Vec<_>>();

        // if let Some(first) = stuff.get(0) {
        //     first_part.push(*first);
        // }
        match stuff.get(0) {
            Some(first) => first_part.push(first.parse::<u32>().unwrap_or(0)),
            None => println!("No value found"),
        }

        // if let Some(second) = stuff.get(1..) {
        //     second_part.push(second.join(" ")); // Join slices into a single string
        // }

        match stuff.get(1) {
            Some(second) => second_part.push(second.parse::<u32>().unwrap_or(0)),
            None => println!("No value found"),
        }
    }

    // get the min and subtract
    let mut result = Vec::new();
    for _ in 0..second_part.iter().count() {
        let first_part_min = match first_part.iter().min() {
            Some(fpm) => *fpm,
            None => 0 as u32,
        };
        let second_part_min = match second_part.iter().min() {
            Some(spm) => *spm,
            None => 0 as u32,
        };

        // get the index of the min
        if let Some(index) = first_part.iter().position(|x| *x == first_part_min) {
            first_part.remove(index);
        }
        if let Some(index) = second_part.iter().position(|x| *x == second_part_min) {
            second_part.remove(index);
        }
        // println!("left part: {:?} and right part: {:?} and count is {} --- {}", first_part_min, second_part_min, first_part.iter().count(), second_part.iter().count());
        let diff = match first_part_min.ge(&second_part_min) {
            true => first_part_min - second_part_min,
            false => second_part_min - first_part_min,
        };
        result.push(diff);
    }

    result.iter().sum()
}
