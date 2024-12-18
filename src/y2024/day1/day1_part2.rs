use std::{collections::HashMap, fs};

pub fn similarity_score() -> u32 {
    let mut first_part = Vec::new();
    let mut second_part = Vec::new();
    let binding = fs::read_to_string("src/y2024/day1/day1.txt").expect("Failed to read file");

    for line in binding.lines() {
        let stuff = line.split_whitespace().collect::<Vec<_>>();

        match stuff.get(0) {
            Some(first) => first_part.push(first.parse::<u32>().unwrap_or(0)),
            None => println!("No value found"),
        }

        match stuff.get(1) {
            Some(second) => second_part.push(second.parse::<u32>().unwrap_or(0)),
            None => println!("No value found"),
        }
    }

    // let mut result = Vec::new();
    // // too expensive
    // for idx in 0..first_part.iter().count() {
    //     let first_part_val = match first_part.get(idx) {
    //       Some(val) => val,
    //       None => &0,
    //     };
    //     if second_part.contains(first_part_val) {
    //         result.push(second_part.iter().filter(|x| *x == first_part_val).count() as u32 * first_part_val);
    //     }
    // }
    // result.iter().sum()

     // Build a frequency map for `second_part` to speed up lookups
     let mut second_part_map = HashMap::new();
     for &value in &second_part {
         second_part_map.entry(value).and_modify(|c| { *c += 1 }).or_insert(1);
     }
 
     // Calculate the similarity score
     let result: u32 = first_part
         .iter()
         .filter_map(|&value| {
             second_part_map.get(&value).map(|&count| count as u32 * value)
         })
         .sum();
 
     println!("{}", result);
     result
}
